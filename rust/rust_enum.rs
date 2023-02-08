use std::io;
use std::fmt::{Debug, Display, Formatter, Result};


macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}
#[derive(Debug, Clone, Copy)]
enum LogLevel {
    FATAL, ERROR, WARNING, INFO, DEBUG, UNKNOW
}

impl Display for LogLevel {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            LogLevel::FATAL => write!(f, "FATAL"),
            LogLevel::ERROR => write!(f, "ERROR"),
            LogLevel::WARNING => write!(f, "WARNING"),
            LogLevel::INFO => write!(f, "INFO"),
            LogLevel::DEBUG => write!(f, "DEBUG"),
            _ => write!(f, "UNKNOW")
        }
    }
}

impl From<LogLevel> for String {
    fn from(level: LogLevel) -> Self{
        match level {
            LogLevel::FATAL => "FATAL".into(),
            LogLevel::ERROR => "ERROR".into(),
            LogLevel::WARNING => "WARNING".into(),
            LogLevel::INFO => "INFO".into(),
            LogLevel::DEBUG => "DEBUG".into(),
            LogLevel::UNKNOW => "UNKNOW".into()
        }
    }
}

impl From<String> for LogLevel {
    fn from(level: String) -> Self {
        // println!("{}", level);
        match level {
            level if level == LogLevel::FATAL.to_string() || level == "FAT" => LogLevel::FATAL,
            level if level == LogLevel::ERROR.to_string() || level == "ERR"  => LogLevel::ERROR,
            level if level == LogLevel::WARNING.to_string() || level =="WAR" => LogLevel::WARNING,
            level if level == LogLevel::INFO.to_string() || level == "INF" => LogLevel::INFO,
            level if level == LogLevel::DEBUG.to_string() || level == "DEB"=> LogLevel::DEBUG,
            _ => LogLevel::UNKNOW
        }
    }

    // 碰到A 双倍, T 三倍， C 不变， G 消除
    fn test() {
        // Write an answer using println!("message...");
        // To debug: eprintln!("Debug message...");
        let arr = dna.split("");
        let mut a = "".to_string();
        for xx in arr {
            let s = xx.to_string();
            match s {
                s if s == "A" => a = a + "AA",
                s if s == "T" => a = a + "TTT",
                s if s == "C" => a = a + "C",
                _ => a = a + "" 
            }
        }
    }

    //  计算字符个数
    fn test2() {
        // Write an answer using println!("message...");
        // To debug: eprintln!("Debug message...");
        // eg.1
        let message = "To debug: eprintln!(\"Debug message...\")";
        let mut count = 0; 
        let message = message.chars().filter(|x| x.is_digit(10)).join("");
        // 数字 48-57
        // 大写 65-90
        // 小写 97-122
        for y in message.chars() {
            let z = y.to_string().parse::<i32>().unwrap() as u32;
            
            match z {
                65..=90 => {
                    count +=1;
                },
                97..=122 => {
                    count +=1;
                },
                _ => {}
            }
        }
        println!("{}", count);
        // eg.2

        let message = "To debug: eprintln!(\"Debug message...\")";
        let mut count = 0; 

        for y in message.chars() {
            match y {
                'a'..='z' => {
                    count +=1;
                },
                'A'..='Z' => {
                    count +=1;
                },
                _ => {}
            }
        }

        println!("{}", count);
    }
}


/**
 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 **/
fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let lvl = parse_input!(input_line, i32);
    // eprintln!("lvl: {}", lvl);
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let n = parse_input!(input_line, i32);
    for _i in 0..n as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let line = input_line.trim_matches('\n').to_string();
        eprintln!("line: {}", line);
        let mut split = line.split('>');
        let start  = split.next();
        let condition = split.next().unwrap(); 
        eprintln!("condition: {}", condition);
        let mut split = condition.split(':');
        let level_str = split.next().unwrap().trim();
        let level_uppercase = level_str.to_uppercase();
        // eprintln!("level charactor: {}", level_str);
        let level: LogLevel = level_uppercase.to_string().into();
        // eprintln!("level: {}, {}", level, level_str);
        let level_number = level as i32;
        // eprintln!("LEVEL:{}, level str {}, level: {}, lvl: {}, level <= lvl: {}", level, level_str, level_number, lvl, level_number <= lvl);

        if level_number <= lvl {
            let line = start.unwrap().to_string() + ">" + condition.replacen(level_str, level.to_string().as_str(), 1).as_str();
            println!("{}", line);
        }
    }

    // Write an answer using println!("message...");
    // To debug: eprintln!("Debug message...");
    
}

// REFERENCES:
// https://www.codingame.com/ide/demo/972988e4ddd83a948a90b8a9006932fb158b18
// https://zhuanlan.zhihu.com/p/454311400
// https://blog.csdn.net/eryk86/article/details/122958621