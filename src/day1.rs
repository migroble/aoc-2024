use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day1)]
pub fn input_gen(input: &str) -> (Vec<i64>, Vec<i64>) {
    input
        .lines()
        .map(|l| {
            let mut parts = l.split_whitespace().map(|d| d.parse::<i64>().unwrap());
            (parts.next().unwrap(), parts.next().unwrap())
        })
        .unzip()
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &(Vec<i64>, Vec<i64>)) -> i64 {
    let mut a = input.0.clone();
    let mut b = input.1.clone();

    a.sort();
    b.sort();

    a.into_iter().zip(b).map(|(a, b)| (a - b).abs()).sum()
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &(Vec<i64>, Vec<i64>)) -> i64 {
    let a = &input.0;
    let b = &input.1;

    a.iter()
        .map(|a| a * b.iter().filter(|b| a == *b).count() as i64)
        .sum()
}
