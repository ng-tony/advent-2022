use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_file_path = &args[1];

    println!("Part one: {}", part_one(input_file_path));
    println!("Part two: {}", part_two(input_file_path));
}

fn part_one(input_file_path: &str) -> String {
    let file = File::open(input_file_path).expect("file not found");
    let contents = BufReader::new(file);

    let mut iter = contents.lines().peekable();
    let length = (iter.peek().unwrap().as_ref().unwrap().len() + 1) / 4;

    //Read Stacks
    let mut stacks: Vec<Vec<char>> = vec![vec![]; length];
    loop {
        let line = iter.next().unwrap().unwrap();
        if line == "" {
            break;
        }
        for i in 0..length {
            stacks[i].push(line.chars().nth(i * 4 + 1).unwrap());
        }
    }
    // Remove empties and legend numbers
    let mut stacks: Vec<Vec<char>> = stacks
        .into_iter()
        .map(|mut stack| {
            stack.pop();
            let b: Vec<char> = stack
                .into_iter()
                .filter(|val| !val.is_numeric() && !val.is_whitespace())
                .rev()
                .collect();
            b
        })
        .collect();

    for line in iter {
        let line = line.unwrap();
        let iter = line.split(" ").enumerate();

        let (amount, from, to) = iter.fold((0, 0, 0), |mut acc, (i, val)| {
            match i {
                1 => {
                    acc.0 = val.parse().unwrap();
                }
                3 => {
                    acc.1 = (val.parse::<i32>().unwrap() - 1) as usize;
                }
                5 => {
                    acc.2 = (val.parse::<i32>().unwrap() - 1) as usize;
                }
                _ => {}
            }
            acc
        });
        for _ in 0..amount {
            let t = stacks[from].pop().unwrap();
            stacks[to].push(t);
        }
    }

    let mut ans = String::new();
    for  stack in stacks {
        ans.push(*stack.last().unwrap());
    }
    ans
}

fn part_two(input_file_path: &str) -> String {
    let file = File::open(input_file_path).expect("file not found");
    let contents = BufReader::new(file);

    let mut iter = contents.lines().peekable();
    let length = (iter.peek().unwrap().as_ref().unwrap().len() + 1) / 4;

    //Read Stacks
    let mut stacks: Vec<Vec<char>> = vec![vec![]; length];
    loop {
        let line = iter.next().unwrap().unwrap();
        if line == "" {
            break;
        }
        for i in 0..length {
            stacks[i].push(line.chars().nth(i * 4 + 1).unwrap());
        }
    }
    // Remove empties and legend numbers
    let mut stacks: Vec<Vec<char>> = stacks
        .into_iter()
        .map(|mut stack| {
            stack.pop();
            let b: Vec<char> = stack
                .into_iter()
                .filter(|val| !val.is_numeric() && !val.is_whitespace())
                .rev()
                .collect();
            b
        })
        .collect();

    for line in iter {
        let line = line.unwrap();
        let iter = line.split(" ").enumerate();

        let (amount, from, to) = iter.fold((0, 0, 0), |mut acc, (i, val)| {
            match i {
                1 => {
                    acc.0 = val.parse().unwrap();
                }
                3 => {
                    acc.1 = (val.parse::<i32>().unwrap() - 1) as usize;
                }
                5 => {
                    acc.2 = (val.parse::<i32>().unwrap() - 1) as usize;
                }
                _ => {}
            }
            acc
        });
        let mut new_vals = vec![];
        let start = stacks[from].len() - amount;
        for i in start..stacks[from].len() {
            new_vals.push(stacks[from][i]);
        }
        stacks[from].truncate(start);
        stacks[to].append(&mut new_vals);
    }

    let mut ans = String::new();
    for stack in stacks {
        ans.push(*stack.last().unwrap());
    }
    ans
}

