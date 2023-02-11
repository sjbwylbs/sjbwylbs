use std::io::{stdin, BufRead};

#[derive(Debug, Clone, Copy)]
struct Pair(i32, i32);

impl std::str::FromStr for Pair {
    type Err = Box<dyn std::error::Error + Send>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<_> = s
            .trim()
            .split_whitespace()
            .map(|i| i.parse().unwrap())
            .collect();
        Ok(Self(parts[0], parts[1]))
    }
}

type Building = Pair;
type Batman = Pair;

impl std::fmt::Display for Batman {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.0, self.1)
    }
}

#[derive(Debug, Clone, Copy)]
struct PossibleBombLocs {
    x_start: i32,
    y_start: i32,
    x_end: i32,
    y_end: i32,
}

#[derive(Debug, Clone, Copy)]
enum BombDir {
    Up,
    UpRight,
    Right,
    DownRight,
    Down,
    DownLeft,
    Left,
    UpLeft,
}

impl std::str::FromStr for BombDir {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "U" => Ok(Self::Up),
            "UR" => Ok(Self::UpRight),
            "R" => Ok(Self::Right),
            "DR" => Ok(Self::DownRight),
            "D" => Ok(Self::Down),
            "DL" => Ok(Self::DownLeft),
            "L" => Ok(Self::Left),
            "UL" => Ok(Self::UpLeft),
            _ => Err("Invalid Direction"),
        }
    }
}

pub fn main() {
    let stdin = stdin();
    let mut lines = stdin.lock().lines().map(Result::unwrap);
    let mut read_line = || lines.next().unwrap();
    let building = read_line().parse::<Building>().unwrap();
    let mut jumps = read_line().parse::<i32>().unwrap();
    let mut batman = read_line().parse::<Batman>().unwrap();

    let mut bomb_loc = PossibleBombLocs {
        x_start: 0,
        y_start: 0,
        x_end: building.0,
        y_end: building.1,
    };

    while jumps > 0 {
        let direction = read_line().parse::<BombDir>().unwrap();
        update_possible_bomb_locs(&mut bomb_loc, direction, batman);
        set_new_batman_pos(&mut batman, &bomb_loc);
        println!("{}", batman);
        if bomb_loc.x_end - bomb_loc.x_start <= 1 && bomb_loc.y_end - bomb_loc.y_start <= 1 {
            break;
        }
        jumps -= 1;
    }
}

fn set_new_batman_pos(batman: &mut Batman, bomb_loc: &PossibleBombLocs) {
    let new_x = (bomb_loc.x_start + bomb_loc.x_end) / 2;
    let new_y = (bomb_loc.y_start + bomb_loc.y_end) / 2;
    batman.0 = new_x;
    batman.1 = new_y;
}

fn update_possible_bomb_locs(bomb_loc: &mut PossibleBombLocs, direction: BombDir, batman: Batman) {
    match direction {
        BombDir::Up => {
            bomb_loc.x_start = batman.0;
            bomb_loc.x_end = batman.0 + 1;
            bomb_loc.y_end = batman.1;
        }
        BombDir::UpRight => {
            bomb_loc.x_start = batman.0 + 1;
            bomb_loc.y_end = batman.1;
        }
        BombDir::Right => {
            bomb_loc.x_start = batman.0 + 1;
            bomb_loc.y_start = batman.1;
            bomb_loc.y_end = batman.1 + 1;
        }
        BombDir::DownRight => {
            bomb_loc.x_start = batman.0 + 1;
            bomb_loc.y_start = batman.1 + 1;
        }
        BombDir::Down => {
            bomb_loc.x_start = batman.0;
            bomb_loc.x_end = batman.0 + 1;
            bomb_loc.y_start = batman.1 + 1;
        }
        BombDir::DownLeft => {
            bomb_loc.x_end = batman.0;
            bomb_loc.y_start = batman.1 + 1;
        }
        BombDir::Left => {
            bomb_loc.x_end = batman.0;
            bomb_loc.y_start = batman.1;
            bomb_loc.y_end = batman.1 + 1;
        }
        BombDir::UpLeft => {
            bomb_loc.x_end = batman.0;
            bomb_loc.y_end = batman.1;
        }
    }
}


// fn read_input<T: std::str::FromStr>() -> T {
//     let mut input_line = String::new();
//     std::io::stdin().read_line(&mut input_line).unwrap();
//     input_line.trim_matches('\n').parse().ok().unwrap()
// }

// fn read_inputs<T: std::str::FromStr>() -> Vec<T> {
//     read_input::<String>().split_whitespace().map(|s| s.parse().ok().unwrap()).collect()
// }

// fn main() {
//     let building = read_inputs::<i32>();
//     let mut xmax = building[0]; // width of the building.
//     let mut ymax = building[1]; // height of the building.
//     let _n: i32 = read_input(); // maximum number of turns before game over.
//     let start = read_inputs::<i32>();
//     let mut x0 = start[0];
//     let mut y0 = start[1];

//     let mut xmin = 0;
//     let mut ymin = 0;

//     loop {
//         let bomb_dir: String = read_input(); // the direction of the bombs from batman's current location (U, UR, R, DR, D, DL, L or UL)

//         if !bomb_dir.contains("D") { ymax = y0 }
//         if !bomb_dir.contains("U") { ymin = y0 + 1 }
//         if !bomb_dir.contains("R") { xmax = x0 }
//         if !bomb_dir.contains("L") { xmin = x0 + 1 }

//         x0 = (xmin + xmax) / 2;
//         y0 = (ymin + ymax) / 2;

//         // the location of the next window Batman should jump to.
//         println!("{} {}", x0, y0);
//     }
// }

// use std::io;

// macro_rules! print_err {
//     ($($arg:tt)*) => (
//         {
//             use std::io::Write;
//             writeln!(&mut ::std::io::stderr(), $($arg)*).ok();
//         }
//     )
// }

// macro_rules! parse_input {
//     ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
// }

// fn read_building_props() -> (i32, i32) {
//     let mut input_line = String::new();
//     io::stdin().read_line(&mut input_line).unwrap();
//     let inputs = input_line.split(" ").collect::<Vec<_>>();
//     let w = parse_input!(inputs[0], i32); // width of the building.
//     let h = parse_input!(inputs[1], i32); // height of the building.
    
//     return (w, h);
// }

// fn read_jump_count() -> i32 {
//     let mut input_line = String::new();
//     io::stdin().read_line(&mut input_line).unwrap();
//     return parse_input!(input_line, i32); // maximum number of turns before game over.
// }

// fn read_pos() -> (i32, i32) {
//     let mut input_line = String::new();
//     io::stdin().read_line(&mut input_line).unwrap();
//     let inputs = input_line.split(" ").collect::<Vec<_>>();
//     let x0 = parse_input!(inputs[0], i32);
//     let y0 = parse_input!(inputs[1], i32);
    
//     return (x0, y0);
// }



// fn main() {
//     let (w, h)          =   read_building_props();
//     let _n              =   read_jump_count();
//     let (mut x, mut y)  =   read_pos();
    
//     let mut x_min   =   0;
//     let mut x_max   =   w;
//     let mut y_min   =   0;
//     let mut y_max   =   h;
    

//     // game loop
//     loop {
//         let mut input_line = String::new();
//         io::stdin().read_line(&mut input_line).unwrap();
//         let bomb_dir = input_line.trim().to_string(); 
        
//         if bomb_dir.contains('U')   { y_max = y; }
//         if bomb_dir.contains('D')   { y_min = y; }
//         if bomb_dir.contains('R')   { x_min = x; }
//         if bomb_dir.contains('L')   { x_max = x; }
        
//         x = (x_min + x_max) / 2;
//         y = (y_min + y_max) / 2;
        
//         println!("{} {}", x, y);
//     }
// }

// use std::io;

// macro_rules! parse_input {
//     ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
// }

// /**
//  * Auto-generated code below aims at helping you parse
//  * the standard input according to the problem statement.
//  **/
// fn main() {
//     let mut input_line = String::new();
//     io::stdin().read_line(&mut input_line).unwrap();
//     let inputs = input_line.split(" ").collect::<Vec<_>>();
//     let w = parse_input!(inputs[0], i32); // width of the building.
//     let h = parse_input!(inputs[1], i32); // height of the building.
//     let mut input_line = String::new();
//     io::stdin().read_line(&mut input_line).unwrap();
//     let mut n = parse_input!(input_line, i32); // maximum number of turns before game over.
//     let mut input_line = String::new();
//     io::stdin().read_line(&mut input_line).unwrap();
//     let inputs = input_line.split(" ").collect::<Vec<_>>();
//     let mut x = parse_input!(inputs[0], i32);
//     let mut y = parse_input!(inputs[1], i32);
//     eprintln!("Width: {}, Height: {}, Steps: {}, Position:({},{}）", w, h, n, x, y);
//     // set map border
//     let mut x1 = 0;
//     let mut y1 = 0;
//     let mut x2 = w - 1;
//     let mut y2 = y - 1;
//     // game loop
//     loop {
//         let mut input_line = String::new();
//         io::stdin().read_line(&mut input_line).unwrap();
//         let bomb_dir = input_line.trim().to_string(); // the direction of the bombs from batman's current location (U, UR, R, DR, D, DL, L or UL)
        
//         // Write an action using println!("message...");
//         // To debug: eprintln!("Debug message...");

//         if bomb_dir.contains('U') {
//             y2 = y - 1;  // UP , decrement max of the h
//         }else if bomb_dir.contains('D') {
//             y1 = y + 1; // DOWN , increment start of the y
//         }

//         if bomb_dir.contains('L') {
//             x2 = x - 1; // LEFT, decrement max of the w 
//         } else if bomb_dir.contains('R') {
//             x1 = x + 1; // RIGHT, increment start of the x 
//         }

//         eprintln!("Direction: {}, Area (x1:{}, y1:{}, x2:{}, y2:{})", bomb_dir, x1, y1, x2, y2);
        
//         x = x1 + (x2 - x1) / 2;
//         y = y1 + (y2 - y1) / 2;

//         n -= 1 ;
        
//         eprintln!("distance x: {}, y: {}, n: {} ", y2 - y1, x2 - x1, n);
//         // the location of the next window Batman should jump to.
//         println!("{} {}", x, y);
//     }
// }

// REFs
//https://www.codingame.com/ide/puzzle/shadows-of-the-knight-episode-1