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
    let mut points = 0;
    let file = File::open(input_file_path).expect("file not found");
    let contents = BufReader::new(file);

    for line in contents.lines() {
        let line = line.unwrap();
        let mut chars = line.chars();
        let (others, yours) = (chars.nth(0).unwrap(), chars.nth(1).unwrap());
        //Match result points
        let result = others as i32 - (yours as i32 - 23);
        match result {
            -2 => {}
            -1 => points += 6,
            0 => points += 3,
            1 => {}
            2 => points += 6,
            _ => panic!(),
        }

        //Picked hand results
        match yours {
            'X' => points += 1,
            'Y' => points += 2,
            'Z' => points += 3,
            _ => panic!(),
        }
    }
    points
}

fn part_two(input_file_path: &str) -> u32 {
    let mut points: u32 = 0;
    let file = File::open(input_file_path).expect("file not found");
    let contents = BufReader::new(file);

    for line in contents.lines() {
        let line = line.unwrap();
        let mut chars = line.chars();
        let (others, result) = (chars.nth(0).unwrap(), chars.nth(1).unwrap());

        //Results
        match result {
            'X' => points += 0,
            'Y' => points += 3,
            'Z' => points += 6,
            _ => panic!(),
        }
        //1 to 3
        let others = others as i32 - 64;
        let adder = match result {
            'X' => (others - 1) % 3,
            'Y' => others,
            'Z' => (others + 1) % 3,
            _ => panic!(),
        };
        points += if adder == 0 { 3 } else { adder as u32 }
    }
    points
}
