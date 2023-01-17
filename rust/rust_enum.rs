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
            LogLevel::WARNING => write!(f, "WARING"),
            LogLevel::INFO => write!(f, "WARING"),
            LogLevel::DEBUG => write!(f, "DEBUG"),
            _ => write!(f, "UNKNOW")
        }
    }
}

impl From<LogLevel> for String {
    fn from(level: LogLevel) -> Self{
        println!("FATAL impl form");
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
        match level {
            // 为什么这里不能比较？
            level if level == LogLevel::FATAL.to_string() => {
                println!("FATAL for LogLevel : {}", level);
                LogLevel::FATAL
            },
            level if level == LogLevel::ERROR.to_string() || level == "ERR"  => LogLevel::ERROR,
            level if level == LogLevel::WARNING.to_string() || level =="WARN" => LogLevel::WARNING,
            level if level == LogLevel::INFO.to_string() => LogLevel::INFO,
            level if level == LogLevel::DEBUG.to_string() => {
                println!("DEBUG for LogLevel : {}", level);
                LogLevel::DEBUG
            },
            _ => {
                // 为什么用 level 的值是FATAL会到这里来？
                println!("Default for LogLevel : {}", level);
                LogLevel::UNKNOW
            }
        }
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
    // println!("lvl: {}", lvl);
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let n = parse_input!(input_line, i32);
    for _i in (0..n as usize) {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let line = input_line.trim_matches('\n').to_string();
        // println!("line: {}", line);
        let mut split = line.split('>');
        split.next();
        let condition = split.next().unwrap(); // FATAL: could not
        // println!("condition: {}", condition);
        let mut split = condition.split(':');
        let level_str = split.next().unwrap();
        println!("level charactor: {}", level_str);
        let level: LogLevel = level_str.to_string().into();
        println!("level: {}", level);
        let level_number = level as i32;
        println!("LEVEL:{}, level: {}, lvl: {}, level <= lvl: {}", level, level_number, lvl, level_number <= lvl);
        if level_number <= lvl {
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