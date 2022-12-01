use std::cmp;
use std::{io, io::prelude::*};

fn line_to_option(line: String) -> Option<i32> {
    if line.is_empty() {
        None
    } else {
        Some(line.parse::<i32>().unwrap())
    }
}

fn main() {
    let mut max_calories = 0;
    let mut group_total = 0;
    for line in io::stdin().lock().lines() {
        match line_to_option(line.unwrap()) {
            Some(n) => group_total += n,
            None => {
                max_calories = cmp::max(max_calories, group_total);
                group_total = 0;
            }
        }
    }
    max_calories = cmp::max(max_calories, group_total);
    println!("{}", max_calories);
}
