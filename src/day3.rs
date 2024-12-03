use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day3)]
pub fn input_gen(input: &str) -> String {
    input.to_string()
}

fn peek_until<'a>(input: &'a str, token: &str) -> Result<&'a str, &'a str> {
    input
        .find(token)
        .map(|offset| &input[offset..])
        .ok_or(&input[input.len()..])
}

fn peek_until_first<'a>(input: &'a str, tokens: &[&str]) -> Result<(&'a str, usize), &'a str> {
    tokens
        .iter()
        .enumerate()
        .filter_map(|(idx, token)| input.find(token).map(|offset| (idx, offset)))
        .min_by_key(|(_, offset)| *offset)
        .map(|(idx, offset)| (&input[offset..], idx))
        .ok_or(&input[input.len()..])
}

fn consume<'a>(input: &'a str, token: &str) -> Result<&'a str, &'a str> {
    if input.starts_with(token) {
        Ok(&input[token.len()..])
    } else {
        Err(input)
    }
}

fn parse_number(input: &str) -> Result<(&str, i64), &str> {
    let digits = input
        .chars()
        .scan((), |_, c| if c.is_numeric() { Some(()) } else { None })
        .count();

    if digits > 0 {
        let n = &input[0..digits].parse::<i64>().unwrap();

        Ok((&input[digits..], *n))
    } else {
        Err(input)
    }
}

fn parse_mul_op(input: &str) -> Result<(&str, (i64, i64)), &str> {
    let input = consume(input, "mul(")?;
    let (input, a) = parse_number(input)?;
    let input = consume(input, ",")?;
    let (input, b) = parse_number(input)?;
    let input = consume(input, ")")?;

    Ok((input, (a, b)))
}

#[aoc(day3, part1)]
pub fn solve_part1(mut input: &str) -> i64 {
    let mut acc = 0;

    while let Ok(rest) = peek_until(input, "mul(") {
        match parse_mul_op(rest) {
            Ok((rest, (a, b))) => {
                input = rest;
                acc += a * b;
            }
            Err(rest) => input = rest,
        }
    }

    acc
}

#[aoc(day3, part2)]
pub fn solve_part2(mut input: &str) -> i64 {
    let mut acc = 0;
    let mut enabled = true;

    while let Ok((rest, idx)) = peek_until_first(input, &["mul(", "do()", "don't()"]) {
        match idx {
            0 => match parse_mul_op(rest) {
                Ok((rest, (a, b))) => {
                    input = rest;

                    if enabled {
                        acc += a * b;
                    }
                }
                Err(rest) => input = rest,
            },
            1 => {
                enabled = true;
                input = consume(rest, "do()").unwrap();
            }
            2 => {
                enabled = false;
                input = consume(rest, "don't()").unwrap();
            }
            _ => unreachable!(),
        }
    }

    acc
}
