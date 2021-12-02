use std::fs;

fn main() {
    let f = fs::read_to_string("./day2/input.txt").expect("Unable to open file!");
    let lines = f.split('\n').collect::<Vec<&str>>();

    let mut submarine = Ship {
        current_depth: 0,
        current_horizontal_position: 0,
        current_aim: 0,
    };

    for line in &lines {
        let mut parts = line.split_whitespace();
        let direction = parts.next().unwrap();
        let depth = parts.next().unwrap().parse::<i32>().unwrap();

        println!("{} {}", direction, depth);

        match direction {
            "forward" => {
                submarine.current_horizontal_position += depth;
                submarine.current_depth += submarine.current_aim * depth;
            }
            "down" => submarine.current_aim += depth,
            "up" => submarine.current_aim -= depth,
            _ => {}
        };
    }

    println!(
        "Ships position: \n
        Horizontal: {}\n
        Depth: {}\n
        Final position: {}\n",
        submarine.current_horizontal_position,
        submarine.current_depth,
        submarine.current_horizontal_position * submarine.current_depth
    );
}

struct Ship {
    current_depth: i32,
    current_horizontal_position: i32,
    current_aim: i32,
}
