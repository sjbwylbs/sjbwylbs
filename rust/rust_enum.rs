// https://www.codingame.com/ide/demo/972988e4ddd83a948a90b8a9006932fb158b18
use std::convert::TryInto;
use std::io;
use std::fmt::{Debug, Display, Formatter, Result};


macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}
#[derive(Debug)]
enum LogLevel {
    FATAL = 0, ERROR = 1, WARNING = 2, INFO = 3, DEBUG = 4
}

impl Display for LogLevel {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            LogLevel::FATAL => write!(f, "FATAL"),
            LogLevel::ERROR => write!(f, "ERROR"),
            LogLevel::WARNING => write!(f, "WARING"),
            LogLevel::INFO => write!(f, "WARING"),
            LogLevel::DEBUG => write!(f, "DEBUG")
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
        }
    }
}

impl From<String> for LogLevel {
    fn from(level: String) -> Self {
        match level {
            level if level == "FATAL" => LogLevel::FATAL,
            level if level == "ERROR" || level == "ERR"  => LogLevel::ERROR,
            level if level == "WARNING" || level =="WARN" => LogLevel::WARNING,
            level if level == "INFO" => LogLevel::INFO,
            level if level == "DEBUG" => LogLevel::DEBUG,
            _ => LogLevel::DEBUG,
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
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let n = parse_input!(input_line, i32);
    for i in 0..n as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let line = input_line.trim_matches('\n').to_string();
        let mut split = line.split('>');
        let mut condition = split.next(); // FATAL: could not
        condition.map( x => x.)
        let mut contains_comma 
        let level: LogLevel = line.to_string().into();
        let level_number = level as i32;
        if level_number <= lvl {
            println!("{}", line);
        }
    }

    // Write an answer using println!("message...");
    // To debug: eprintln!("Debug message...");
    
}
