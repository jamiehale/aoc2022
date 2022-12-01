use std::{io, io::prelude::*};

fn line_to_option(line: String) -> Option<i32> {
    if line.is_empty() {
        None
    } else {
        Some(line.parse::<i32>().unwrap())
    }
}

fn add_and_update(n: i32, totals: &mut Vec<i32>) {
    totals.push(n);
    totals.sort_unstable_by(|a, b| b.cmp(a));
    totals.truncate(3);
}

fn main() {
    let mut max_calories = vec![0, 0, 0];
    let mut group_total = 0;
    for line in io::stdin().lock().lines() {
        match line_to_option(line.unwrap()) {
            Some(n) => group_total += n,
            None => {
                add_and_update(group_total, &mut max_calories);
                group_total = 0
            }
        }
    }
    add_and_update(group_total, &mut max_calories);
    println!("{}", max_calories.iter().sum::<i32>());
}
