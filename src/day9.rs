use std::{collections::HashSet, iter::repeat_n};

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day9)]
pub fn input_gen(input: &str) -> Vec<u32> {
    input.chars().map(|c| c.to_digit(10).unwrap()).collect()
}

fn expand_disk_map(disk_map: &[u32]) -> Vec<(Option<usize>, u32)> {
    disk_map
        .iter()
        .enumerate()
        .map(|(idx, size)| {
            if idx % 2 == 0 {
                (Some(idx / 2), *size)
            } else {
                (None, *size)
            }
        })
        .collect()
}

fn compact_disk_blocks(mut disk: Vec<(Option<usize>, u32)>) -> Vec<(Option<usize>, u32)> {
    loop {
        let Some((src_idx, src_size)) =
            disk.iter().enumerate().rev().find_map(|(idx, (id, size))| {
                if id.is_some() && *size > 0 {
                    Some((idx, *size))
                } else {
                    None
                }
            })
        else {
            break disk;
        };
        let Some((dst_idx, dst_size)) = disk.iter().enumerate().find_map(|(idx, (id, size))| {
            if id.is_none() && *size > 0 {
                Some((idx, *size))
            } else {
                None
            }
        }) else {
            break disk;
        };

        if src_size >= dst_size {
            disk[dst_idx].0 = disk[src_idx].0;
            disk[src_idx].1 -= dst_size;
        } else {
            disk[dst_idx].1 -= src_size;
            disk[src_idx].1 = 0;
            disk.insert(dst_idx, (disk[src_idx].0, src_size));
        }
    }
    .into_iter()
    .filter(|(id, size)| id.is_some() && *size > 0)
    .collect()
}

#[aoc(day9, part1)]
pub fn solve_part1(input: &[u32]) -> usize {
    let disk = expand_disk_map(input);
    let disk = compact_disk_blocks(disk);
    disk.iter()
        .map(|(id, size)| repeat_n(id, *size as usize))
        .flatten()
        .enumerate()
        .filter_map(|(idx, id)| id.map(|id| (idx, id)))
        .map(|(idx, id)| idx * id)
        .sum()
}

fn compact_disk_spaces(mut disk: Vec<(Option<usize>, u32)>) -> Vec<(Option<usize>, u32)> {
    let mut i = 0;

    while i + 1 < disk.len() {
        if disk[i].0.is_none() && disk[i + 1].0.is_none() {
            disk[i].1 += disk[i + 1].1;
            disk.remove(i + 1);
        } else {
            i += 1;
        }
    }

    disk.into_iter().filter(|(_, size)| *size > 0).collect()
}

fn compact_disk_files(mut disk: Vec<(Option<usize>, u32)>) -> Vec<(Option<usize>, u32)> {
    let mut checked = HashSet::new();
    loop {
        let checked2 = checked.clone();
        let Some((src_idx, src_size, dst_idx, _dst_size)) = disk
            .iter()
            .enumerate()
            .rev()
            .filter_map(|(idx, (id, size))| id.map(|id| (idx, (id, *size))))
            .filter(|(_, (id, _))| !checked2.contains(id))
            .filter(|(_, (_, size))| *size > 0)
            .map(|(b_idx, (id, b_size))| {
                checked.insert(id);
                disk.iter()
                    .enumerate()
                    .find(|(_, (id, s_size))| id.is_none() && *s_size >= b_size)
                    .map(|(s_idx, (_, s_size))| (b_idx, b_size, s_idx, *s_size))
            })
            .find(|data| data.is_some())
            .flatten()
        else {
            break disk;
        };

        if dst_idx < src_idx {
            let id = disk[src_idx].0;
            disk[src_idx].0 = None;

            disk[dst_idx].1 -= src_size;
            disk.insert(dst_idx, (id, src_size));

            disk = compact_disk_spaces(disk);
        }
    }
    .into_iter()
    .filter(|(_, size)| *size > 0)
    .collect()
}

#[aoc(day9, part2)]
pub fn solve_part2(input: &[u32]) -> usize {
    let disk = expand_disk_map(input);
    let disk = compact_disk_files(disk);
    disk.iter()
        .map(|(id, size)| repeat_n(id, *size as usize))
        .flatten()
        .enumerate()
        .filter_map(|(idx, id)| id.map(|id| (idx, id)))
        .map(|(idx, id)| idx * id)
        .sum()
}
