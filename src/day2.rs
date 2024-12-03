use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day2)]
pub fn input_gen(input: &str) -> Vec<Vec<i64>> {
    input
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(|d| d.parse::<i64>().unwrap())
                .collect()
        })
        .collect()
}

fn is_report_valid(report: &[i64]) -> bool {
    let increasing = report[1] > report[0];

    report
        .windows(2)
        .find(|levels| {
            let diff = (levels[0] - levels[1]).abs();

            !(diff <= 3
                && diff >= 1
                && if increasing {
                    levels[1] > levels[0]
                } else {
                    levels[1] < levels[0]
                })
        })
        .is_none()
}

#[aoc(day2, part1)]
pub fn solve_part1(input: &[Vec<i64>]) -> usize {
    input
        .iter()
        .filter(|report| is_report_valid(report))
        .count()
}

#[aoc(day2, part2)]
pub fn solve_part2(input: &[Vec<i64>]) -> usize {
    input
        .into_iter()
        .filter(|report| {
            (-1..report.len() as i64)
                .find(|n| {
                    let report = report
                        .iter()
                        .enumerate()
                        .filter(|(i, _)| *i as i64 != *n)
                        .map(|(_, l)| *l)
                        .collect::<Vec<i64>>();

                    is_report_valid(&report)
                })
                .is_some()
        })
        .count()
}
