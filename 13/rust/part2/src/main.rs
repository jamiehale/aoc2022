use core::fmt;
use std::slice::Iter;
use std::{io, io::prelude::*};

enum Item {
    Integer(u32),
    List(Vec<Item>),
}

impl fmt::Debug for Item {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Item::Integer(n) => write!(f, "{}", n)?,
            Item::List(l) => {
                let s: Vec<String> = l.iter().map(|i| format!("{:?}", i)).collect();
                write!(f, "[{}]", s.join(","))?
            }
        }
        fmt::Result::Ok(())
    }
}

#[derive(Debug)]
struct Packet {
    items: Vec<Item>,
}

fn parse_list(chars: &mut Iter<u8>) -> Vec<Item> {
    let mut items: Vec<Item> = vec![];
    let mut parsing_int = false;
    let mut n = 0;
    loop {
        let c = *chars.next().unwrap() as char;
        // println!("  Found {}", c);
        match c {
            '[' => {
                // println!("    Parsing list...");
                items.push(Item::List(parse_list(chars)));
                // println!("    -> Got {:?}", items.last().unwrap());
            }
            ']' => {
                if parsing_int {
                    // println!("  Adding {}", n);
                    items.push(Item::Integer(n));
                    parsing_int = false;
                    n = 0;
                }
                // println!("  Closing list");
                break;
            }
            '0'..='9' => {
                parsing_int = true;
                n = n * 10 + (c as u8 - '0' as u8) as u32;
                // println!("  Got digit {} n is now {}", c, n);
            }
            ',' => {
                if parsing_int {
                    // println!("  Adding {} after comma", n);
                    items.push(Item::Integer(n));
                    parsing_int = false;
                    n = 0;
                }
            }
            _ => {
                panic!("Unexpected character in stream");
            }
        }
    }
    return items;
}

fn parse_line(line: String) -> Packet {
    // println!("Parsing line {}", line);
    let mut chars = line.as_bytes().iter();
    let first = chars.next().unwrap();
    if *first != '[' as u8 {
        panic!("Expecting [ as first character on line");
    }
    let items = parse_list(&mut chars);
    Packet { items }
}

#[derive(PartialEq)]
enum Comparison {
    LessThan,
    EqualTo,
    GreaterThan,
}

fn compare_items(left: &Vec<Item>, right: &Vec<Item>) -> Comparison {
    println!("--- comparing");
    println!("{:?}", left);
    println!("{:?}", right);
    for i in 0..left.len() {
        println!("** index {}", i);
        if i >= right.len() {
            println!("-- ran out of right");
            return Comparison::GreaterThan;
        }
        let left_value = &left[i];
        let right_value = &right[i];

        match left_value {
            Item::Integer(n) => match right_value {
                Item::Integer(m) => {
                    if n < m {
                        println!("== left int < right int - ok");
                        return Comparison::LessThan;
                    }
                    if n > m {
                        println!("== left int > right int - not ok");
                        return Comparison::GreaterThan;
                    }
                    // else equal means test next index
                }
                Item::List(l) => {
                    println!("== converting left int to list");
                    let left_as_list = vec![Item::Integer(*n)];
                    match compare_items(&left_as_list, l) {
                        Comparison::LessThan => return Comparison::LessThan,
                        Comparison::GreaterThan => return Comparison::GreaterThan,
                        Comparison::EqualTo => {} // equal means test next index
                    }
                }
            },
            Item::List(l) => match right_value {
                Item::Integer(n) => {
                    println!("== converting right int to list");
                    let right_as_list = vec![Item::Integer(*n)];
                    match compare_items(l, &right_as_list) {
                        Comparison::LessThan => return Comparison::LessThan,
                        Comparison::GreaterThan => return Comparison::GreaterThan,
                        Comparison::EqualTo => {} // equal means test next index
                    }
                }
                Item::List(m) => {
                    println!("== comparing sublists");
                    println!("{:?}", l);
                    println!("{:?}", m);
                    match compare_items(l, m) {
                        Comparison::LessThan => return Comparison::LessThan,
                        Comparison::GreaterThan => return Comparison::GreaterThan,
                        Comparison::EqualTo => {} // equal means test next index
                    }
                }
            },
        }
    }
    if left.len() < right.len() {
        println!("-- ran out of left - ok");
        Comparison::LessThan
    } else {
        Comparison::EqualTo
    }
}

fn main() {
    let mut packets: Vec<Packet> = io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap())
        .filter(|line| !line.is_empty())
        .map(parse_line)
        .collect();
    packets.push(Packet {
        items: vec![Item::List(vec![Item::Integer(2)])],
    });
    packets.push(Packet {
        items: vec![Item::List(vec![Item::Integer(6)])],
    });
    packets.sort_by(|a, b| match compare_items(&a.items, &b.items) {
        Comparison::LessThan => std::cmp::Ordering::Less,
        Comparison::GreaterThan => std::cmp::Ordering::Greater,
        Comparison::EqualTo => std::cmp::Ordering::Equal,
    });
    let score = packets.iter().enumerate().fold(1, |acc, (i, b)| {
        let is_2 = compare_items(&b.items, &vec![Item::List(vec![Item::Integer(2)])])
            == Comparison::EqualTo;
        let is_6 = compare_items(&b.items, &vec![Item::List(vec![Item::Integer(6)])])
            == Comparison::EqualTo;
        if is_2 || is_6 {
            acc * (i + 1)
        } else {
            acc
        }
    });
    println!("{:?}", score);
}
