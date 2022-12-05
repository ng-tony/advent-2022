use std::collections::HashSet;
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
    let mut sum = 0;
    let file = File::open(input_file_path).expect("file not found");
    let contents = BufReader::new(file);

    for line in contents.lines() {
        let mut chars_in_first_half: HashSet<char> = HashSet::new();
        let chars: Vec<char> = line.unwrap().chars().collect();

        let length = chars.len();

        for &char in chars.iter().take(length/2){
            chars_in_first_half.insert(char);
        }

        for char in chars.iter().skip(length/2).take(length/2){
            if let Some(&dupe) = chars_in_first_half.get(&char) {
                sum += if dupe.is_ascii_lowercase() {
                    dupe as i32 - 96
                } else {
                    dupe as i32 - 64 + 26
                };
                break;
            }
        }
    }
    sum as u32
}

fn part_two(input_file_path: &str) -> u32 {
    let mut sum = 0;
    let file = File::open(input_file_path).expect("file not found");
    let contents = BufReader::new(file);

    let mut iter = contents.lines();
    while let [Some(line1), Some(line2), Some(line3)] = [iter.next(), iter.next(), iter.next()] {
        let lines = [line1, line2, line3];
        let mut sets = lines.iter().fold(vec![], |mut acc, line| {
            let mut set: HashSet<char> = HashSet::new();
            for char in line.as_ref().unwrap().chars() {
                set.insert(char);
            }
            acc.push(set);
            acc
        });
        let (intersection, others) = sets.split_at_mut(1);
        let intersection = &mut intersection[0];
        for other in others {
            intersection.retain(|e| other.contains(e));
        }
        for dupe in intersection.drain() {
            sum += if dupe.is_ascii_lowercase() {
                dupe as i32 - 96
            } else {
                dupe as i32 - 64 + 26
            };
        }
    }
    sum as u32
}
