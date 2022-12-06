use lazy_static::lazy_static;
use regex::Regex;
// use std::collections::HashSet;
use std::{io, io::prelude::*};

fn process_starting_state(line: &String) {
    for (i, chunk) in line.as_bytes().chunks(4).enumerate() {
        let s = String::from_utf8(chunk).unwrap();
        if s[0] == '[' && s[2] == ']' {
            println!()
        }
    }
}

fn process_move(line: &String) {}

fn process(lines: Vec<String>) -> String {
    let mut processed_starting_state = false;
    for line in &lines {
        if !processed_starting_state {
            if line.is_empty() {
                processed_starting_state = true;
            } else {
                process_starting_state(line);
            }
        } else {
            process_move(line);
        }
    }

    String::from("wat")
}

fn main() {
    let result = process(
        io::stdin()
            .lock()
            .lines()
            .map(|line| line.unwrap())
            .collect::<Vec<String>>(),
    );
    println!("{:?}", result);
}
