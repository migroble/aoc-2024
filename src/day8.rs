use std::{
    collections::{HashMap, HashSet},
    ops::{Add, Sub},
};

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position(i64, i64);

#[derive(Clone, Copy)]
pub struct Offset(i64, i64);

impl Add<Offset> for Position {
    type Output = Position;

    fn add(self, rhs: Offset) -> Self::Output {
        Position(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Sub<Offset> for Position {
    type Output = Position;

    fn sub(self, rhs: Offset) -> Self::Output {
        Position(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl Sub<Position> for Position {
    type Output = Offset;

    fn sub(self, rhs: Position) -> Self::Output {
        Offset(self.0 - rhs.0, self.1 - rhs.1)
    }
}

pub struct World {
    map: HashMap<char, Vec<Position>>,
    bounds: (i64, i64),
}

#[aoc_generator(day8)]
pub fn input_gen(input: &str) -> World {
    let mut map = HashMap::new();

    let height = input.lines().count() as i64;
    let width = input.lines().next().unwrap().chars().count() as i64;

    input
        .lines()
        .enumerate()
        .map(|(y, l)| {
            l.chars()
                .enumerate()
                .map(move |(x, c)| (Position(x as i64, y as i64), c))
        })
        .flatten()
        .filter(|(_, c)| *c != '.')
        .for_each(|(p, c)| map.entry(c).or_insert_with(Vec::new).push(p));

    World {
        map,
        bounds: (width, height),
    }
}

#[aoc(day8, part1)]
pub fn solve_part1(input: &World) -> usize {
    input
        .map
        .iter()
        .map(|(_, antennae)| {
            antennae
                .iter()
                .map(|antenna1| {
                    antennae
                        .iter()
                        .filter(move |antenna2| antenna1 != *antenna2)
                        .map(move |antenna2| (*antenna1, *antenna2))
                })
                .flatten()
                .map(|(a1, a2)| {
                    let offset = a2 - a1;
                    [a1 - offset, a2 + offset].into_iter()
                })
                .flatten()
                .filter(|a| a.0 >= 0 && a.1 >= 0 && a.0 < input.bounds.0 && a.1 < input.bounds.1)
        })
        .flatten()
        .collect::<HashSet<_>>()
        .len()
}

#[aoc(day8, part2)]
pub fn solve_part2(input: &World) -> usize {
    input
        .map
        .iter()
        .map(|(_, antennae)| {
            antennae
                .iter()
                .map(|antenna1| {
                    antennae
                        .iter()
                        .filter(move |antenna2| antenna1 != *antenna2)
                        .map(move |antenna2| (*antenna1, *antenna2))
                })
                .flatten()
                .map(|(a1, a2)| {
                    let offset = a2 - a1;

                    (0..)
                        .scan(a1, move |a, _| {
                            if a.0 >= 0 && a.1 >= 0 && a.0 < input.bounds.0 && a.1 < input.bounds.1
                            {
                                let p = *a;
                                *a = *a - offset;
                                Some(p)
                            } else {
                                None
                            }
                        })
                        .chain((0..).scan(a2, move |a, _| {
                            *a = *a + offset;

                            if a.0 >= 0 && a.1 >= 0 && a.0 < input.bounds.0 && a.1 < input.bounds.1
                            {
                                Some(*a)
                            } else {
                                None
                            }
                        }))
                })
                .flatten()
        })
        .flatten()
        .collect::<HashSet<_>>()
        .len()
}
