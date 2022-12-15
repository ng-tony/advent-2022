use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::usize;

//I should probably clean it up, instead of all the cnp
fn main() {
    let args: Vec<String> = env::args().collect();
    let input_file_path = &args[1];

    println!("Part one: {}", part_one(input_file_path));
    println!("Part two: {}", part_two(input_file_path));
}
fn part_one(input_file_path: &str) -> u32 {
    let file = File::open(input_file_path).expect("file not found");
    let contents = BufReader::new(file);
    let (mut tail, mut head) = ((0, 0), (0, 0));
    let mut traversed: HashSet<(i32, i32)> = HashSet::new();

    traversed.insert(tail);
    let mut iter = contents.lines();
    while let Some(Ok(line)) = iter.next() {
        let mut words = line.split(" ");
        let dir = words.next().unwrap();
        let steps = words.next().unwrap().parse::<usize>().unwrap();
        let movement_fn = match dir {
            "L" => |pos: &mut (i32, i32)| pos.0 -= 1,
            "R" => |pos: &mut (i32, i32)| pos.0 += 1,
            "D" => |pos: &mut (i32, i32)| pos.1 -= 1,
            "U" => |pos: &mut (i32, i32)| pos.1 += 1,
            _ => panic!("bad dir"),
        };

        for _ in 0..steps {
            let head_last = head;
            movement_fn(&mut head);
            if i32::abs(head.0 - tail.0) > 1 || i32::abs(head.1 - tail.1) > 1 {
                tail = head_last;
                traversed.insert(tail);
            }
        }
    }
    traversed.len() as u32
}

fn part_two(input_file_path: &str) -> u32 {
    let file = File::open(input_file_path).expect("file not found");
    let contents = BufReader::new(file);
    let mut snake = vec![(0, 0); 10];
    let mut traversed: HashSet<(i32, i32)> = HashSet::new();

    traversed.insert((0, 0));
    let mut iter = contents.lines();
    while let Some(Ok(line)) = iter.next() {
        let mut words = line.split(" ");
        let dir = words.next().unwrap();
        let steps = words.next().unwrap().parse::<usize>().unwrap();
        let movement_fn = match dir {
            "L" => |pos: &mut (i32, i32)| pos.0 -= 1,
            "R" => |pos: &mut (i32, i32)| pos.0 += 1,
            "D" => |pos: &mut (i32, i32)| pos.1 -= 1,
            "U" => |pos: &mut (i32, i32)| pos.1 += 1,
            _ => panic!("bad dir"),
        };
        for _ in 0..steps {
            movement_fn(&mut snake[0]);

            //Blegh hate this nested match, but i don't want to fix it 
            for i in 1..10 {
                let parent = snake[i - 1];
                let curr = &mut snake[i];
                match parent.0 - curr.0 {
                    val if val <= -2 => {
                        curr.0 -= 1;
                        match parent.1 - curr.1 {
                            val if val > 0 => curr.1 += 1,
                            val if val < 0 => curr.1 -= 1,
                            _ => {}
                        }
                    }
                    val if val >= 2 => {
                        curr.0 += 1;
                        match parent.1 - curr.1 {
                            val if val > 0 => curr.1 += 1,
                            val if val < 0 => curr.1 -= 1,
                            _ => {}
                        }
                    }
                    _ => {}
                }
                match parent.1 - curr.1 {
                    val if val <= -2 => {
                        curr.1 -= 1;
                        match parent.0 - curr.0 {
                            val if val > 0 => curr.0 += 1,
                            val if val < 0 => curr.0 -= 1,
                            _ => {}
                        }
                    }
                    val if val >= 2 => {
                        curr.1 += 1;
                        match parent.0 - curr.0 {
                            val if val > 0 => curr.0 += 1,
                            val if val < 0 => curr.0 -= 1,
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
            traversed.insert(snake.last().unwrap().clone());
        }
    }

    traversed.len() as u32
}
