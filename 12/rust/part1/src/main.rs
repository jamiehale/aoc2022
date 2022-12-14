use std::fmt;
use std::{collections::HashMap, io, io::prelude::*, u32};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    row: i32,
    column: i32,
}

impl Position {
    fn new() -> Self {
        Self { row: 0, column: 0 }
    }

    fn from(row: usize, column: usize) -> Self {
        Self {
            row: row as i32,
            column: column as i32,
        }
    }

    fn up(&self) -> Self {
        Self {
            row: self.row - 1,
            column: self.column,
        }
    }

    fn down(&self) -> Self {
        Self {
            row: self.row + 1,
            column: self.column,
        }
    }

    fn left(&self) -> Self {
        Self {
            row: self.row,
            column: self.column - 1,
        }
    }

    fn right(&self) -> Self {
        Self {
            row: self.row,
            column: self.column + 1,
        }
    }
}

impl fmt::Debug for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({},{})", self.row, self.column)
    }
}

fn shortest_distance(
    from: Position,
    to: Position,
    heights: &HashMap<Position, u8>,
    visited: &Vec<Position>,
) -> Option<u32> {
    if *heights.get(&from).unwrap() > 0 {
        println!("yay!");
    }
    // println!(
    //     "Checking shortest distance from {:?} (height={}) through {:?}",
    //     from,
    //     heights.get(&from).unwrap(),
    //     visited
    // );
    if from == to {
        println!("We've arrived!");
        return Some(0);
    }
    let current_height = heights.get(&from).unwrap();
    let all_possible_next_steps = vec![from.up(), from.right(), from.down(), from.left()];
    let possible_next_steps: Vec<&Position> = all_possible_next_steps
        .iter()
        .filter(|position| !visited.contains(*position))
        .filter(|position| heights.contains_key(*position))
        .filter(|position| *heights.get(*position).unwrap() <= (*current_height + 1))
        .collect();
    // println!(" -> {:?}", possible_next_steps);
    let distances: Vec<u32> = possible_next_steps
        .iter()
        .map(|position| {
            let mut new_visited = visited.clone();
            new_visited.push(from);
            shortest_distance(**position, to, heights, &new_visited)
        })
        .filter(|distance| distance.is_some())
        .map(|distance| distance.unwrap())
        .collect();
    if distances.is_empty() {
        None
    } else {
        Some(*distances.iter().min().unwrap() + 1)
    }
}

fn main() {
    let height_markers: Vec<Vec<char>> = io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap().chars().collect())
        .collect();
    let mut heights: HashMap<Position, u8> = HashMap::new();
    let mut start_position: Option<Position> = None;
    let mut target_position: Option<Position> = None;
    for i in 0..height_markers.len() {
        let row = &height_markers[i];
        for j in 0..row.len() {
            match row[j] {
                'S' => {
                    heights.insert(Position::from(i, j), 0);
                    start_position = Some(Position::from(i, j));
                }
                'E' => {
                    heights.insert(Position::from(i, j), 25);
                    target_position = Some(Position::from(i, j));
                }
                _ => {
                    heights.insert(Position::from(i, j), row[j] as u8 - 'a' as u8);
                }
            }
        }
    }
    let mut neighbours: HashMap<Position, Vec<Position>> = HashMap::new();
    for i in 0..height_markers.len() {
        let row = &height_markers[i];
        for j in 0..row.len() {
            let p = Position::from(i,j).up();
            if heights.contains_key(&p) {
                if heights[p.] <= 
            }
        }
    }
    let shortest_path = shortest_distance(
        start_position.unwrap(),
        target_position.unwrap(),
        &heights,
        &vec![],
    );
    println!("{}", shortest_path.unwrap());
}
