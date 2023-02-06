use std::io;

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

/**
 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 **/
fn main() {

    let mut next_checkpoint_changed = false;
    let mut last_next_checkpoint_x:i32 = 0;
    let mut last_next_checkpoint_y:i32 = 0;
    let mut distance = 0;
    // game loop
    loop {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        let x = parse_input!(inputs[0], i32);
        let y = parse_input!(inputs[1], i32);

        let next_checkpoint_x = parse_input!(inputs[2], i32); // x position of the next check point
        let next_checkpoint_y = parse_input!(inputs[3], i32); // y position of the next check point
        let next_checkpoint_dst = parse_input!(inputs[4], i32); // distance to the next checkpoint
        let next_checkpoint_angle = parse_input!(inputs[5], i32); // angle between your pod orientation and the direction of the next checkpoint

        next_checkpoint_changed = next_checkpoint_x != last_next_checkpoint_x && next_checkpoint_y != last_next_checkpoint_y;
        if next_checkpoint_changed {
            distance = (next_checkpoint_dst as f32 * 0.2) as i32;
            last_next_checkpoint_x = next_checkpoint_x;
            last_next_checkpoint_y = next_checkpoint_y;
        }

        eprintln!("A:{},{}, B:{},{}, check_dst:{}, angle:{}, distance:{}", x,y,next_checkpoint_x,next_checkpoint_y, next_checkpoint_dst,next_checkpoint_angle,distance);
        // opponent 
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        eprintln!("opponent: {}", inputs.join(" "));
        let opponent_x = parse_input!(inputs[0], i32);
        let opponent_y = parse_input!(inputs[1], i32);

        // Write an action using println!("message...");
        // To debug: eprintln!("Debug message...");


        // You have to output the target position
        // followed by the power (0 <= thrust <= 100)
        // i.e.: "x y thrust"
        let mut thrust = 100;
        let mut can_with_boost = false;
        if next_checkpoint_dst > distance {
            can_with_boost = true;
        } else {
            thrust = 50;
        }

        if next_checkpoint_angle > 90 || next_checkpoint_angle < -90 {
            thrust = 0;
        }

        if thrust == 0 {
            can_with_boost = false;
        }

        if can_with_boost {
            eprintln!("{} {} BOOST", next_checkpoint_x, next_checkpoint_y);
            println!("{} {} BOOST", next_checkpoint_x, next_checkpoint_y);
        } else {
            eprintln!("{} {} {}", next_checkpoint_x, next_checkpoint_y, thrust);
            println!("{} {} {}", next_checkpoint_x, next_checkpoint_y, thrust);
        }
    }
}




// REFs
// https://www.codingame.com/ide/puzzle/mad-pod-racing