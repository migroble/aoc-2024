use std::collections::{HashMap, HashSet};

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Position(i64, i64);

impl Position {
    fn step(&self, dir: Direction) -> Position {
        match dir {
            Direction::Up => Position(self.0, self.1 - 1),
            Direction::Right => Position(self.0 + 1, self.1),
            Direction::Down => Position(self.0, self.1 + 1),
            Direction::Left => Position(self.0 - 1, self.1),
        }
    }

    fn is_within(&self, bounds: (i64, i64)) -> bool {
        self.0 >= 0 && self.1 >= 0 && self.0 < bounds.0 && self.1 < bounds.1
    }
}

#[derive(Clone, Copy, PartialEq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn turn_right(self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

#[derive(Clone)]
struct Guard {
    pos: Position,
    dir: Direction,
}

#[derive(Clone)]
pub struct World {
    map: HashSet<Position>,
    bounds: (i64, i64),
    visited: HashMap<Position, Vec<Direction>>,
    guard: Guard,
    looping: bool,
}

impl World {
    fn step(&mut self) -> bool {
        let new_pos = self.guard.pos.step(self.guard.dir);
        let is_in_map = new_pos.is_within(self.bounds);

        if is_in_map {
            if self.map.contains(&new_pos) {
                self.guard.dir = self.guard.dir.turn_right();
            } else {
                self.guard.pos = new_pos;

                self.looping = self
                    .visited
                    .get(&self.guard.pos)
                    .map(|v| v.contains(&self.guard.dir))
                    .unwrap_or(false);

                self.visited
                    .entry(self.guard.pos)
                    .or_insert_with(Vec::new)
                    .push(self.guard.dir);
            }
        }

        is_in_map && !self.looping
    }

    fn simulate(&mut self) {
        while self.step() {}
    }

    fn visited_count(&self) -> usize {
        self.visited.len()
    }

    fn visited_positions(&self) -> impl IntoIterator<Item = Position> {
        self.visited.keys().copied().collect::<Vec<_>>()
    }

    fn is_looping(&self) -> bool {
        self.looping
    }
}

#[aoc_generator(day6)]
pub fn input_gen(input: &str) -> World {
    let mut it = input
        .lines()
        .enumerate()
        .map(|(y, l)| {
            l.chars()
                .enumerate()
                .map(move |(x, c)| (Position(x as i64, y as i64), c))
        })
        .flatten();

    let map = it
        .clone()
        .filter(|(_, c)| *c == '#')
        .map(|(p, _)| p)
        .collect();

    let guard = it
        .find(|(_, c)| *c == '^')
        .map(|(pos, _)| Guard {
            pos,
            dir: Direction::Up,
        })
        .unwrap();

    let height = input.lines().count() as i64;
    let width = input.lines().next().unwrap().chars().count() as i64;

    let mut visited = HashMap::new();
    visited.insert(guard.pos, vec![guard.dir]);

    World {
        map,
        bounds: (width, height),
        visited,
        guard,
        looping: false,
    }
}

#[aoc(day6, part1)]
pub fn solve_part1(input: &World) -> usize {
    let mut world = input.clone();

    world.simulate();

    world.visited_count()
}

#[aoc(day6, part2)]
pub fn solve_part2(input: &World) -> usize {
    let initial_position = input.visited.keys().next().unwrap();

    let mut world = input.clone();

    world.simulate();

    let positions = world.visited_positions();

    positions
        .into_iter()
        .filter(|p| *p != *initial_position)
        .filter(|p| {
            let mut world = input.clone();

            world.map.insert(*p);
            world.simulate();
            world.is_looping()
        })
        .count()
}
