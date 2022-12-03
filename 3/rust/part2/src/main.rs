use std::{io, io::prelude::*};

fn char_to_offset(b: &u8) -> u8 {
    if *b <= 'Z' as u8 {
        b - ('A' as u8) + 26
    } else {
        b - ('a' as u8)
    }
}

fn char_to_u64(b: &u8) -> u64 {
    1 << char_to_offset(b)
}

fn string_to_item_map(s: &String) -> u64 {
    s.as_bytes()
        .iter()
        .map(char_to_u64)
        .reduce(|acc, n| acc | n)
        .unwrap()
}

fn score_from_set_bit(n: u64) -> u32 {
    for i in 0..64 {
        if ((n >> i) & 1) > 0 {
            return i + 1;
        }
    }
    0
}

fn chunk_to_points(lines: &[String]) -> u32 {
    score_from_set_bit(
        lines
            .iter()
            .map(string_to_item_map)
            .reduce(|acc, item_map| acc & item_map)
            .unwrap(),
    )
}

fn main() {
    let result = io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<String>>()
        .chunks(3)
        .map(chunk_to_points)
        .sum::<u32>();
    println!("{}", result);
}
