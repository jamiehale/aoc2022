use std::{io, io::prelude::*};

// A rock
// B paper
// C scissors
// X lose
// Y tie
// Z win

// rock 1
// paper 2
// scissors 3

// win 6
// draw 3
// lose 0

fn to_points(line: String) -> i32 {
    match line.as_str() {
        "A X" => 3, // lose + scissors
        "A Y" => 4, // tie + rock
        "A Z" => 8, // win + paper
        "B X" => 1, // lose + rock
        "B Y" => 5, // tie + paper
        "B Z" => 9, // win + scissors
        "C X" => 2, // lose + paper
        "C Y" => 6, // tie + scissors
        "C Z" => 7, // win + rock
        _ => 0,
    }
}

fn main() {
    let result = io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap())
        .map(to_points)
        .sum::<i32>();
    println!("{}", result);
}
