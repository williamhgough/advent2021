pub struct Ship {
    pub current_depth: i32,
    pub current_horizontal_position: i32,
    pub current_aim: i32,
}

pub enum Direction {
    Forward,
    Down,
    Up,
}

pub struct Instruction {
    pub direction: Direction,
    pub units: i32,
}

impl Ship {
    pub fn find_position(&self) -> i32 {
        self.current_horizontal_position * self.current_depth
    }

    pub fn process_part_2(self, instr: &Instruction) -> Self {
        match instr.direction {
            Direction::Forward => Self {
                current_horizontal_position: self.current_horizontal_position + instr.units,
                current_depth: self.current_depth + (self.current_aim * instr.units),
                ..self
            },
            Direction::Down => Self {
                current_aim: self.current_aim + instr.units,
                ..self
            },
            Direction::Up => Self {
                current_aim: self.current_aim - instr.units,
                ..self
            },
        }
    }

    pub fn process_part_1(self, instr: &Instruction) -> Self {
        match instr.direction {
            Direction::Forward => Self {
                current_horizontal_position: self.current_horizontal_position + instr.units,
                ..self
            },
            Direction::Down => Self {
                current_depth: self.current_depth - instr.units,
                ..self
            },
            Direction::Up => Self {
                current_depth: self.current_depth + instr.units,
                ..self
            },
        }
    }
}

impl Default for Ship {
    fn default() -> Self {
        Self {
            current_depth: 0,
            current_horizontal_position: 0,
            current_aim: 0,
        }
    }
}

#[aoc_generator(day2)]
pub fn parse(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split(' ');
            let direction = match parts.next().unwrap() {
                "forward" => Direction::Forward,
                "down" => Direction::Down,
                "up" => Direction::Up,
                _ => Direction::Forward,
            };
            let units = parts.next().unwrap().parse::<i32>().unwrap();

            Instruction { direction, units }
        })
        .collect()
}

#[aoc(day2, part1)]
pub fn solve_part1(instructions: &[Instruction]) -> i32 {
    instructions
        .iter()
        .fold(Ship::default(), Ship::process_part_1)
        .find_position()
}

#[aoc(day2, part2)]
pub fn solve_part2(instructions: &[Instruction]) -> i32 {
    instructions
        .iter()
        .fold(Ship::default(), Ship::process_part_2)
        .find_position()
}
