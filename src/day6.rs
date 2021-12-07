const NUMBER_DAYS: usize = 9;
const NUMBER_GENERATIONS_SMALL: usize = 80;
const NUMBER_GENERATIONS_BIG: usize = 256;

#[aoc_generator(day6)]
pub fn parse(input: &str) -> Vec<usize> {
    let mut initial_population = [0; NUMBER_DAYS];
    input.trim().split(',').for_each(|age| {
        initial_population[age.parse::<usize>().expect("failed to parse fish")] += 1
    });
    initial_population.to_vec()
}

fn calculate_populations(population: &mut Vec<usize>, generations: usize) -> usize {
    (0..generations).for_each(|_| {
        let day_0 = population[0];
        (0..NUMBER_DAYS - 1).for_each(|i| population[i] = population[i + 1]);
        population[6] += day_0;
        population[8] = day_0;
    });
    population.iter().sum()
}

#[aoc(day6, part1)]
pub fn solve_part1(input: &[usize]) -> usize {
    calculate_populations(&mut input.to_vec(), NUMBER_GENERATIONS_SMALL)
}

#[aoc(day6, part2)]
pub fn solve_part2(input: &[usize]) -> usize {
    calculate_populations(&mut input.to_vec(), NUMBER_GENERATIONS_BIG)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one() {
        assert_eq!(solve_part1(&parse("3,4,3,1,2")), 5934);
    }

    #[test]
    fn part_two() {
        assert_eq!(solve_part2(&parse("3,4,3,1,2")), 26984457539);
    }
}
