use std::{io, io::prelude::*};

fn visible_in_column(
    column: usize,
    rows: &mut dyn Iterator<Item = usize>,
    heights: &Vec<Vec<u32>>,
    height: u32,
) -> u32 {
    let mut count = 0;
    for row in rows {
        count += 1;
        if heights[row][column] >= height {
            break;
        }
    }
    count
}

fn visible_in_row(
    row: usize,
    columns: &mut dyn Iterator<Item = usize>,
    heights: &Vec<Vec<u32>>,
    height: u32,
) -> u32 {
    let mut count = 0;
    for column in columns {
        count += 1;
        if heights[row][column] >= height {
            break;
        }
    }
    count
}

fn main() {
    let heights = io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap())
        .map(|line| {
            line.as_bytes()
                .iter()
                .map(|b| *b as u32 - '0' as u32)
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();
    let rows = heights.len();
    let columns = heights[0].len();
    let mut top_score = 0;

    for row in 0..rows {
        for column in 0..columns {
            let height = heights[row][column];
            let up_count = visible_in_column(column, &mut (0..row).rev(), &heights, height);
            let down_count = visible_in_column(column, &mut ((row + 1)..rows), &heights, height);
            let left_count = visible_in_row(row, &mut (0..column).rev(), &heights, height);
            let right_count = visible_in_row(row, &mut ((column + 1)..columns), &heights, height);
            // println!(
            //     "({: >2},{: >2}) {} {} {} {}",
            //     row, column, up_count, down_count, left_count, right_count
            // );
            let score = up_count * down_count * left_count * right_count;
            if score > top_score {
                top_score = score;
            }
        }
    }
    println!("{}", top_score);
}
