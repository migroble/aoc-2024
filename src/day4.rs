use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day4)]
pub fn input_gen(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect()
}

const DIRECTIONS: [(i64, i64); 8] = [
    (0, 1),
    (0, -1),
    (1, 0),
    (-1, 0),
    (1, 1),
    (1, -1),
    (-1, 1),
    (-1, -1),
];

#[aoc(day4, part1)]
pub fn solve_part1(input: &[Vec<char>]) -> usize {
    const WORD: &str = "XMAS";

    input
        .iter()
        .enumerate()
        .map(|(y, v)| v.iter().enumerate().map(move |(x, c)| ((x, y), *c)))
        .flatten()
        .filter(|(_, c)| *c == WORD.chars().next().unwrap())
        .map(|(p, _)| DIRECTIONS.iter().map(move |d| (p, *d)))
        .flatten()
        .filter(|((x, y), (dx, dy))| {
            WORD.chars().skip(1).eq((1..WORD.len()).scan((), |_, i| {
                let x1 = *x as i64 + i as i64 * dx;
                let y1 = *y as i64 + i as i64 * dy;

                if x1 >= 0
                    && y1 >= 0
                    && (y1 as usize) < input.len()
                    && (x1 as usize) < input[y1 as usize].len()
                {
                    Some(input[y1 as usize][x1 as usize])
                } else {
                    None
                }
            }))
        })
        .count()
}

#[aoc(day4, part2)]
pub fn solve_part2(input: &[Vec<char>]) -> usize {
    input
        .iter()
        .enumerate()
        .map(|(y, v)| v.iter().enumerate().map(move |(x, c)| ((x, y), *c)))
        .flatten()
        .filter(|(_, c)| *c == 'A')
        .map(|(p, _)| p)
        .filter(|(x, y)| {
            *x as i64 - 1 >= 0
                && *y as i64 - 1 >= 0
                && *y + 1 < input.len()
                && *x + 1 < input[*y].len()
        })
        .filter(|(x, y)| {
            input[*y - 1][*x - 1] == 'M'
                && input[*y - 1][*x + 1] == 'S'
                && input[*y + 1][*x - 1] == 'M'
                && input[*y + 1][*x + 1] == 'S'
                || input[*y - 1][*x - 1] == 'S'
                    && input[*y - 1][*x + 1] == 'S'
                    && input[*y + 1][*x - 1] == 'M'
                    && input[*y + 1][*x + 1] == 'M'
                || input[*y - 1][*x - 1] == 'S'
                    && input[*y - 1][*x + 1] == 'M'
                    && input[*y + 1][*x - 1] == 'S'
                    && input[*y + 1][*x + 1] == 'M'
                || input[*y - 1][*x - 1] == 'M'
                    && input[*y - 1][*x + 1] == 'M'
                    && input[*y + 1][*x - 1] == 'S'
                    && input[*y + 1][*x + 1] == 'S'
        })
        .count()
}
