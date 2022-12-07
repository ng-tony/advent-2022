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

fn part_one(input_file_path: &str) -> u32 {
    let file = File::open(input_file_path).expect("file not found");
    let contents = BufReader::new(file);

    let line = contents.lines().next().unwrap().unwrap();

    let mut chars_since_repeat = 0;
    let mut chars = line.chars().collect::<Vec<char>>();
    chars.append(&mut vec!['0'; 4]);

    for (i, &char) in chars.iter().enumerate() {
        for j in i + 1..i + 5 - chars_since_repeat {
            if chars[j] == char {
                chars_since_repeat = 0;
            }
        }
        chars_since_repeat += 1;
        if chars_since_repeat == 4 {
            return (i + 2) as u32;
        }
    }
    0
}

fn part_two(input_file_path: &str) -> u32 {
    let file = File::open(input_file_path).expect("file not found");
    let contents = BufReader::new(file);

    let line = contents.lines().next().unwrap().unwrap();

    let mut chars_since_repeat = 0;
    let mut chars = line.chars().collect::<Vec<char>>();
    chars.append(&mut vec!['0'; 14]);

    for (i, &char) in chars.iter().enumerate() {
        for j in i + 1..(i + 15-chars_since_repeat) {
            if chars[j] == char {
                chars_since_repeat = 0;
            }
        }
        chars_since_repeat += 1;
        if chars_since_repeat == 14 {
            return (i + 2) as u32;
        }
    }
    0
}

