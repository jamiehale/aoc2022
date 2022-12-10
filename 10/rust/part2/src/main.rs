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

fn draw_pixel(position: u32, sprite_position: &i32, line: &mut String) {
    line.push(
        if position as i32 >= sprite_position - 1 && position as i32 <= sprite_position + 1 {
            '#'
        } else {
            '.'
        },
    );
}

fn flush_line_if_full(current_line: &mut String) {
    if current_line.len() == 40 {
        println!("{}", current_line);
        current_line.clear();
    }
}

fn main() {
    let mut cycle: u32 = 1;
    let mut reg_x: i32 = 1;

    let instructions = io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap())
        .map(line_to_instruction)
        .collect::<Vec<Instruction>>();

    let mut current_line = String::from("");

    // println!(
    //     "Sprite position : {}###{}",
    //     ".".repeat(reg_x as usize - 1),
    //     ".".repeat(40 - reg_x as usize - 2)
    // );

    for instruction in instructions {
        // println!();
        match instruction {
            Instruction::Noop => {
                // println!("Start cycle {: >4}: begin executing noop", cycle);
                let position = (cycle - 1) % 40;
                // println!(
                //     "During cycle {: >3}: CRT draws pixel in position {}",
                //     cycle, position
                // );
                draw_pixel(position, &reg_x, &mut current_line);
                // println!("Current CRT row : {}", current_line);
                // println!("End of cycle {: >3}: finish executing noop", cycle);
                flush_line_if_full(&mut current_line);
                cycle += 1;
            }
            Instruction::AddX(n) => {
                // println!("Start cycle {: >4}: begin executing addx {}", cycle, n);
                let position = (cycle - 1) % 40;
                // println!(
                //     "During cycle {: >3}: CRT draws pixel in position {}",
                //     cycle, position
                // );
                draw_pixel(position, &reg_x, &mut current_line);
                // println!("Current CRT row : {}", current_line);
                // println!("");
                flush_line_if_full(&mut current_line);
                cycle += 1;
                let position = (cycle - 1) % 40;
                // println!(
                //     "During cycle {: >3}: CRT draws pixel in position {}",
                //     cycle, position
                // );
                draw_pixel(position, &reg_x, &mut current_line);
                // println!("Current CRT row : {}", current_line);
                reg_x += n;
                // println!(
                //     "End of cycle {: >3}: finish executing addx {} (Register X is now {})",
                //     cycle, n, reg_x
                // );
                // if reg_x < 1 {
                //     println!(
                //         "Sprite position : {}{}",
                //         "#".repeat(max(3, reg_x + 2) as usize),
                //         ".".repeat((40 - reg_x - 2) as usize)
                //     );
                // } else if reg_x > 38 {
                //     println!(
                //         "Sprite position : {}{}",
                //         ".".repeat((reg_x - 1) as usize),
                //         "#".repeat(max(3, reg_x - 39) as usize),
                //     );
                // } else {
                //     println!(
                //         "Sprite position : {}###{}",
                //         ".".repeat((reg_x - 1) as usize),
                //         ".".repeat((40 - reg_x - 2) as usize)
                //     );
                // }
                flush_line_if_full(&mut current_line);
                cycle += 1;
            }
        }
    }
}
