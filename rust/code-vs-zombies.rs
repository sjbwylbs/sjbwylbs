use std::io;

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}
#[derive(Debug,PartialEq,Eq)]
struct Point {
    x: i32,
    y: i32
}

impl Point {
    fn new(x:i32, y:i32) -> Point {
        Point {x, y}
    }
    fn distance(&self,  other: &Point) -> f32{
        let xx = self.x.abs_diff(other.x);
        let yy = self.y.abs_diff(other.y);
        eprintln!("abs({}-{}) = {}", self.x, other.x, xx);
        eprintln!("abs({}-{}) = {}", self.y, other.y, yy);
        ((xx * xx + yy * yy) as f32).sqrt()
    }
}

/**
 * Save humans, destroy zombies!
 **/
fn main() {

    // game loop
    loop {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        let x = parse_input!(inputs[0], i32);
        let y = parse_input!(inputs[1], i32);
        eprintln!("position[x:{},y:{}]", x, y);
        let user = Point::new(x,y);
        let mut move_x = x;
        let mut move_y = y;
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let human_count = parse_input!(input_line, i32);
        let mut vec_human:Vec<Point> = Vec::new();
        for i in 0..human_count as usize {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            let inputs = input_line.split(" ").collect::<Vec<_>>();
            let human_id = parse_input!(inputs[0], i32);
            let human_x = parse_input!(inputs[1], i32);
            let human_y = parse_input!(inputs[2], i32);
            eprintln!("human-id:{}-x:{},y:{}",human_id, human_x, human_y);
            vec_human.push(Point::new(human_x,human_y));
        }
        eprintln!("{:?}", vec_human);

        let mut min:f32 = 100_000.0;
        for temp in vec_human {
            let t = temp.distance(&user);
            eprintln!("distance:{}", t);
            if t < min {
                min = t;
                move_x = temp.x;
                move_y = temp.y;
            }
        }

        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let zombie_count = parse_input!(input_line, i32);
        let mut vec_zombie:Vec<Point> = Vec::new();
        for i in 0..zombie_count as usize {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            let inputs = input_line.split(" ").collect::<Vec<_>>();
            let zombie_id = parse_input!(inputs[0], i32);
            let zombie_x = parse_input!(inputs[1], i32);
            let zombie_y = parse_input!(inputs[2], i32);
            let zombie_xnext = parse_input!(inputs[3], i32);
            let zombie_ynext = parse_input!(inputs[4], i32);

            eprintln!("zombie-id:{}-x:{},y:{},xnext:{},ynext:{}",zombie_id, zombie_x, zombie_y, zombie_xnext, zombie_ynext);
            //move_x = zombie_xnext;
            //move_y = zombie_ynext;
            vec_zombie.push(Point::new(zombie_x,zombie_y));
        }
        eprintln!("{:?}", vec_zombie);

        // Write an action using println!("message...");
        // To debug: eprintln!("Debug message...");
        eprintln!("{} {}",move_x, move_y);
        println!("{} {}",move_x, move_y); // Your destination coordinates
    }
}


//REFs
// https://www.codingame.com/ide/puzzle/code-vs-zombies