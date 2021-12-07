#[aoc_generator(day7)]
pub fn parse(input: &str) -> Vec<usize> {
    input
        .trim()
        .split(',')
        .flat_map(|x| x.parse::<usize>())
        .collect()
}

#[aoc(day7, part1)]
pub fn solve_part1(input: &[usize]) -> usize {
    let mut sorted = input.to_vec();
    sorted.sort();
    let median_position = median(&sorted);

    input.iter().fold(0, |acc, &x| {
        acc + (x as i16 - median_position as i16).abs() as usize
    })
}

#[aoc(day7, part2)]
pub fn solve_part2(input: &[usize]) -> usize {
    let mean_position = mean(input);

    input.iter().fold(0, |acc, &v| {
        let positions_to_move = if mean_position > v {
            mean_position - v
        } else {
            v - mean_position
        };
        let fuel_requirements = positions_to_move * (1 + positions_to_move) / 2;
        acc + fuel_requirements
    })
}

fn median(sorted_array: &[usize]) -> usize {
    if (sorted_array.len() % 2) == 0 {
        let ind_left = sorted_array.len() / 2 - 1;
        let ind_right = sorted_array.len() / 2;
        ((sorted_array[ind_left] + sorted_array[ind_right]) / 2) as usize
    } else {
        sorted_array[(sorted_array.len() / 2)]
    }
}

fn mean(numbers: &[usize]) -> usize {
    numbers.iter().sum::<usize>() / numbers.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one() {
        assert_eq!(solve_part1(&parse("16,1,2,0,4,2,7,1,2,14")), 37);
    }

    #[test]
    fn part_two() {
        assert_eq!(solve_part2(&parse("16,1,2,0,4,2,7,1,2,14")), 170);
    }
}
