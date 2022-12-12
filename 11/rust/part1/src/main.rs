use std::{io, io::prelude::*, vec};

enum Operation {
    Add(u32),
    Multiply(u32),
    Square,
}

struct Monkey {
    items: Vec<u32>,
    operation: Operation,
    divisor: u32,
    true_target: usize,
    false_target: usize,
    inspection_count: u32,
}

impl Monkey {
    fn inspect_and_throw_all(&mut self, throws: &mut Vec<Throw>) {
        for item in &self.items {
            // println!("  Monkey inspects an item with a worry level of {}.", item);
            let mut worry_level = match self.operation {
                Operation::Add(n) => {
                    // println!("    Worry level increases by {} to {}.", n, item + n);
                    item + n
                }
                Operation::Multiply(n) => {
                    // println!("    Worry level is multiplied by {} to {}.", n, item * n);
                    item * n
                }
                Operation::Square => {
                    // println!(
                    //     "    Worry level is multiplied by itself to {}.",
                    //     item * item
                    // );
                    item * item
                }
            };
            // println!(
            //     "    Monkey gets bored with item. Worry level is divided by 3 to {}.",
            //     worry_level / 3
            // );
            worry_level /= 3;
            if worry_level % self.divisor == 0 {
                // println!("    Current worry level is divisible by {}.", self.divisor);
                // println!(
                //     "    Item with worry level {} is thrown to monkey {}",
                //     worry_level, self.true_target
                // );
                throws.push(Throw {
                    item: worry_level,
                    target: self.true_target,
                });
            } else {
                // println!(
                //     "    Current worry level is not divisible by {}.",
                //     self.divisor
                // );
                // println!(
                //     "    Item with worry level {} is thrown to monkey {}",
                //     worry_level, self.false_target
                // );
                throws.push(Throw {
                    item: worry_level,
                    target: self.false_target,
                });
            }
            self.inspection_count += 1;
        }
        self.items.clear();
    }

    fn receive_throw(&mut self, item: u32) {
        self.items.push(item);
    }
}

struct Throw {
    item: u32,
    target: usize,
}

fn main() {
    let lines = std::io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<String>>();
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

    for round in 0..20 {
        for i in 0..monkies.len() {
            // println!("Monkey {}:", i);
            let source_monkey = &mut monkies[i];
            let mut throws: Vec<Throw> = vec![];
            source_monkey.inspect_and_throw_all(&mut throws);
            for throw in throws {
                let target_monkey = &mut monkies[throw.target];
                target_monkey.receive_throw(throw.item);
            }
        }
        // for i in 0..monkies.len() {
        //     println!(
        //         "Monkey {}: {}",
        //         i,
        //         monkies[i]
        //             .items
        //             .iter()
        //             .map(|n| n.to_string())
        //             .collect::<Vec<String>>()
        //             .join(", ")
        //     );
        // }
    }

    for i in 0..monkies.len() {
        println!(
            "Monkey {} inspected items {} times",
            i, monkies[i].inspection_count
        );
    }

    let mut inspection_counts = monkies
        .iter()
        .map(|monkey| monkey.inspection_count)
        .collect::<Vec<u32>>();
    inspection_counts.sort();
    inspection_counts.reverse();
    println!("{}", inspection_counts[0] * inspection_counts[1]);
}
