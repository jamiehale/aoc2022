use std::collections::HashMap;
use std::{io, io::prelude::*};

fn key_from_path(path: &Vec<String>) -> String {
    let mut joined = path.join("/");
    joined.insert_str(0, "/");
    joined
}

fn process_command(
    command: &String,
    path: &mut Vec<String>,
    folder_folders: &mut HashMap<String, Vec<String>>,
    folder_files: &mut HashMap<String, Vec<(String, usize)>>,
) {
    match command.as_str() {
        "$ cd /" => {
            path.clear();
        }
        "$ cd .." => {
            path.pop();
        }
        "$ ls" => {}
        _ => {
            let name = &command[5..];
            path.push(String::from(name));
        }
    }
    match folder_folders.get_mut(&key_from_path(path)) {
        None => {
            folder_folders.insert(key_from_path(path), vec![]);
        }
        _ => {}
    }
    match folder_files.get_mut(&key_from_path(path)) {
        None => {
            folder_files.insert(key_from_path(path), vec![]);
        }
        _ => {}
    }
}

fn calculate_folder_size(
    folder_folders: &HashMap<String, Vec<String>>,
    folder_files: &HashMap<String, Vec<(String, usize)>>,
    name: &String,
) -> usize {
    let total_folder_size = match folder_folders.get(name) {
        Some(folders) => folders
            .iter()
            .map(|folder_name| {
                calculate_folder_size(
                    folder_folders,
                    folder_files,
                    &format!(
                        "{}{}",
                        if name == "/" {
                            String::from("/")
                        } else {
                            format!("{}/", name)
                        },
                        folder_name
                    ),
                )
            })
            .sum::<usize>(),
        None => 0,
    };
    let total_file_size = match folder_files.get(name) {
        Some(files) => files.iter().map(|(_, size)| size).sum::<usize>(),
        None => 0,
    };

    // println!(
    //     "{}: folders = {} files = {}",
    //     name, total_folder_size, total_file_size
    // );
    total_folder_size + total_file_size
}

fn main() {
    let mut folder_folders: HashMap<String, Vec<String>> = HashMap::new();
    let mut folder_files: HashMap<String, Vec<(String, usize)>> = HashMap::new();
    let mut path: Vec<String> = vec![];

    let all_lines = io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<String>>();
    let mut lines = all_lines.iter();

    loop {
        let line = lines.next();
        match line {
            Some(command) => {
                // println!("processing line: {} (cwd = {})", command, path.join("/"));
                if command.starts_with("$") {
                    process_command(&command, &mut path, &mut folder_folders, &mut folder_files);
                } else if command.starts_with("dir") {
                    let tokens: Vec<&str> = command.split(' ').collect();
                    let name = tokens[1];
                    // println!("  adding folder {} to {}", name, path.join("/"));
                    match folder_folders.get_mut(&key_from_path(&path)) {
                        Some(entry) => {
                            entry.push(String::from(name));
                        }
                        None => {
                            folder_folders.insert(key_from_path(&path), vec![String::from(name)]);
                        }
                    };
                } else {
                    let tokens: Vec<&str> = command.split(' ').collect();
                    let size = tokens[0].parse::<usize>().unwrap();
                    let name = tokens[1];
                    // println!("  adding file {} ({}) to {}", name, size, path.join("/"));
                    match folder_files.get_mut(&key_from_path(&path)) {
                        Some(entry) => {
                            entry.push((String::from(name), size));
                        }
                        None => {
                            folder_files
                                .insert(key_from_path(&path), vec![(String::from(name), size)]);
                        }
                    };
                }
            }
            None => {
                break;
            }
        }
    }
    let root_folder_size =
        calculate_folder_size(&folder_folders, &folder_files, &String::from("/"));
    let amount_to_free: usize = root_folder_size - (70000000 - 30000000);
    let min_free = folder_folders
        .keys()
        .map(|name| calculate_folder_size(&folder_folders, &folder_files, name))
        .filter(|size| *size >= amount_to_free)
        .min()
        .unwrap();
    println!("{}", min_free);
}
