use std::{io, io::prelude::*, vec};

enum Operation {
    Add(u64),
    Multiply(u64),
    Square,
}

struct Monkey {
    items: Vec<u64>,
    operation: Operation,
    divisor: u64,
    true_target: usize,
    false_target: usize,
    inspection_count: u32,
}

impl Monkey {
    fn inspect_and_throw_all(&mut self, common_divisor: u64, throws: &mut Vec<Throw>) {
        for item in &self.items {
            let mut worry_level = match self.operation {
                Operation::Add(n) => item + n,
                Operation::Multiply(n) => item * n,
                Operation::Square => item * item,
            };
            worry_level %= common_divisor;
            if worry_level % self.divisor == 0 {
                throws.push(Throw {
                    item: worry_level,
                    target: self.true_target,
                });
            } else {
                throws.push(Throw {
                    item: worry_level,
                    target: self.false_target,
                });
            }
            self.inspection_count += 1;
        }
        self.items.clear();
    }

    fn receive_throw(&mut self, item: u64) {
        self.items.push(item);
    }
}

struct Throw {
    item: u64,
    target: usize,
}

fn main() {
    let mut monkies = vec![
        Monkey {
            items: vec![98, 97, 98, 55, 56, 72],
            operation: Operation::Multiply(13),
            divisor: 11,
            true_target: 4,
            false_target: 7,
            inspection_count: 0,
        },
        Monkey {
            items: vec![73, 99, 55, 54, 88, 50, 55],
            operation: Operation::Add(4),
            divisor: 17,
            true_target: 2,
            false_target: 6,
            inspection_count: 0,
        },
        Monkey {
            items: vec![67, 98],
            operation: Operation::Multiply(11),
            divisor: 5,
            true_target: 6,
            false_target: 5,
            inspection_count: 0,
        },
        Monkey {
            items: vec![82, 91, 92, 53, 99],
            operation: Operation::Add(8),
            divisor: 13,
            true_target: 1,
            false_target: 2,
            inspection_count: 0,
        },
        Monkey {
            items: vec![52, 62, 94, 96, 52, 87, 53, 60],
            operation: Operation::Square,
            divisor: 19,
            true_target: 3,
            false_target: 1,
            inspection_count: 0,
        },
        Monkey {
            items: vec![94, 80, 84, 79],
            operation: Operation::Add(5),
            divisor: 2,
            true_target: 7,
            false_target: 0,
            inspection_count: 0,
        },
        Monkey {
            items: vec![89],
            operation: Operation::Add(1),
            divisor: 3,
            true_target: 0,
            false_target: 5,
            inspection_count: 0,
        },
        Monkey {
            items: vec![70, 59, 63],
            operation: Operation::Add(3),
            divisor: 7,
            true_target: 4,
            false_target: 3,
            inspection_count: 0,
        },
    ];

    let common_divisor = monkies.iter().fold(1, |acc, monkey| acc * monkey.divisor);

    for round in 0..10000 {
        for i in 0..monkies.len() {
            let source_monkey = &mut monkies[i];
            let mut throws: Vec<Throw> = vec![];
            source_monkey.inspect_and_throw_all(common_divisor, &mut throws);
            for throw in throws {
                let target_monkey = &mut monkies[throw.target];
                target_monkey.receive_throw(throw.item);
            }
        }
    }

    for i in 0..monkies.len() {
        println!(
            "Monkey {} inspected items {} times",
            i, monkies[i].inspection_count
        );
    }

    let mut inspection_counts = monkies
        .iter()
        .map(|monkey| monkey.inspection_count as u64)
        .collect::<Vec<u64>>();
    inspection_counts.sort();
    inspection_counts.reverse();
    println!("{}", inspection_counts[0] * inspection_counts[1]);
}
