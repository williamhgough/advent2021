extern crate nalgebra as na;

use na::{DMatrix, RowDVector};

#[aoc_generator(day9)]
pub fn parse(input: &str) -> DMatrix<u32> {
    let rows: Vec<RowDVector<u32>> = input
        .lines()
        .map(|line| {
            let row: Vec<u32> = line.chars().into_iter().map(|f| f.into()).collect();
            RowDVector::from(row)
        })
        .collect();
    DMatrix::from_rows(&rows)
}

#[aoc(day9, part1)]
pub fn solve_part1(input: &DMatrix<u32>) -> usize {
    0
}
