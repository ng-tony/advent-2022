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
        let line = line.unwrap();
        let v = line.split(",");

        let v: Vec<_> = v
            .map(|interval_str| {
                let mut iter = interval_str.split("-");
                [
                    (iter.next()).unwrap().parse::<i32>().unwrap(),
                    (iter.next()).unwrap().parse::<i32>().unwrap(),
                ]
            })
            .collect();

        let d = [v[0][0] - v[1][0], v[0][1] - v[1][1]];
        if !(d.iter().all(|val| val.is_positive()) || d.iter().all(|val| val.is_negative())) {
            sum += 1;
        }
    }
    sum as u32
}

fn part_two(input_file_path: &str) -> u32 {
    let mut sum = 0;
    let file = File::open(input_file_path).expect("file not found");
    let contents = BufReader::new(file);

    for line in contents.lines() {
        let line = line.unwrap();
        let v = line.split(",");

        let v: Vec<_> = v
            .map(|interval_str| {
                let mut iter = interval_str.split("-");
                [
                    (iter.next()).unwrap().parse::<i32>().unwrap(),
                    (iter.next()).unwrap().parse::<i32>().unwrap(),
                ]
            })
            .collect();

        let a = v[0];
        let b = v[1];

        //|___|
        //  |___|
        //b[0] <= a[1] && b[1] >= a[1]
        //
        //  |___|
        //|___|
        //
        //a[0] <= b[1] && a[1] >= b[1]

        if (b[0] <= a[1] && b[1] >= a[1]) || (a[0] <= b[1] && a[1] >= b[1]) {
            sum += 1;
        }
    }
    sum as u32
}
