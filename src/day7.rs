use std::fmt::Debug;
use std::iter::repeat_n;

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day7)]
pub fn input_gen(input: &str) -> Vec<(i64, Vec<i64>)> {
    input
        .lines()
        .map(|l| {
            let mut parts = l.split(':');
            let sum = parts.next().unwrap().parse::<i64>().unwrap();
            let nums = parts
                .next()
                .unwrap()
                .split_whitespace()
                .map(|d| d.parse::<i64>().unwrap())
                .collect();

            (sum, nums)
        })
        .collect()
}

#[derive(Debug, Clone, Copy)]
enum Op {
    Add,
    Mul,
    Concat,
}

fn permute<'a, T: Clone + Debug>(values: &'a [T], n: usize) -> impl Iterator<Item = Vec<T>> + 'a {
    let mut iters = (0..n)
        .map(move |i| {
            (0..values.len())
                .map(move |v| repeat_n(values[v].clone(), values.len().pow(i as u32)))
                .flatten()
                .cycle()
        })
        .collect::<Vec<_>>();

    (0..values.len().pow(n as u32))
        .map(move |_| iters.iter_mut().map(|it| it.next().unwrap()).collect())
}

fn eval_ops(nums: &[i64], ops: &[Op]) -> i64 {
    nums[1..]
        .iter()
        .zip(ops)
        .fold(nums[0], |acc, (n, op)| match op {
            Op::Add => acc + n,
            Op::Mul => acc * n,
            Op::Concat => (acc * 10_i64.pow(n.ilog10() + 1)) + n,
        })
}

#[aoc(day7, part1)]
pub fn solve_part1(input: &[(i64, Vec<i64>)]) -> i64 {
    input
        .iter()
        .filter(|(sum, nums)| {
            permute(&[Op::Add, Op::Mul], nums.len() - 1)
                .find(|ops| eval_ops(nums, &ops) == *sum)
                .is_some()
        })
        .map(|(sum, _)| sum)
        .sum()
}

#[aoc(day7, part2)]
pub fn solve_part2(input: &[(i64, Vec<i64>)]) -> i64 {
    input
        .iter()
        .filter(|(sum, nums)| {
            permute(&[Op::Add, Op::Mul, Op::Concat], nums.len() - 1)
                .find(|ops| eval_ops(nums, &ops) == *sum)
                .is_some()
        })
        .map(|(sum, _)| sum)
        .sum()
}
