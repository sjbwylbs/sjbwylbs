use std::io;

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

/**
 * The while loop represents the game.
 * Each iteration represents a turn of the game
 * where you are given inputs (the heights of the mountains)
 * and where you have to print an output (the index of the mountain to fire on)
 * The inputs you are given are automatically updated according to your last actions.
 **/
fn main() -> ! {

    // game loop
    loop {
        let mut mountain_hs = vec![];
        for i in 0..8 as usize {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            let mountain_h = parse_input!(input_line, i32); // represents the height of one mountain.
            eprintln!("Mountain Height: {}", mountain_h);
            mountain_hs.push((i, mountain_h))
        }

        mountain_hs.sort_by_key( |k| k.1);
        println!("{}", mountain_hs.pop().unwrap().0);
        // let duration = std::time::Duration::from_secs(50);
        // std::io::thread::sleep(duration);
        // Write an action using println!("message...");
        // To debug: eprintln!("Debug message...");

        // println!("4"); // The index of the mountain to fire on.
    }
}

// REFs
// https://www.codingame.com/ide/puzzle/the-descent