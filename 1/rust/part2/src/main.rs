use std::{io, io::prelude::*};

fn main() {
    let mut max_calories = vec![0, 0, 0];
    let mut group_total = 0;
    for line in io::stdin().lock().lines() {
        match line.unwrap().parse::<i32>() {
            Ok(n) => group_total += n,
            Err(_) => {
                max_calories.push(group_total);
                max_calories.sort_unstable_by(|a, b| b.cmp(a));
                max_calories.truncate(3);
                group_total = 0;
            }
        }
    }
    max_calories.push(group_total);
    max_calories.sort_unstable_by(|a, b| b.cmp(a));
    max_calories.truncate(3);
    println!("{}", max_calories.iter().sum::<i32>());
}
