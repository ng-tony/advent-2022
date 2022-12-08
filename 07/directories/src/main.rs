use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::iter::Peekable;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_file_path = &args[1];

    println!("Part one: {}", part_one(input_file_path));
    println!("Part two: {}", part_two(input_file_path));
}
//Implementation breaks if user doesn't run dir, after traversing to a new directory
//Could fix if I add a flag, but is unneccessary for this input
fn map_dir(file: File) -> HashMap<String, u32> {
    let contents = BufReader::new(file);
    //Assume all directories are traveresd
    let mut directories = HashMap::new();
    let mut pwd = "".to_string();
    directories.insert(pwd, 0);
    pwd = "".to_string();

    fn handle_ls(
        iter: &mut Peekable<std::io::Lines<BufReader<File>>>,
        directories: &mut HashMap<String, u32>,
        pwd: &String,
    ) {
        let mut sum = 0;
        while let Some(Ok(line)) = iter.peek() {
            let mut words = line.split(" ");
            match words.next() {
                Some("$") => {
                    break;
                }
                Some("dir") => {
                    //Hope there's no untraversed directories :p
                }
                Some(val) => {
                    sum += str::parse::<u32>(val).unwrap();
                }
                _ => panic!("Ls format broke"),
            }
            iter.next();
        }
        *directories.entry(pwd.clone()).or_insert(sum) += 0;

        let mut curr = pwd.clone();
        while !curr.is_empty() {
            match curr.rfind("/") {
                Some(last_index) => curr.truncate(last_index),
                None => curr.truncate(0),
            }
            *directories.entry(curr.clone()).or_insert(sum) += sum;
        }
    }

    let mut iter = contents.lines().peekable();
    while let Some(Ok(line)) = iter.next() {
        let mut words = line.split(" ").skip(1);
        match words.next() {
            Some("ls") => {
                handle_ls(&mut iter, &mut directories, &pwd);
            }

            Some("cd") => match words.next() {
                Some("/") => {
                    pwd = "".to_string();
                }
                Some("..") => match pwd.rfind("/") {
                    Some(last_index) => pwd.truncate(last_index),
                    None => pwd.truncate(0),
                },
                Some(dir) => {
                    pwd.push_str(&format!("/{dir}"));
                }
                _ => panic!("Bad cd"),
            },
            _ => panic!(),
        }
    }
    return directories;
}

fn part_one(input_file_path: &str) -> u32 {
    let file = File::open(input_file_path).expect("file not found");
    let mut sum = 0;
    let directories = map_dir(file);
    for (_, &value) in directories.iter() {
        if value <= 100000 {
            sum += value
        }
    }
    sum
}

fn part_two(input_file_path: &str) -> u32 {
    let file = File::open(input_file_path).expect("file not found");
    let mut best_val = u32::MAX;
    let directories = map_dir(file);
    let total = directories.get("").unwrap();
    let to_free = 30000000 - (70000000 - total);
    for (_, &value) in directories.iter() {
        if value >= to_free && value < best_val {
            best_val = value;
        }
    }
    best_val
}
