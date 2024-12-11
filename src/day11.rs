use std::fmt::Debug;

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day11)]
pub fn input_gen(input: &str) -> Vec<usize> {
    input
        .split_whitespace()
        .map(|n| n.parse::<usize>().unwrap())
        .collect()
}

struct StoneIterator {
    stones: Vec<usize>,
}

impl StoneIterator {
    fn new(stones: Vec<usize>) -> Self {
        Self { stones }
    }
}

impl Iterator for StoneIterator {
    type Item = Vec<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        self.stones = self
            .stones
            .iter()
            .map(|s| {
                if *s == 0 {
                    vec![1]
                } else {
                    let digits = s.ilog10() + 1;

                    if digits % 2 == 0 {
                        let magnitude = 10_usize.pow(digits / 2);
                        vec![s / magnitude, s % magnitude]
                    } else {
                        vec![s * 2024]
                    }
                }
                .into_iter()
            })
            .flatten()
            .collect();

        Some(self.stones.clone())
    }
}

#[aoc(day11, part1)]
pub fn solve_part1(input: &[usize]) -> usize {
    StoneIterator::new(input.to_vec()).nth(24).unwrap().len()
}

#[derive(Debug, Clone, Copy)]
enum Effect {
    Spawn { value: usize, count: usize },
    Increment { amount: usize },
}

struct State {
    next: Option<Box<State>>,
    effects: Vec<Effect>,
    count: usize,
}

impl Debug for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut counts = vec![self.count];

        let mut root = self;
        while let Some(ref next) = root.next {
            counts.push(next.count);
            root = &*next;
        }

        write!(f, "State {{ {counts:?} }}")
    }
}

impl State {
    fn new(effects: &[Vec<Effect>]) -> Self {
        Self {
            next: effects.get(1).map(|_| Box::new(State::new(&effects[1..]))),
            effects: effects[0].clone(),
            count: 0,
        }
    }

    fn increment(&mut self, count: usize) {
        self.count += count;
    }

    fn step(&mut self) -> Vec<Effect> {
        let mut effects = if self.count > 0 {
            self.effects
                .iter()
                .copied()
                .map(|e| match e {
                    Effect::Spawn { value, count } => Effect::Spawn {
                        value,
                        count: count * self.count,
                    },
                    Effect::Increment { amount } => Effect::Increment {
                        amount: self.count * amount,
                    },
                })
                .collect::<Vec<Effect>>()
        } else {
            vec![]
        };

        if let Some(ref mut state) = self.next {
            effects.extend(state.step().into_iter());

            state.count += self.count;
        }

        self.count = 0;

        effects
    }
}

#[derive(Debug)]
struct StoneCycler {
    count: usize,
    states: [State; 10],
}

impl StoneCycler {
    fn new() -> Self {
        Self {
            count: 0,
            states: [
                State::new(&[vec![Effect::Spawn { value: 1, count: 1 }]]),
                State::new(&[
                    vec![],
                    vec![Effect::Increment { amount: 1 }],
                    vec![
                        Effect::Increment { amount: 2 },
                        Effect::Spawn { value: 2, count: 2 },
                        Effect::Spawn { value: 0, count: 1 },
                        Effect::Spawn { value: 4, count: 1 },
                    ],
                ]),
                State::new(&[
                    vec![],
                    vec![Effect::Increment { amount: 1 }],
                    vec![
                        Effect::Increment { amount: 2 },
                        Effect::Spawn { value: 4, count: 2 },
                        Effect::Spawn { value: 0, count: 1 },
                        Effect::Spawn { value: 8, count: 1 },
                    ],
                ]),
                State::new(&[
                    vec![],
                    vec![Effect::Increment { amount: 1 }],
                    vec![
                        Effect::Increment { amount: 2 },
                        Effect::Spawn { value: 6, count: 1 },
                        Effect::Spawn { value: 0, count: 1 },
                        Effect::Spawn { value: 7, count: 1 },
                        Effect::Spawn { value: 2, count: 1 },
                    ],
                ]),
                State::new(&[
                    vec![],
                    vec![Effect::Increment { amount: 1 }],
                    vec![
                        Effect::Increment { amount: 2 },
                        Effect::Spawn { value: 8, count: 1 },
                        Effect::Spawn { value: 0, count: 1 },
                        Effect::Spawn { value: 9, count: 1 },
                        Effect::Spawn { value: 6, count: 1 },
                    ],
                ]),
                State::new(&[
                    vec![],
                    vec![],
                    vec![Effect::Increment { amount: 1 }],
                    vec![Effect::Increment { amount: 2 }],
                    vec![
                        Effect::Increment { amount: 4 },
                        Effect::Spawn { value: 2, count: 2 },
                        Effect::Spawn { value: 0, count: 2 },
                        Effect::Spawn { value: 4, count: 1 },
                        Effect::Spawn { value: 8, count: 3 },
                    ],
                ]),
                State::new(&[
                    vec![],
                    vec![],
                    vec![Effect::Increment { amount: 1 }],
                    vec![Effect::Increment { amount: 2 }],
                    vec![
                        Effect::Increment { amount: 4 },
                        Effect::Spawn { value: 2, count: 1 },
                        Effect::Spawn { value: 4, count: 2 },
                        Effect::Spawn { value: 5, count: 2 },
                        Effect::Spawn { value: 7, count: 1 },
                        Effect::Spawn { value: 9, count: 1 },
                        Effect::Spawn { value: 6, count: 1 },
                    ],
                ]),
                State::new(&[
                    vec![],
                    vec![],
                    vec![Effect::Increment { amount: 1 }],
                    vec![Effect::Increment { amount: 2 }],
                    vec![
                        Effect::Increment { amount: 4 },
                        Effect::Spawn { value: 2, count: 2 },
                        Effect::Spawn { value: 8, count: 1 },
                        Effect::Spawn { value: 6, count: 2 },
                        Effect::Spawn { value: 7, count: 1 },
                        Effect::Spawn { value: 0, count: 1 },
                        Effect::Spawn { value: 3, count: 1 },
                    ],
                ]),
                State::new(&[
                    vec![],
                    vec![],
                    vec![Effect::Increment { amount: 1 }],
                    vec![
                        Effect::Increment { amount: 2 },
                        Effect::Spawn { value: 8, count: 1 },
                    ],
                    vec![
                        Effect::Increment { amount: 3 },
                        Effect::Spawn { value: 3, count: 1 },
                        Effect::Spawn { value: 2, count: 2 },
                        Effect::Spawn { value: 7, count: 2 },
                        Effect::Spawn { value: 6, count: 1 },
                    ],
                ]),
                State::new(&[
                    vec![],
                    vec![],
                    vec![Effect::Increment { amount: 1 }],
                    vec![Effect::Increment { amount: 2 }],
                    vec![
                        Effect::Increment { amount: 4 },
                        Effect::Spawn { value: 3, count: 1 },
                        Effect::Spawn { value: 6, count: 2 },
                        Effect::Spawn { value: 8, count: 2 },
                        Effect::Spawn { value: 9, count: 1 },
                        Effect::Spawn { value: 1, count: 1 },
                        Effect::Spawn { value: 4, count: 1 },
                    ],
                ]),
            ],
        }
    }

    fn add(&mut self, n: usize) {
        assert!(n < 10);
        self.states[n].count += 1;
        self.count += 1;
    }

    fn step(&mut self) {
        let effects = self
            .states
            .iter_mut()
            .map(|s| s.step())
            .flatten()
            .collect::<Vec<Effect>>();

        effects.into_iter().for_each(|e| match e {
            Effect::Spawn { value, count } => self.states[value].increment(count),
            Effect::Increment { amount } => self.count += amount,
        });
    }

    fn count(&self) -> usize {
        self.count
    }
}

#[aoc(day11, part2)]
pub fn solve_part2(input: &[usize]) -> usize {
    let mut stones = input.to_vec();
    let mut cycler = StoneCycler::new();

    (0..75).for_each(|_| {
        stones = StoneIterator::new(stones.clone()).next().unwrap();
        cycler.step();

        stones
            .iter()
            .filter(|&&s| s < 10)
            .for_each(|&s| cycler.add(s));
        stones.retain(|&s| s >= 10);
    });

    stones.len() + cycler.count()
}
