use std::cmp;
use std::{io, io::prelude::*};

fn main() {
    let mut max_calories = 0;
    let mut group_total = 0;
    for line in io::stdin().lock().lines() {
        match line.unwrap().parse::<i32>() {
            Ok(n) => group_total += n,
            Err(_) => {
                max_calories = cmp::max(max_calories, group_total);
                group_total = 0;
            }
        }
    }
    max_calories = cmp::max(max_calories, group_total);
    println!("{}", max_calories);
}
