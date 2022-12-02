use std::{io, io::prelude::*};

// A rock
// B paper
// C scissors
// X rock
// Y paper
// Z scissors

// rock 1
// paper 2
// scissors 3

// win 6
// draw 3
// lose 0

fn to_points(line: String) -> i32 {
    match line.as_str() {
        "A X" => 4, // tie + rock
        "A Y" => 8, // win + paper
        "A Z" => 3, // lose + scissors
        "B X" => 1, // lose + rock
        "B Y" => 5, // tie + paper
        "B Z" => 9, // win + scissors
        "C X" => 7, // win + rock
        "C Y" => 2, // lose + paper
        "C Z" => 6, // tie + scissors
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
