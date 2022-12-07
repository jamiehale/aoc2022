use lazy_static::lazy_static;
use regex::Regex;
// use std::collections::HashSet;
use std::collections::VecDeque;
use std::fmt;
use std::str::FromStr;
use std::{io, io::prelude::*};

struct CrateState {
    stacks: [VecDeque<char>; 9],
}

impl CrateState {
    pub fn new() -> Self {
        let stacks: [VecDeque<char>; 9] = Default::default();
        Self { stacks }
    }

    pub fn build(&mut self, stack: usize, c: char) {
        self.stacks[stack].push_front(c);
    }

    pub fn mov(&mut self, count: usize, from: usize, to: usize) {
        let mut removed: VecDeque<_> = self.stacks[from - 1]
            .drain((self.stacks[from - 1].len() - count)..)
            .collect();
        self.stacks[to - 1].append(&mut removed);
    }

    pub fn mov_singles(&mut self, count: usize, from: usize, to: usize) {
        for i in 0..count {
            self.mov(1, from, to);
        }
    }

    pub fn tops(&self) -> String {
        self.stacks
            .iter()
            .map(|stack| stack.back().unwrap())
            .into_iter()
            .collect()
    }
}

impl fmt::Debug for CrateState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in 1..10 {
            writeln!(f, "{}: {:?}", i, self.stacks[i - 1]).unwrap();
        }
        fmt::Result::Ok(())
    }
}

fn process_starting_state(crate_state: &mut CrateState, line: &String) {
    for (i, c) in line.chars().enumerate() {
        if i % 4 == 1 {
            if c != ' ' && c.is_ascii_alphabetic() {
                crate_state.build(i / 4, c);
            }
        }
    }
}

fn usize_from_str(s: &str) -> usize {
    String::from_str(s).unwrap().parse::<usize>().unwrap()
}

fn process_move(crate_state: &mut CrateState, line: &String) {
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r"^move (?P<count>\d+) from (?P<from>\d+) to (?P<to>\d+)$").unwrap();
    }
    let captures = RE.captures(line).unwrap();
    let count = usize_from_str(captures.name("count").unwrap().as_str());
    let from = usize_from_str(captures.name("from").unwrap().as_str());
    let to = usize_from_str(captures.name("to").unwrap().as_str());

    crate_state.mov(count, from, to);
}

fn process(lines: Vec<String>) -> String {
    let mut crate_state = CrateState::new();
    let mut processed_starting_state = false;
    for line in &lines {
        if !processed_starting_state {
            if line.is_empty() {
                processed_starting_state = true;
            } else {
                process_starting_state(&mut crate_state, line);
                println!("{:?}", crate_state);
            }
        } else {
            println!("Processing {}", line);
            process_move(&mut crate_state, line);
            println!("{:?}", crate_state);
        }
    }

    crate_state.tops()
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
