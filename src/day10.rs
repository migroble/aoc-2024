use std::collections::HashSet;

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day10)]
pub fn input_gen(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect()
}

struct Trail {
    endings: Vec<(usize, usize)>,
}

impl Trail {
    fn from_map(map: &[Vec<u32>], coords: (usize, usize)) -> Trail {
        let height = map[coords.1][coords.0];
        let endings = if height < 9 {
            [(1, 0), (0, 1), (-1, 0), (0, -1)]
                .into_iter()
                .filter(|offset| coords.0 as i64 >= -offset.0 && coords.1 as i64 >= -offset.1)
                .map(|offset| {
                    (
                        (coords.0 as i64 + offset.0) as usize,
                        (coords.1 as i64 + offset.1) as usize,
                    )
                })
                .filter(|coords| {
                    coords.1 < map.len()
                        && coords.0 < map[coords.1].len()
                        && map[coords.1][coords.0] == height + 1
                })
                .map(|coords| Trail::from_map(map, coords))
                .map(|c| c.endings.into_iter())
                .flatten()
                .collect()
        } else {
            vec![coords]
        };

        Trail { endings }
    }
}

#[aoc(day10, part1)]
pub fn solve_part1(input: &[Vec<u32>]) -> usize {
    input
        .iter()
        .enumerate()
        .map(|(y, l)| {
            l.iter()
                .enumerate()
                .filter(|(_, h)| **h == 0)
                .map(move |(x, _)| {
                    Trail::from_map(input, (x, y))
                        .endings
                        .into_iter()
                        .collect::<HashSet<_>>()
                        .len()
                })
        })
        .flatten()
        .sum()
}

#[aoc(day10, part2)]
pub fn solve_part2(input: &[Vec<u32>]) -> usize {
    input
        .iter()
        .enumerate()
        .map(|(y, l)| {
            l.iter()
                .enumerate()
                .filter(|(_, h)| **h == 0)
                .map(move |(x, _)| Trail::from_map(input, (x, y)).endings.len())
        })
        .flatten()
        .sum()
}
