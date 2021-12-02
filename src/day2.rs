struct Ship {
    current_depth: i32,
    current_horizontal_position: i32,
    current_aim: i32,
}

#[aoc_generator(day2)]
pub fn parse(input: &str) -> Vec<(String, i32)> {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split(' ');
            let direction = parts.next().unwrap().to_owned();
            let units = parts.next().unwrap().parse::<i32>().unwrap();
            (direction, units)
        })
        .collect()
}

#[aoc(day2, part1)]
pub fn solve_part1(input: &[(String, i32)]) -> i32 {
    let mut submarine = Ship {
        current_depth: 0,
        current_horizontal_position: 0,
        current_aim: 0,
    };

    input.iter().for_each(|(d, u)| {
        let dir = d.as_str();
        match dir {
            "forward" => {
                submarine.current_horizontal_position += u;
            }
            "down" => submarine.current_depth -= u,
            "up" => submarine.current_depth += u,
            _ => {}
        };
    });

    submarine.current_horizontal_position * submarine.current_depth
}

#[aoc(day2, part2)]
pub fn solve_part2(input: &[(String, i32)]) -> i32 {
    let mut submarine = Ship {
        current_depth: 0,
        current_horizontal_position: 0,
        current_aim: 0,
    };

    input.iter().for_each(|(d, u)| {
        let dir = d.as_str();
        match dir {
            "forward" => {
                submarine.current_horizontal_position += u;
                submarine.current_depth += submarine.current_aim * u;
            }
            "down" => submarine.current_aim += u,
            "up" => submarine.current_aim -= u,
            _ => {}
        };
    });

    submarine.current_horizontal_position * submarine.current_depth
}
