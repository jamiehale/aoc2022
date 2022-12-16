use core::fmt;
use std::cmp;
use std::{io, io::prelude::*, ops::Range};

#[derive(Debug, Clone, Copy, PartialEq)]
enum Cell {
    Empty,
    Wall,
    Sand,
}

#[derive(Clone, Copy)]
struct Point {
    row: usize,
    column: usize,
}

impl Point {
    fn from(row: usize, column: usize) -> Self {
        Self { row, column }
    }
}

struct World {
    cells: Vec<Vec<Cell>>,
    height: usize,
}

impl fmt::Debug for World {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let max_width = self.cells.iter().map(|row| row.len()).max().unwrap();
        for row in self.cells.iter() {
            for column in 300..row.len() {
                match row[column] {
                    Cell::Empty => write!(f, ".")?,
                    Cell::Wall => write!(f, "#")?,
                    Cell::Sand => write!(f, "o")?,
                };
            }
            for _ in cmp::max(row.len(), 300)..max_width {
                write!(f, ".")?;
            }
            write!(f, "\n")?;
        }
        fmt::Result::Ok(())
    }
}

fn grow_columns(column: usize, row: &mut Vec<Cell>) {
    if column >= row.len() {
        row.extend(vec![Cell::Empty; column - row.len() + 1].into_iter());
    }
}

impl World {
    fn new() -> Self {
        Self {
            cells: vec![],
            height: 0,
        }
    }

    fn from(walls: &Vec<Vec<Point>>) -> Self {
        let mut world = Self {
            cells: vec![],
            height: 0,
        };
        for wall in walls {
            world.set(
                Point {
                    row: wall[0].row,
                    column: wall[0].column,
                },
                Cell::Wall,
            );
            for i in 1..wall.len() {
                let from = &wall[i - 1];
                let to = &wall[i];
                if from.row == to.row {
                    let r = if from.column < to.column {
                        Range {
                            start: from.column,
                            end: to.column + 1,
                        }
                    } else {
                        Range {
                            start: to.column,
                            end: from.column + 1,
                        }
                    };
                    for c in r {
                        world.set(Point::from(from.row, c), Cell::Wall);
                    }
                } else {
                    let r = if from.row < to.row {
                        Range {
                            start: from.row,
                            end: to.row + 1,
                        }
                    } else {
                        Range {
                            start: to.row,
                            end: from.row + 1,
                        }
                    };
                    for c in r {
                        world.set(Point::from(c, from.column), Cell::Wall);
                    }
                }
            }
        }
        world.height = world.cells.len();
        world
    }

    fn grow_rows(&mut self, row: usize) {
        if row >= self.cells.len() {
            self.cells
                .extend(vec![vec![]; row - self.cells.len() + 1].into_iter());
        }
    }

    fn get(&self, row: usize, column: usize) -> Cell {
        if row >= self.cells.len() {
            return Cell::Empty;
        }
        let r = &self.cells[row];
        if column >= r.len() {
            return Cell::Empty;
        }
        r[column]
    }

    fn set(&mut self, point: Point, value: Cell) {
        self.grow_rows(point.row);
        let mut r = &mut self.cells[point.row];
        grow_columns(point.column, r);
        r[point.column] = value;
    }

    fn add_sand(&mut self, point: Point) {
        let mut p = point;
        loop {
            if p.row >= self.height {
                self.set(p, Cell::Sand);
                break;
            }

            match self.get(p.row + 1, p.column) {
                Cell::Empty => {
                    p = Point::from(p.row + 1, p.column);
                }
                _ => match self.get(p.row + 1, p.column - 1) {
                    Cell::Empty => {
                        p = Point::from(p.row + 1, p.column - 1);
                    }
                    _ => match self.get(p.row + 1, p.column + 1) {
                        Cell::Empty => {
                            p = Point::from(p.row + 1, p.column + 1);
                        }
                        _ => {
                            self.set(p, Cell::Sand);
                            break;
                        }
                    },
                },
            }
        }
    }
}

fn read_walls() -> Vec<Vec<Point>> {
    io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap())
        .map(|line| {
            line.split(" -> ")
                .map(|coords| {
                    coords
                        .split(",")
                        .map(|s| s.parse::<usize>().unwrap())
                        .collect::<Vec<usize>>()
                })
                .map(|p| Point {
                    row: p[1],
                    column: p[0],
                })
                .collect::<Vec<Point>>()
        })
        .collect()
}

fn main() {
    let walls = read_walls();
    let mut world = World::from(&walls);
    println!("{:?}", world);
    let mut count = 0;
    loop {
        world.add_sand(Point::from(0, 500));
        count += 1;
        if world.get(0, 500) == Cell::Sand {
            break;
        }
        if count % 100 == 0 {
            println!("{:?}", world);
        }
        // if count == 30 {
        //     break;
        // }
    }
    println!("{}", count);
}
