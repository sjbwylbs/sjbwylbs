
# 关于通道在多线程以及多协程下的性能对比测试
REF:!原文[https://rustcc.cn/article?id=b9480b40-a765-4c23-978e-ba78bbe1b8dd]

上次提到了tokio::sync::Mutex的性能不行，tokio官方文档推荐的解决方案：一、如果不存在长时间异步任务的可以用标准库互斥锁std::sync::Mutex，tokio官方文档的还推荐了一个第三方互斥锁parking_lot，但我测了无论是互斥锁还是读写锁性能都不如标准库的锁，所以就不推荐了；二、可以用tokio::sync::mpsc::unbounded_channel等异步通道来代替锁，甚至连教程里mini-redis都是基于通道的思路实现的，于是我就想测了测通道的性能。

测试思路就是最简单的发号器，以发号器作为生产者producer，然后分出多个协程用作消费，一共发100万个号，消费子协程通过向生产者发出信号（子协程以及消费者对应的序号），然后通过调度器通道dispatcher同步等待返回值，并将返回值添加到作为消费者的数组consumers，大体就是这么个思路。

同时作为对比，测试了单线程下百万序号分配给N个数组的时间，原子计数器在多线程的情况下的发号时间，互斥锁在多线程下的分配时间，std::sync::mpsc::channel标准库通道的百万次发号时间，以及最流行的第三方无锁通道库crossbeam_channel的性能。

以及在tokio的runtime下，异步互斥锁tokio::sync::Mutex的发号时间，异步无限制通道tokio::sync::mpsc::unbounded_channel的发号时间，以及第三方库async_channel以及postage的异步发号时间。其中async_channel是crossbeam_channel在github的issue里推荐异步方案，而且两者都是支持接收者Receiver的克隆的，std和tokio的通道都不支持接收者克隆。

不管是同步通道还是异步通道，可用的第三方库都比较少，同步库还有功能性的延迟队列delay_queue以及阻塞队列blocking_queue不过基本都是基于标准库的同步锁实现的，最后性能和标准库的std::sync::mpsc::channel基本一致，而其他异步通道库大多跑不起来，就没选进来。

下面主要展示下基于tokio官方的通道发号器的实现思路。
```rust
fn tokio_unbounded_channel() {
    print!("\ntokio-unbounded-channel: ");
    
    // 循环N_LOOP次测试
    for _ in 1..=N_LOOP {
        // 初始化生产者通道，用的无限容量的tokio异步通道
        let (producer_sender, mut producer_receiver) = tokio::sync::mpsc::unbounded_channel();
        // 初始化调度器，为了让协程开启后再生产调度通道，所以先用None初始化
        let dispatcher = tokio::sync::Mutex::new(vec![None; N_THREADS]);
        // 消费者数组，用异步互斥锁先初始化，否则无法轻易分享到协程，等进入协程后再分别解锁，并不影响通道的性能测试
        let consumers = (0..N_THREADS).map(|_| tokio::sync::Mutex::new(Vec::new())).collect::<Vec<_>>();
        // 异步的barrier/wait阻塞锁，为的是等各个线程的调度通道都初始化完成后，生产者再开始接受数据，不影响通道性能测试
        let barrier = tokio::sync::Barrier::new(N_THREADS + 1);
        // 计时开始
        let start = Instant::now();    
        // 用了一个第三方库作用域库，用法和标准库的thead::scope几乎一致，可以省去跨线程引用计数Arc，也可以不用join
        tokio_scoped::scope(|s| {
            for i in 0..N_THREADS {
                // 循环里将各个公共变量通过引用的形式传进协程作用域
                let producer_sender = producer_sender.clone();
                let dispatcher = &dispatcher;
                let consumers = &consumers;
                let barrier = &barrier;
                s.spawn(async move {
                    // 初始化调度器通道
                    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();
                    // 在子协程里初始化可以避免接收者的跨作用域复制，而发送者一般都支持克隆，所以把发送者克隆出去
                    dispatcher.lock().await[i] = Some(tx);
                    // 等待所有发送者初始化完成
                    barrier.wait().await;                    
                    // 把消费者数组从锁中取出来，避免影响push的性能
                    let mut consumer = consumers[i].lock().await;
                    for _ in 0..N_TIMES {
                        // 向生产者发送该协程所在的序号
                        producer_sender.send(i).unwrap();
                        // 同步等待并接收通过调度器dispatcher通道返回来的序号
                        let j = rx.recv().await.unwrap();
                        // 将序号推入消费者数组
                        consumer.push(j);
                    }
                });
            }
            // 将公共变量以引用的形式传入生产者协程的作用域
            let dispatcher = &dispatcher;
            let barrier = &barrier;
            s.spawn(async move {
                // 等待调度通道初始化完成
                barrier.wait().await;    
                // 把调度通道从锁中取出来，以保证使用的时候不影响通道发送效率 
                let dispatcher = dispatcher.lock().await.iter().map(|some_tx| some_tx.clone().unwrap()).collect::<Vec<_>>();
                // 进行百万次发号循环
                for j in 0..N_TIMES * N_THREADS {
                    // 接收从各个协程发来的序号
                    let i = producer_receiver.recv().await.unwrap();
                    // 向序号对应的调度器通道顺序发送号码
                    dispatcher[i].send(j).unwrap();
                }
            });
        });
        // 打印总耗时
        print!("{:?}, ", start.elapsed());
    }
}
```
    因为官方文档说锁性能不行，但为了严谨，也测试了官方的说法，这里生产者就是用异步互斥锁包住的一个usize数字，
```rust
fn tokio_mutex() {
    print!("\ntokio-mutex: ");
    
    for _ in 1..=N_LOOP {
        let consumers = (0..N_THREADS).map(|_| tokio::sync::Mutex::new(Vec::new())).collect::<Vec<_>>();
        // 初始化锁以及序号
        let producer = tokio::sync::Mutex::new(0usize);
        let start = Instant::now();
        tokio_scoped::scope(|s| {
            for i in 0..N_THREADS {
                let producer = &producer;
                let consumers = &consumers;
                s.spawn(async move {
                    let mut consumer = consumers[i].lock().await;
                    for _ in 0..N_TIMES {
                        // 直接解锁获取排号
                        let mut j = producer.lock().await;
                        // 将序号推入消费者数组
                        consumer.push(*j);
                        // 让生产者序号自增
                        *j += 1;
                    }
                });
            }
        });
        print!("{:?}, ", start.elapsed());
    }
}
```
如果是个5个线程5个循环，最终可能会得到这样的发号结果，即每个数组都获得了5个号，所有数组获得的号码独立无重复：
```bash
43.4µs, consumers: [Mutex { data: [0, 1, 2, 3, 4] }, Mutex { data: [5, 9, 13, 17, 21] }, Mutex { data: [8, 12, 16, 20, 24] }, Mutex { data: [6, 10, 14, 18, 22] }, Mutex { data: [7, 11, 15, 19, 23] }]
40.5µs, consumers: [Mutex { data: [0, 1, 2, 3, 4] }, Mutex { data: [5, 6, 7, 8, 9] }, Mutex { data: [10, 13, 16, 19, 22] }, Mutex { data: [11, 14, 17, 20, 23] }, Mutex { data: [12, 15, 18, 21, 24] }]     
33.6µs, consumers: [Mutex { data: [0, 1, 2, 3, 4] }, Mutex { data: [16, 18, 20, 22, 24] }, Mutex { data: [5, 6, 7, 8, 9] }, Mutex { data: [10, 11, 12, 13, 14] }, Mutex { data: [15, 17, 19, 21, 23] }]     
32.9µs, consumers: [Mutex { data: [0, 1, 2, 3, 4] }, Mutex { data: [10, 11, 12, 14, 17] }, Mutex { data: [5, 6, 7, 8, 9] }, Mutex { data: [13, 16, 19, 21, 23] }, Mutex { data: [15, 18, 20, 22, 24] }]     
34.7µs, consumers: [Mutex { data: [0, 1, 2, 3, 4] }, Mutex { data: [5, 6, 7, 8, 9] }, Mutex { data: [10, 11, 12, 13, 14] }, Mutex { data: [20, 21, 22, 23, 24] }, Mutex { data: [15, 16, 17, 18, 19] }]     
27.2µs, consumers: [Mutex { data: [0, 1, 2, 3, 4] }, Mutex { data: [5, 6, 7, 8, 9] }, Mutex { data: [10, 11, 12, 13, 14] }, Mutex { data: [15, 16, 17, 18, 19] }, Mutex { data: [20, 21, 22, 23, 24] }]     
33.2µs, consumers: [Mutex { data: [0, 1, 2, 3, 4] }, Mutex { data: [5, 6, 7, 8, 9] }, Mutex { data: [20, 21, 22, 23, 24] }, Mutex { data: [10, 11, 12, 13, 14] }, Mutex { data: [15, 16, 17, 18, 19] }]     
29.1µs, consumers: [Mutex { data: [0, 1, 2, 3, 4] }, Mutex { data: [5, 6, 7, 8, 9] }, Mutex { data: [10, 11, 12, 13, 14] }, Mutex { data: [15, 16, 17, 18, 19] }, Mutex { data: [20, 21, 22, 23, 24] }]     
31.3µs, consumers: [Mutex { data: [0, 1, 2, 3, 4] }, Mutex { data: [5, 6, 7, 8, 9] }, Mutex { data: [20, 21, 22, 23, 24] }, Mutex { data: [10, 11, 12, 13, 14] }, Mutex { data: [15, 16, 17, 18, 19] }]     
35µs, consumers: [Mutex { data: [0, 1, 2, 3, 4] }, Mutex { data: [10, 13, 16, 19, 22] }, Mutex { data: [5, 6, 7, 8, 9] }, Mutex { data: [11, 14, 17, 20, 23] }, Mutex { data: [12, 15, 18, 21, 24] }]
```
当然，5*5的发号器是无法测出性能，于是我们可以测出分10个线程/协程百万次发号性能，以及1000个线程/协程的发号性能。

测试环境是win10，2.6Ghz，12线程，不同环境下结果不一定相同，仅供参考，最终结果如下：
```bash
threads: 10, times: 100000

single-thread: 3.783ms, 2.3065ms, 2.362ms, 3.0346ms, 3.5482ms, 3.5821ms, 2.2885ms, 2.3595ms, 2.7192ms, 3.4395ms, 
threads-atomic: 21.9533ms, 22.5012ms, 22.1012ms, 21.1667ms, 21.5774ms, 21.6005ms, 21.1701ms, 21.2619ms, 21.2166ms, 21.4414ms, 
threads-mutex: 23.9623ms, 24.0711ms, 23.8364ms, 20.1042ms, 19.0233ms, 19.282ms, 23.0869ms, 19.7093ms, 21.37ms, 19.7068ms, 
threads-channel: 1.6353534s, 1.6252843s, 1.6425524s, 1.6559191s, 1.641655s, 1.6410002s, 1.6198816s, 1.637836s, 1.6453233s, 1.6372631s, 
threads-crossbeam-channel: 159.0092ms, 163.6825ms, 159.459ms, 159.3077ms, 158.3843ms, 159.1896ms, 159.619ms, 158.5752ms, 158.7249ms, 160.4228ms, 
tokio-mutex: 184.6276ms, 177.1989ms, 177.8124ms, 185.4614ms, 180.4103ms, 182.756ms, 178.7914ms, 185.8894ms, 185.6178ms, 183.6666ms, 
tokio-unbounded-channel: 332.9238ms, 327.6631ms, 324.7683ms, 327.3036ms, 332.2728ms, 314.0601ms, 329.8565ms, 335.5576ms, 336.3912ms, 330.2399ms, 
tokio-async-channel: 318.8884ms, 320.8109ms, 318.8901ms, 315.8831ms, 324.3891ms, 323.8225ms, 316.8371ms, 315.7305ms, 317.4599ms, 316.8412ms, 
tokio-postage: 408.6799ms, 414.7718ms, 410.2594ms, 423.4415ms, 406.069ms, 405.1291ms, 402.2925ms, 420.6135ms, 415.2888ms, 413.4756ms, 

threads: 1000, times: 1000

single-thread: 5.2886ms, 4.8897ms, 4.6462ms, 4.7382ms, 4.9838ms, 4.7364ms, 5.0485ms, 4.4898ms, 5.0647ms, 4.6328ms, 
threads-atomic: 38.8132ms, 34.1405ms, 33.8424ms, 33.9646ms, 33.8314ms, 33.7577ms, 33.7202ms, 33.7758ms, 36.6253ms, 35.2552ms, 
threads-mutex: 35.8882ms, 34.372ms, 34.2713ms, 34.4301ms, 34.1899ms, 34.3877ms, 33.8223ms, 34.5564ms, 34.5704ms, 35.3365ms, 
threads-channel: 1.72273s, 1.6365346s, 1.6199249s, 1.6276635s, 1.6322794s, 1.618129s, 1.6333964s, 1.628561s, 1.6276872s, 1.6353098s, 
threads-crossbeam-channel: 210.8052ms, 215.4796ms, 209.6983ms, 210.1558ms, 207.7503ms, 212.0342ms, 220.2059ms, 209.3559ms, 209.2762ms, 216.1148ms, 
tokio-mutex: 189.8871ms, 184.5607ms, 196.8624ms, 186.0492ms, 184.4969ms, 188.0062ms, 183.7379ms, 187.2251ms, 184.5999ms, 181.53ms, 
tokio-unbounded-channel: 310.2307ms, 296.9085ms, 298.4247ms, 301.4347ms, 303.53ms, 307.2157ms, 316.1747ms, 306.7214ms, 314.8052ms, 304.5194ms, 
tokio-async-channel: 268.8818ms, 262.5957ms, 255.4317ms, 265.0369ms, 265.0607ms, 263.4119ms, 262.3272ms, 266.9498ms, 262.4605ms, 259.726ms, 
tokio-postage: 360.0674ms, 350.0455ms, 351.7645ms, 360.3491ms, 340.112ms, 354.3098ms, 348.8364ms, 349.2154ms, 347.6289ms, 347.7171ms, 
```
可以看出，首先单线程计数推数组百万次操作大约是2-6毫秒，多线程原子计数和多线程互斥锁大约是20-40ms，且随着线程数增多而负担加重。

基于线程的阻塞通道性能不加，平均时间都在1600+毫秒这样，所以在多线程里其实是不建议用标准库的通道进行大规模调度的。

而crossbeam-channel的无锁通道则表现优异，百万次操作多线程都在220ms以内，功能上也比标准库的通道更好用。

tokio运行时这边，10个协程和1000个协程对性能的影响都不算太大。

标准库里用异步锁Mutex大约在180ms上下，用异步通道unbounded_channel大约在300-330ms上下，官方文档虽然说了异步互斥锁有性能问题，其性能比通道来说还是马马虎虎的，起码比官方推荐的通道会快上接近一倍，在一些既需要异步等待，又需要加锁的类似于事务的操作中用异步锁也完全没问题。

async_channel性能略好于标准库的tokio::sync::mpsc::unbounded_channel，特别是在多协程的状态下，功能上也更好用，值得日常使用。

postage虽然功能上与async_channel相似，但性能却不太佳，可能是其没有不限容量通道，只能通过大缓存通道来初始化导致的。

源码直接贴帖子里，因为一个环境总共就测三四组数据，所以就没有抽象出功能来（抽象太费脑筋），大体上代码结构都是相似的。
```rust
use std::thread;
use std::sync::{Barrier, Mutex};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Instant;

use tokio;
use tokio_scoped;

use crossbeam_channel;
use blockingqueue;
use delay_queue::{Delay, DelayQueue};
use postage::{self, prelude::Stream, sink::Sink};
use async_channel;


const N_THREADS: usize = 10;
const N_TIMES: usize = 100000;
const N_LOOP: usize = 10;

#[tokio::main(worker_threads = 10)]
async fn main() {
    print!("threads: {}, times: {}\n\n", N_THREADS, N_TIMES);
    
    single_thread();
    threads_atomic();
    threads_mutex();
    threads_channel();
    threads_crossbeam_channel();
    tokio_mutex();
    tokio_unbounded_channel();
    tokio_async_channel();
    tokio_postage();
}

fn single_thread() {
    print!("single-thread: ");

    for _ in 1..=N_LOOP {
        let start = Instant::now();
        let mut consumers = (0..N_THREADS).map(|_| Vec::new()).collect::<Vec<_>>();
        for i in 0..N_THREADS {
            for j in i * N_TIMES .. (i + 1) * N_TIMES {
                consumers[i].push(j);
            }
        }
        print!("{:?}, ", start.elapsed());
    }
}

fn threads_atomic() {
    print!("\nthreads-atomic: ");
    
    for _ in 1..=N_LOOP {
        let consumers = (0..N_THREADS).map(|_| Mutex::new(Vec::new())).collect::<Vec<_>>();
        let producer = AtomicUsize::new(0);
        let start = Instant::now();
        thread::scope(|s| {
            for i in 0..N_THREADS {
                let producer = &producer;
                let consumers = &consumers;
                s.spawn(move || {
                    let mut consumer = consumers[i].lock().unwrap();
                    for _ in 0..N_TIMES {
                        let j = producer.fetch_add(1, Ordering::Relaxed);
                        consumer.push(j);
                    }
                });
            }
        });
        print!("{:?}, ", start.elapsed());
    }
}

fn threads_mutex() {
    print!("\nthreads-mutex: ");
    
    for _ in 1..=N_LOOP {
        let consumers = (0..N_THREADS).map(|_| Mutex::new(Vec::new())).collect::<Vec<_>>();
        let producer = Mutex::new(0usize);
        let start = Instant::now();
        thread::scope(|s| {
            for i in 0..N_THREADS {
                let producer = &producer;
                let consumers = &consumers;
                s.spawn(move || {
                    let mut consumer = consumers[i].lock().unwrap();
                    for _ in 0..N_TIMES {
                        let mut j = producer.lock().unwrap();
                        consumer.push(*j);
                        *j += 1;
                    }
                });
            }
        });
        print!("{:?}, ", start.elapsed());
    }
}

fn threads_channel() {
    print!("\nthreads-channel: ");
    
    for _ in 1..=N_LOOP {
        let (producer_sender, producer_receiver) = std::sync::mpsc::channel();
        let dispatcher = Mutex::new(vec![None; N_THREADS]);
        let consumers = (0..N_THREADS).map(|_| Mutex::new(Vec::new())).collect::<Vec<_>>();
        let barrier = Barrier::new(N_THREADS + 1);
        let start = Instant::now();    
        thread::scope(|s| {
            for i in 0..N_THREADS {
                let producer_sender = producer_sender.clone();
                let dispatcher = &dispatcher;
                let consumers = &consumers;
                let barrier = &barrier;
                s.spawn(move || {
                    let (tx, rx) = std::sync::mpsc::channel();
                    dispatcher.lock().unwrap()[i] = Some(tx);
                    barrier.wait();                    
                    let mut consumer = consumers[i].lock().unwrap();
                    for _ in 0..N_TIMES {
                        producer_sender.send(i).unwrap();
                        let j = rx.recv().unwrap();
                        consumer.push(j);
                    }
                });
            }
            barrier.wait();            
            let dispatcher = dispatcher.lock().unwrap().iter().map(|some_tx| some_tx.clone().unwrap()).collect::<Vec<_>>();
            for j in 0..N_TIMES * N_THREADS {
                let i = producer_receiver.recv().unwrap();
                dispatcher[i].send(j).unwrap();
            }
        });
        print!("{:?}, ", start.elapsed());
    }
}

fn threads_crossbeam_channel() {
    print!("\nthreads-crossbeam-channel: ");
    
    for _ in 1..=N_LOOP {
        let (producer_sender, producer_receiver) = crossbeam_channel::unbounded();
        let dispatcher = Mutex::new(vec![None; N_THREADS]);
        let consumers = (0..N_THREADS).map(|_| Mutex::new(Vec::new())).collect::<Vec<_>>();
        let barrier = Barrier::new(N_THREADS + 1);
        let start = Instant::now();    
        thread::scope(|s| {
            for i in 0..N_THREADS {
                let producer_sender = producer_sender.clone();
                let dispatcher = &dispatcher;
                let consumers = &consumers;
                let barrier = &barrier;
                s.spawn(move || {
                    let (tx, rx) = crossbeam_channel::unbounded();
                    dispatcher.lock().unwrap()[i] = Some(tx);
                    barrier.wait();                    
                    let mut consumer = consumers[i].lock().unwrap();
                    for _ in 0..N_TIMES {
                        producer_sender.send(i).unwrap();
                        let j = rx.recv().unwrap();
                        consumer.push(j);
                    }
                });
            }
            barrier.wait();            
            let dispatcher = dispatcher.lock().unwrap().iter().map(|some_tx| some_tx.clone().unwrap()).collect::<Vec<_>>();
            for j in 0..N_TIMES * N_THREADS {
                let i = producer_receiver.recv().unwrap();
                dispatcher[i].send(j).unwrap();
            }
        });
        print!("{:?}, ", start.elapsed());
    }
}

fn tokio_mutex() {
    print!("\ntokio-mutex: ");
    
    for _ in 1..=N_LOOP {
        let consumers = (0..N_THREADS).map(|_| tokio::sync::Mutex::new(Vec::new())).collect::<Vec<_>>();
        let producer = tokio::sync::Mutex::new(0usize);
        let start = Instant::now();
        tokio_scoped::scope(|s| {
            for i in 0..N_THREADS {
                let producer = &producer;
                let consumers = &consumers;
                s.spawn(async move {
                    let mut consumer = consumers[i].lock().await;
                    for _ in 0..N_TIMES {
                        let mut j = producer.lock().await;
                        consumer.push(*j);
                        *j += 1;
                    }
                });
            }
        });
        print!("{:?}, ", start.elapsed());
        // println!("consumers: {:?}", consumers);
    }
}

fn tokio_unbounded_channel() {
    print!("\ntokio-unbounded-channel: ");
    
    for _ in 1..=N_LOOP {
        let (producer_sender, mut producer_receiver) = tokio::sync::mpsc::unbounded_channel();
        let dispatcher = tokio::sync::Mutex::new(vec![None; N_THREADS]);
        let consumers = (0..N_THREADS).map(|_| tokio::sync::Mutex::new(Vec::new())).collect::<Vec<_>>();
        let barrier = tokio::sync::Barrier::new(N_THREADS + 1);
        let start = Instant::now();    
        tokio_scoped::scope(|s| {
            for i in 0..N_THREADS {
                let producer_sender = producer_sender.clone();
                let dispatcher = &dispatcher;
                let consumers = &consumers;
                let barrier = &barrier;
                s.spawn(async move {
                    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();
                    dispatcher.lock().await[i] = Some(tx);
                    barrier.wait().await;                    
                    let mut consumer = consumers[i].lock().await;
                    for _ in 0..N_TIMES {
                        producer_sender.send(i).unwrap();
                        let j = rx.recv().await.unwrap();
                        consumer.push(j);
                    }
                });
            }
            let dispatcher = &dispatcher;
            let barrier = &barrier;
            s.spawn(async move {
                barrier.wait().await;            
                let dispatcher = dispatcher.lock().await.iter().map(|some_tx| some_tx.clone().unwrap()).collect::<Vec<_>>();
                for j in 0..N_TIMES * N_THREADS {
                    let i = producer_receiver.recv().await.unwrap();
                    dispatcher[i].send(j).unwrap();
                }
            });
        });
        print!("{:?}, ", start.elapsed());
        // println!("consumers: {:?}", consumers);
    }
}

fn tokio_async_channel() {
    print!("\ntokio-async-channel: ");
    
    for _ in 1..=N_LOOP {
        let (producer_sender, producer_receiver) = async_channel::unbounded();
        let dispatcher = tokio::sync::Mutex::new(vec![None; N_THREADS]);
        let consumers = (0..N_THREADS).map(|_| tokio::sync::Mutex::new(Vec::new())).collect::<Vec<_>>();
        let barrier = tokio::sync::Barrier::new(N_THREADS + 1);
        let start = Instant::now();    
        tokio_scoped::scope(|s| {
            for i in 0..N_THREADS {
                let producer_sender = producer_sender.clone();
                let dispatcher = &dispatcher;
                let consumers = &consumers;
                let barrier = &barrier;
                s.spawn(async move {
                    let (tx, rx) = async_channel::unbounded();
                    dispatcher.lock().await[i] = Some(tx);
                    barrier.wait().await;                    
                    let mut consumer = consumers[i].lock().await;
                    for _ in 0..N_TIMES {
                        producer_sender.send(i).await.ok();
                        let j = rx.recv().await.unwrap();
                        consumer.push(j);
                    }
                });
            }
            let dispatcher = &dispatcher;
            let barrier = &barrier;
            s.spawn(async move {
                barrier.wait().await;            
                let dispatcher = dispatcher.lock().await.iter().map(|some_tx| some_tx.clone().unwrap()).collect::<Vec<_>>();
                for j in 0..N_TIMES * N_THREADS {
                    let i = producer_receiver.recv().await.unwrap();
                    dispatcher[i].send(j).await.ok();
                }
            });
        });
        print!("{:?}, ", start.elapsed());
    }
}

fn tokio_postage() {
    print!("\ntokio-postage: ");
    
    for _ in 1..=N_LOOP {
        let (producer_sender, mut producer_receiver) = postage::mpsc::channel::<usize>(N_TIMES);
        let dispatcher = tokio::sync::Mutex::new(vec![None; N_THREADS]);
        let consumers = (0..N_THREADS).map(|_| tokio::sync::Mutex::new(Vec::new())).collect::<Vec<_>>();
        let barrier = tokio::sync::Barrier::new(N_THREADS + 1);
        let start = Instant::now();    
        tokio_scoped::scope(|s| {
            for i in 0..N_THREADS {
                let mut producer_sender = producer_sender.clone();
                let dispatcher = &dispatcher;
                let consumers = &consumers;
                let barrier = &barrier;
                s.spawn(async move {
                    let (tx, mut rx) = postage::mpsc::channel::<usize>(N_TIMES);
                    dispatcher.lock().await[i] = Some(tx);
                    barrier.wait().await;                    
                    let mut consumer = consumers[i].lock().await;
                    for _ in 0..N_TIMES {
                        producer_sender.send(i).await.ok();
                        let j = rx.recv().await.unwrap();
                        consumer.push(j);
                    }
                });
            }
            let dispatcher = &dispatcher;
            let barrier = &barrier;
            s.spawn(async move {
                barrier.wait().await;            
                let mut dispatcher = dispatcher.lock().await.iter().map(|some_tx| some_tx.clone().unwrap()).collect::<Vec<_>>();
                for j in 0..N_TIMES * N_THREADS {
                    let i = producer_receiver.recv().await.unwrap();
                    dispatcher[i].send(j).await.ok();
                }
            });
        });
        print!("{:?}, ", start.elapsed());
    }
}
```
