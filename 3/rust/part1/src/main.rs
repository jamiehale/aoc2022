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

fn string_to_item_map(s: &str) -> u64 {
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

fn to_points(line: String) -> u32 {
    let (comp1, comp2) = line.split_at(line.len() / 2);
    let comp1_items = string_to_item_map(comp1);
    let comp2_items = string_to_item_map(comp2);
    score_from_set_bit(comp1_items & comp2_items)
}

fn main() {
    let result = io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap())
        .map(to_points)
        .sum::<u32>();
    println!("{}", result);
}
