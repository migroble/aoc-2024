use std::collections::BTreeMap;

use aoc_runner_derive::{aoc, aoc_generator};

type Ruleset = BTreeMap<i64, Vec<i64>>;

pub struct Input {
    rules: Ruleset,
    updates: Vec<Vec<i64>>,
}

#[aoc_generator(day5)]
pub fn input_gen(input: &str) -> Input {
    let mut parts = input.split("\n\n");
    let rules_str = parts.next().unwrap();
    let updates_str = parts.next().unwrap();

    let mut rules = Ruleset::new();
    rules_str
        .lines()
        .map(|l| {
            let mut parts = l.split('|').map(|d| d.parse::<i64>().unwrap());
            (parts.next().unwrap(), parts.next().unwrap())
        })
        .for_each(|(a, b)| {
            rules.entry(a).or_insert_with(Vec::new).push(b);
        });

    let updates = updates_str
        .lines()
        .map(|l| {
            l.split(',')
                .map(|d| d.parse::<i64>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect();

    Input { rules, updates }
}

fn check_rules(rules: &Ruleset, update: &[i64]) -> Option<(usize, usize)> {
    rules
        .iter()
        .filter_map(|(target, pages)| {
            update
                .iter()
                .enumerate()
                .find(|(_, v)| *v == target)
                .map(|(idx, _)| (idx, pages))
        })
        .map(|(target_idx, pages)| {
            pages.iter().filter_map(move |n| {
                update[..target_idx]
                    .iter()
                    .enumerate()
                    .rfind(|(_, m)| n == *m)
                    .map(|(idx, _)| (target_idx, idx))
            })
        })
        .flatten()
        .next()
}

#[aoc(day5, part1)]
pub fn solve_part1(input: &Input) -> i64 {
    input
        .updates
        .iter()
        .filter(|u| check_rules(&input.rules, u).is_none())
        .map(|u| u[u.len() / 2])
        .sum()
}

#[aoc(day5, part2)]
pub fn solve_part2(input: &Input) -> i64 {
    input
        .updates
        .clone()
        .iter_mut()
        .filter(|u| !check_rules(&input.rules, u).is_none())
        .map(|u| {
            while let Some((a, b)) = check_rules(&input.rules, u) {
                u.swap(a, b);
            }

            u
        })
        .map(|u| u[u.len() / 2])
        .sum()
}
