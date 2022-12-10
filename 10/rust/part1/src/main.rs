use std::{io, io::prelude::*};

enum Instruction {
    Noop,
    AddX(i32),
}

fn line_to_instruction(line: String) -> Instruction {
    match &line[0..4] {
        "noop" => Instruction::Noop,
        _ => Instruction::AddX(line[5..].parse::<i32>().unwrap()),
    }
}

fn next_cycle(cycle: &mut u32, reg_x: &mut i32, total_signal_strength: &mut i32) {
    if [20, 60, 100, 140, 180, 220].contains(cycle) {
        *total_signal_strength += *cycle as i32 * *reg_x;
    }
    *cycle += 1;
}

fn main() {
    let mut cycle: u32 = 1;
    let mut reg_x: i32 = 1;

    let mut total_signal_strength: i32 = 0;

    let instructions = io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap())
        .map(line_to_instruction)
        .collect::<Vec<Instruction>>();

    for instruction in instructions {
        match instruction {
            Instruction::Noop => {
                next_cycle(&mut cycle, &mut reg_x, &mut total_signal_strength);
            }
            Instruction::AddX(n) => {
                next_cycle(&mut cycle, &mut reg_x, &mut total_signal_strength);
                next_cycle(&mut cycle, &mut reg_x, &mut total_signal_strength);
                reg_x += n;
            }
        }
    }

    println!("{}", total_signal_strength);
}
