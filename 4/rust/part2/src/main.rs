use std::{io, io::prelude::*};

fn to_range_pair(s: &str) -> (u32, u32) {
    let q = s
        .split('-')
        .map(|n| n.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();
    (q[0], q[1])
}

fn to_range_pairs(s: String) -> ((u32, u32), (u32, u32)) {
    let elves = s.split(',').map(to_range_pair).collect::<Vec<(u32, u32)>>();
    (elves[0], elves[1])
}

fn to_points(elf_ranges: ((u32, u32), (u32, u32))) -> u32 {
    if elf_ranges.0 .0 < elf_ranges.1 .0 {
        if elf_ranges.0 .1 >= elf_ranges.1 .0 {
            1
        } else {
            0
        }
    } else {
        if elf_ranges.1 .1 >= elf_ranges.0 .0 {
            1
        } else {
            0
        }
    }
}

fn main() {
    let result = io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap())
        .map(to_range_pairs)
        .map(to_points)
        .sum::<u32>();
    println!("{}", result);
}
