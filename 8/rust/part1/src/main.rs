use std::{io, io::prelude::*};

fn mark_visible(row: usize, column: usize, visible: &mut Vec<Vec<bool>>) {
    visible[row][column] = true;
}

fn count_visible(visible: &Vec<Vec<bool>>) -> u32 {
    visible.iter().fold(0, |acc, v| {
        v.iter()
            .fold(acc, |inner_acc, b| inner_acc + if *b { 1 } else { 0 })
    })
}

fn main() {
    let heights = io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap())
        .map(|line| {
            line.as_bytes()
                .iter()
                .map(|b| *b as i32 - '0' as i32)
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();
    let rows = heights.len();
    let columns = heights[0].len();
    let mut visible = vec![vec![false; columns]; rows];

    for i in 0..columns {
        mark_visible(0, i, &mut visible);
        mark_visible(rows - 1, i, &mut visible);
    }
    for i in 1..(rows - 1) {
        mark_visible(i, 0, &mut visible);
        mark_visible(i, columns - 1, &mut visible)
    }

    for i in 0..rows {
        let mut running_max = -1;
        for j in 0..columns {
            process_location(i, j, &mut visible, &heights, &mut running_max);
        }
        running_max = -1;
        for j in (0..columns).rev() {
            process_location(i, j, &mut visible, &heights, &mut running_max);
        }
    }
    for j in 0..columns {
        let mut running_max = -1;
        for i in 0..rows {
            process_location(i, j, &mut visible, &heights, &mut running_max);
        }
        running_max = -1;
        for i in (0..rows).rev() {
            process_location(i, j, &mut visible, &heights, &mut running_max);
        }
    }

    println!("{}", count_visible(&visible));
}

fn process_location(
    i: usize,
    j: usize,
    visible: &mut Vec<Vec<bool>>,
    heights: &Vec<Vec<i32>>,
    running_max: &mut i32,
) {
    if heights[i][j] > *running_max {
        mark_visible(i, j, visible);
        *running_max = heights[i][j];
    }
}
