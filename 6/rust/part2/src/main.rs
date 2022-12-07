use itertools::Itertools;
use std::collections::VecDeque;
use std::{io, io::prelude::*};

fn is_deque_unique(deque: &VecDeque<char>, length: usize) -> bool {
    deque.into_iter().unique().collect::<Vec<&char>>().len() == length
}

fn main() {
    let mut deque: VecDeque<char> = VecDeque::new();
    for (i, c) in io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<String>>()[0]
        .chars()
        .enumerate()
    {
        deque.push_back(c);

        while deque.len() > 14 {
            deque.pop_front();
        }

        if is_deque_unique(&deque, 14) {
            println!("{}", i + 1);
            return;
        }
    }
}
