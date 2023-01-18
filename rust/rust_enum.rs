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