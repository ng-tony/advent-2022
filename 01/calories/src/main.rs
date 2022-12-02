use std::cmp::Reverse;
use std::collections::BinaryHeap;
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
    let (mut curr_sum, mut max) = (0, 0);

    let file = File::open(input_file_path).expect("file not found");
    let contents = BufReader::new(file);

    for line in contents.lines(){
        match line.unwrap().parse::<u32>() {
            Ok(curr) => {
                curr_sum += curr;
            }
            Err(_) => {
                max = max.max(curr_sum);
                curr_sum = 0;
            }
        }
    }
    max.max(curr_sum)
}

fn part_two(input_file_path: &str) -> u32 {

    let mut heap = BinaryHeap::new();
    heap.push(Reverse(0));
    heap.push(Reverse(0));
    heap.push(Reverse(0));
    let mut curr_sum = 0;

    let file = File::open(input_file_path).expect("file not found");
    let contents = BufReader::new(file);

    for line in contents.lines() {
        match line.unwrap().parse::<u32>() {
            Ok(curr) => {
                curr_sum += curr;
            }
            Err(_) => {
                heap.push(Reverse(curr_sum));
                heap.pop();
                curr_sum = 0;
            }
        }
    }
    heap.push(Reverse(curr_sum));
    heap.pop();

    println!("{:?}", heap);
    heap.pop().unwrap().0 + heap.pop().unwrap().0 + heap.pop().unwrap().0
}
