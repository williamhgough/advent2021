#[aoc_generator(day6)]
pub fn parse(input: &str) -> Vec<usize> {
    let mut fish = vec![];
    while let Some(val) = input.split(',').next() {
        fish.push(val.parse::<usize>().unwrap());
    }
    println!("{:?}", fish);
    fish
}

#[aoc(day6, part1)]
pub fn solve_part1(input: &[usize]) -> usize {
    let mut fish = input.to_vec();
    let mut day_count = 0;
    while day_count < 80 {
        println!("{:?}", fish);
        let mut new_fish = vec![];
        fish.iter().for_each(|&f| {
            if f > 0 {
                new_fish.push(f - 1);
            } else {
                new_fish.push(6);
                new_fish.push(8);
            }
        });
        fish = new_fish.clone();
        day_count += 1;
    }

    fish.len()
}
