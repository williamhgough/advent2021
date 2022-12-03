#[aoc_generator(day8)]
pub fn parse(input: &str) -> Vec<usize> {
    input
        .trim()
        .split(',')
        .flat_map(|x| x.parse::<usize>())
        .collect()
}
