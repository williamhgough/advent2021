use std::fs;

fn main() {
    let f = fs::read_to_string("./input.txt").expect("Unable to open file!");
    let lines = f.split('\n').collect::<Vec<&str>>();

    let mut prev_num = 0;
    let mut num_increased = 0;
    for &i in &lines {
        let num = i.parse::<i32>().unwrap();

        if num.gt(&prev_num) && !prev_num.eq(&0) {
            num_increased += 1;
        }
        prev_num = num;
    }

    println!("Part 1: Number increased: {}", num_increased);

    let mut prev_sum = 0;
    num_increased = 0;
    for i in 0..lines.len() - 2 {
        let num = lines[i].parse::<i32>().unwrap()
            + lines[i + 1].parse::<i32>().unwrap()
            + lines[i + 2].parse::<i32>().unwrap();
        if num.gt(&prev_sum) && !prev_sum.eq(&0) {
            num_increased += 1;
        }
        prev_sum = num;
    }

    println!("Part 2: Number increased: {}", num_increased);
}
