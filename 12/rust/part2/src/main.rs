use std::collections::{HashSet, VecDeque};
use std::fmt;
use std::{collections::HashMap, io, io::prelude::*, u32};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    row: usize,
    column: usize,
}

impl Position {
    fn new() -> Self {
        Self { row: 0, column: 0 }
    }

    fn from(row: usize, column: usize) -> Self {
        Self { row, column }
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

fn main() {
    let height_markers: Vec<Vec<char>> = io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap().chars().collect())
        .collect();
    let rows = height_markers.len();
    let columns = height_markers[0].len();
    let mut heights: Vec<Vec<u8>> = vec![vec![0; columns]; rows];
    let mut start_position: Option<Position> = None;
    let mut target_position: Option<Position> = None;
    for i in 0..rows {
        let row = &height_markers[i];
        for j in 0..columns {
            match row[j] {
                'S' => {
                    heights[i][j] = 0;
                    start_position = Some(Position::from(i, j));
                }
                'E' => {
                    heights[i][j] = 25;
                    target_position = Some(Position::from(i, j));
                }
                _ => {
                    heights[i][j] = row[j] as u8 - 'a' as u8;
                }
            }
        }
    }
    let mut neighbours: Vec<Vec<Vec<Position>>> = vec![vec![vec![]; columns]; rows];
    for i in 0..rows {
        for j in 0..columns {
            if i != 0 {
                if heights[i - 1][j] <= heights[i][j] + 1 {
                    neighbours[i][j].push(Position::from(i - 1, j));
                }
            }
            if i < rows - 1 {
                if heights[i + 1][j] <= heights[i][j] + 1 {
                    neighbours[i][j].push(Position::from(i + 1, j));
                }
            }
            if j != 0 {
                if heights[i][j - 1] <= heights[i][j] + 1 {
                    neighbours[i][j].push(Position::from(i, j - 1));
                }
            }
            if j < columns - 1 {
                if heights[i][j + 1] <= heights[i][j] + 1 {
                    neighbours[i][j].push(Position::from(i, j + 1));
                }
            }
        }
    }
    let mut shortest_paths: Vec<usize> = vec![];
    for i in 0..rows {
        for j in 0..columns {
            if heights[i][j] == 0 {
                shortest_paths.push(shortest_path(
                    Position::from(i, j),
                    target_position.unwrap(),
                    &neighbours,
                ));
            }
        }
    }
    shortest_paths.sort();
    println!("{}", shortest_paths[0]);
}

fn shortest_path(from: Position, to: Position, neighbours: &Vec<Vec<Vec<Position>>>) -> usize {
    let rows = neighbours.len();
    let columns = neighbours[0].len();
    let mut distances: Vec<Vec<u32>> = vec![vec![std::u32::MAX; columns]; rows];
    let mut prev: Vec<Vec<Option<Position>>> = vec![vec![None; columns]; rows];
    let mut frontier: VecDeque<Position> = VecDeque::new();
    let mut visited: HashSet<Position> = HashSet::new();
    frontier.push_front(from);
    distances[from.row][from.column] = 0;
    while !frontier.is_empty() {
        let u = frontier.pop_front().unwrap();
        println!("Point {:?}", u);
        for n in &neighbours[u.row][u.column] {
            if !visited.contains(n) && !frontier.contains(n) {
                let alt = distances[u.row][u.column] + 1;
                if alt < distances[n.row][n.column] {
                    distances[n.row][n.column] = alt;
                    prev[n.row][n.column] = Some(u);
                }
                frontier.push_back(*n);
            }
        }
        visited.insert(u);
    }
    if prev[to.row][to.column].is_none() {
        return std::usize::MAX;
    }
    let mut shortest_path: VecDeque<Position> = VecDeque::new();
    let mut p = to;
    loop {
        shortest_path.push_front(p);
        p = prev[p.row][p.column].unwrap();
        if p == from {
            break;
        }
    }
    shortest_path.len()
}
