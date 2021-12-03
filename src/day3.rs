use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Calculator {
    pub store: HashMap<i32, Vec<i32>>,
    pub instructions: Vec<Instruction>,
    pub gamma: String,
    pub epsilon: String,
}

impl Calculator {
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

    fn ones_and_zeroes(&self, index: i32) -> (usize, usize) {
        (
            self.store
                .get(&index)
                .unwrap()
                .iter()
                .filter(|&&x| x == 1)
                .count(),
            self.store
                .get(&index)
                .unwrap()
                .iter()
                .filter(|&&x| x == 0)
                .count(),
        )
    }

    fn gamma_rate(&mut self) -> i64 {
        for i in 0..self.store.keys().len() {
            let (ones, zeroes) = self.ones_and_zeroes(i as i32);

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
            let (ones, zeroes) = self.ones_and_zeroes(i as i32);

            if ones.lt(&zeroes) {
                self.epsilon = format!("{}{}", self.epsilon, 1);
            } else {
                self.epsilon = format!("{}{}", self.epsilon, 0);
            }
        }

        i64::from_str_radix(&self.epsilon, 2).unwrap()
    }
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

    for i in 0..instructions.get(0).unwrap().binary.len() {
        calc.store_bit_at_index(i)
    }

    calc.gamma_rate() * calc.epsilon_rate()
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
}
