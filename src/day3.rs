use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Calculator {
    pub store: HashMap<i32, Vec<i32>>,
    pub instructions: Vec<Instruction>,
    pub gamma: String,
    pub epsilon: String,
}

impl Calculator {
    fn store_input(&mut self) {
        for i in 0..self.instructions.get(0).unwrap().binary.len() {
            self.store_bit_at_index(i)
        }
    }

    fn store_bit_at_index(&mut self, bit: usize) {
        self.instructions.iter().for_each(|instr| {
            let bin_str = instr.binary.as_str();
            let val: i32 = match bin_str.chars().nth(bit) {
                None => 0,
                Some(x) => x.to_string().parse::<i32>().unwrap(),
            };

            match self.store.get_mut(&(*&bit as i32)) {
                None => {
                    self.store.insert(*&bit as i32, vec![val]);
                }
                Some(v) => v.push(val),
            };
        })
    }

    fn gamma_rate(&mut self) -> i64 {
        for i in 0..self.store.keys().len() {
            let (ones, zeroes) = ones_and_zeroes_for_set(self.instructions.clone(), i);

            if ones.gt(&zeroes) {
                self.gamma = format!("{}{}", self.gamma, 1);
            } else {
                self.gamma = format!("{}{}", self.gamma, 0);
            }
        }

        i64::from_str_radix(&self.gamma, 2).unwrap()
    }

    fn epsilon_rate(&mut self) -> i64 {
        for i in 0..self.store.keys().len() {
            let (ones, zeroes) = ones_and_zeroes_for_set(self.instructions.clone(), i);

            if ones.lt(&zeroes) {
                self.epsilon = format!("{}{}", self.epsilon, 1);
            } else {
                self.epsilon = format!("{}{}", self.epsilon, 0);
            }
        }

        i64::from_str_radix(&self.epsilon, 2).unwrap()
    }

    fn oxygen_generator_rating(&self) -> i64 {
        let mut filtered = self.instructions.clone();
        let len = filtered.last().unwrap().binary.len();
        for i in 0..len {
            let (ones, zeroes) = ones_and_zeroes_for_set(filtered.clone(), i);

            if filtered.len() == 1 {
                return i64::from_str_radix(&filtered.last().unwrap().binary, 2).unwrap();
            }

            if filtered.len() == 2 && ones == zeroes {
                filtered = filtered
                    .into_iter()
                    .filter(|x| x.binary.as_str().chars().last().unwrap() == '1')
                    .collect();
                continue;
            }

            if ones > zeroes {
                filtered = filtered
                    .into_iter()
                    .filter(|x| x.binary.as_str().chars().nth(i).unwrap() == '1')
                    .collect();
            } else {
                filtered = filtered
                    .into_iter()
                    .filter(|x| x.binary.as_str().chars().nth(i).unwrap() == '0')
                    .collect();
            }
        }

        i64::from_str_radix(&filtered.last().unwrap().binary, 2).unwrap()
    }

    fn co2_scrubber(&self) -> i64 {
        let mut filtered = self.instructions.clone();
        let len = filtered.last().unwrap().binary.len();
        for i in 0..len {
            let (ones, zeroes) = ones_and_zeroes_for_set(filtered.clone(), i);

            if filtered.len() == 1 {
                return i64::from_str_radix(&filtered.last().unwrap().binary, 2).unwrap();
            }

            if filtered.len() == 2 && ones == zeroes {
                filtered = filtered
                    .into_iter()
                    .filter(|x| x.binary.as_str().chars().last().unwrap() == '0')
                    .collect();
                continue;
            }

            if ones < zeroes {
                filtered = filtered
                    .into_iter()
                    .filter(|x| x.binary.as_str().chars().nth(i).unwrap() == '1')
                    .collect();
            } else {
                filtered = filtered
                    .into_iter()
                    .filter(|x| x.binary.as_str().chars().nth(i).unwrap() == '0')
                    .collect();
            }
        }

        i64::from_str_radix(&filtered.last().unwrap().binary, 2).unwrap()
    }
}

fn ones_and_zeroes_for_set(instructions: Vec<Instruction>, bit: usize) -> (i32, i32) {
    let mut store = vec![];
    instructions.iter().for_each(|instr| {
        let bin_str = instr.binary.as_str();
        let val: i32 = match bin_str.chars().nth(bit) {
            None => 0,
            Some(x) => x.to_string().parse::<i32>().unwrap(),
        };

        store.push(val);
    });
    (
        store.iter().filter(|&&x| x == 1).count() as i32,
        store.iter().filter(|&&x| x == 0).count() as i32,
    )
}

#[derive(Clone, Debug)]
pub struct Instruction {
    pub binary: String,
}

#[aoc_generator(day3)]
pub fn parse(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|s| Instruction {
            binary: String::from(s),
        })
        .collect()
}

#[aoc(day3, part1)]
pub fn solve_part1(instructions: &[Instruction]) -> i64 {
    let mut calc = Calculator {
        store: HashMap::new(),
        instructions: instructions.to_vec(),
        gamma: String::from(""),
        epsilon: String::from(""),
    };

    calc.store_input();

    calc.gamma_rate() * calc.epsilon_rate()
}

#[aoc(day3, part2)]
pub fn solve_part2(instructions: &[Instruction]) -> i64 {
    let calc = Calculator {
        store: HashMap::new(),
        instructions: instructions.to_vec(),
        gamma: String::from(""),
        epsilon: String::from(""),
    };

    calc.oxygen_generator_rating() * calc.co2_scrubber()
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = r"00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";

    #[test]
    fn part1_example() {
        assert_eq!(solve_part1(&parse(&TEST_INPUT)), 198);
    }

    #[test]
    fn oxygen_test() {
        let mut calc = Calculator {
            store: HashMap::new(),
            instructions: parse(&TEST_INPUT),
            gamma: "".to_string(),
            epsilon: "".to_string(),
        };

        calc.store_input();

        assert_eq!(calc.oxygen_generator_rating(), 23);
    }

    #[test]
    fn co2_test() {
        let mut calc = Calculator {
            store: HashMap::new(),
            instructions: parse(&TEST_INPUT),
            gamma: "".to_string(),
            epsilon: "".to_string(),
        };

        calc.store_input();

        assert_eq!(calc.co2_scrubber(), 10);
    }
}
