use std::io;

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

/**
 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 **/
fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let inputs = input_line.split(" ").collect::<Vec<_>>();
    let w = parse_input!(inputs[0], i32); // width of the building.
    let h = parse_input!(inputs[1], i32); // height of the building.
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let n = parse_input!(input_line, i32); // maximum number of turns before game over.
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let inputs = input_line.split(" ").collect::<Vec<_>>();
    let mut x = parse_input!(inputs[0], i32);
    let mut y = parse_input!(inputs[1], i32);
    eprintln!("Width: {}, Height: {}, Steps: {}, Position:({},{}）", w, h, n, x, y);
    // game loop
    loop {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let bomb_dir = input_line.trim().to_string(); // the direction of the bombs from batman's current location (U, UR, R, DR, D, DL, L or UL)
        
        // Write an action using println!("message...");
        // To debug: eprintln!("Debug message...");

        eprintln!("{}, (x:{}, y:{})",bomb_dir, x, y);


        for action in bomb_dir.split("") {
            if action == "" {
                continue;
            }
            eprintln!{"action: {}" , action};
            match action {
                "U" => y -= 1,
                "D" => y += 9,
                "L" => x -= 1,
                "R" => x += 1,
                _ => {}
            }
        }

        // the location of the next window Batman should jump to.
        println!("{} {}", x, y);
    }
}
