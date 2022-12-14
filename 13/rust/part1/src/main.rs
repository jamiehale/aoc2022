use std::slice::Iter;
use std::{io, io::prelude::*};

#[derive(Debug)]
enum Item {
    Integer(u32),
    List(Vec<Item>),
}

impl fmt::Debug for Item {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Item::Integer(n) => write!(f, "{}", n),
            Item::List(l) => {
                let s:Vec<String> = l.iter().map(|i| format!("{:?}", i)).collect();
                write!(f, "{}", l.iter().map(|i| format!("{:?}", i)).collect())
        }
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

fn compare_items(left: &Vec<Item>, right: &Vec<Item>) -> bool {
    println!("--- comparing");
    println!("{:?}", left);
    println!("{:?}", right);
    for i in 0..left.len() {
        println!("** index {}", i);
        if i >= right.len() {
            println!("-- ran out of right");
            return false;
        }
        let left_value = &left[i];
        let right_value = &right[i];

        match left_value {
            Item::Integer(n) => match right_value {
                Item::Integer(m) => {
                    if n < m {
                        println!("== left int < right int - ok");
                        return true;
                    }
                    if n > m {
                        println!("== left int > right int - not ok");
                        return false;
                    }
                }
                Item::List(l) => {
                    println!("== converting left int to list");
                    let left_as_list = vec![Item::List(vec![Item::Integer(*n)])];
                    if !compare_items(&left_as_list, l) {
                        return false;
                    }
                }
            },
            Item::List(l) => match right_value {
                Item::Integer(n) => {
                    println!("== converting right int to list");
                    let right_as_list = vec![Item::List(vec![Item::Integer(*n)])];
                    if !compare_items(l, &right_as_list) {
                        return false;
                    }
                }
                Item::List(m) => {
                    println!("== comparing sublists");
                    println!("{:?}", l);
                    println!("{:?}", m);
                    if !compare_items(l, m) {
                        return false;
                    }
                }
            },
        }
    }
    println!("-- ran out of left - ok");
    true
}

fn main() {
    let lines: Vec<Packet> = io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap())
        .filter(|line| !line.is_empty())
        .map(parse_line)
        .collect();
    let mut pairs: Vec<bool> = lines
        .chunks(2)
        .map(|f| {
            let left = &f[0];
            let right = &f[1];
            println!("\nStarting {:?} vs {:?}", left, right);
            compare_items(&left.items, &right.items)
        })
        .collect();
    let count = pairs
        .iter()
        .enumerate()
        .fold(0, |acc, (i, b)| if *b { acc + i + 1 } else { acc });
    println!("{:?}", count);
}
