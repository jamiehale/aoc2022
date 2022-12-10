use std::fmt;
use std::{collections::HashSet, io, io::prelude::*};

#[derive(Clone, Copy, Debug)]
enum Direction {
    Right,
    Up,
    Down,
    Left,
}

enum DiagonalDirection {
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
}

struct Instruction {
    direction: Direction,
    count: u8,
}

fn to_instruction(line: String) -> Instruction {
    Instruction {
        direction: match &line[0..1] {
            "R" => Direction::Right,
            "D" => Direction::Down,
            "U" => Direction::Up,
            "L" => Direction::Left,
            _ => panic!("Unexpected direction"),
        },
        count: line[2..].parse::<u8>().unwrap(),
    }
}

fn position_to_key(position: &Position) -> String {
    format!("{},{}", position.x, position.y)
}

#[derive(PartialEq)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn new() -> Self {
        Position { x: 0, y: 0 }
    }

    fn moved(&self, direction: Direction) -> Self {
        match direction {
            Direction::Up => Position {
                x: self.x,
                y: self.y + 1,
            },
            Direction::Down => Position {
                x: self.x,
                y: self.y - 1,
            },
            Direction::Right => Position {
                x: self.x + 1,
                y: self.y,
            },
            Direction::Left => Position {
                x: self.x - 1,
                y: self.y,
            },
        }
    }

    fn moved_diagonally(&self, direction: DiagonalDirection) -> Self {
        match direction {
            DiagonalDirection::UpLeft => Position {
                x: self.x - 1,
                y: self.y + 1,
            },
            DiagonalDirection::UpRight => Position {
                x: self.x + 1,
                y: self.y + 1,
            },
            DiagonalDirection::DownLeft => Position {
                x: self.x - 1,
                y: self.y - 1,
            },
            DiagonalDirection::DownRight => Position {
                x: self.x + 1,
                y: self.y - 1,
            },
        }
    }

    fn moved_towards(&self, other: &Self) -> Self {
        if self.x == other.x {
            if self.y == other.y {
                Position {
                    x: self.x,
                    y: self.y,
                }
            } else if self.y < other.y {
                self.moved(Direction::Up)
            } else {
                self.moved(Direction::Down)
            }
        } else if self.y == other.y {
            if self.x < other.x {
                self.moved(Direction::Right)
            } else {
                self.moved(Direction::Left)
            }
        } else {
            if self.x < other.x {
                if self.y < other.y {
                    self.moved_diagonally(DiagonalDirection::UpRight)
                } else {
                    self.moved_diagonally(DiagonalDirection::DownRight)
                }
            } else {
                if self.y < other.y {
                    self.moved_diagonally(DiagonalDirection::UpLeft)
                } else {
                    self.moved_diagonally(DiagonalDirection::DownLeft)
                }
            }
        }
    }

    fn is_adjacent_to(&self, other: &Self) -> bool {
        self.moved_towards(other) == *other
    }
}

impl fmt::Debug for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

fn main() {
    let all_instructions = io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap())
        .map(to_instruction)
        .collect::<Vec<Instruction>>();
    let mut instructions = all_instructions.iter();
    let mut visited: HashSet<String> = HashSet::new();
    let mut head = Position::new();
    let mut tail = Position::new();

    let mut active_instruction: Option<&Instruction> = None;
    let mut head_direction = Direction::Up;
    let mut head_count = 0;

    visited.insert(position_to_key(&tail));

    loop {
        println!("head {:?} tail {:?}", head, tail);
        if active_instruction.is_none() {
            active_instruction = instructions.next();
            match active_instruction {
                Some(instruction) => {
                    // println!(
                    //     "loading active instruction = {:?} {}",
                    //     instruction.direction, instruction.count
                    // );
                    head_direction = instruction.direction;
                    head_count = instruction.count;
                }
                None => break,
            }
        }
        println!("processing: {:?} ({} left)", head_direction, head_count);
        head = head.moved(head_direction);
        head_count -= 1;
        if head_count == 0 {
            active_instruction = None;
        }

        if !tail.is_adjacent_to(&head) {
            tail = tail.moved_towards(&head);
            visited.insert(position_to_key(&tail));
        }
    }

    println!("{}", visited.len());
    // println!("{:?}", visited);
}
