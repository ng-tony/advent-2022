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

    let mut forest: Vec<Vec<u8>> = vec![];
    let mut iter = contents.lines();
    while let Some(Ok(line)) = iter.next() {
        let mut row = vec![];
        for tree_char in line.chars() {
            let tree_height = tree_char.to_digit(10).unwrap();
            row.push(tree_height as u8);
        }
        forest.push(row);
    }
    let (length, width) = (forest.len(), forest[0].len());

    let mut visible_trees: HashSet<(usize, usize)> = HashSet::new();
    //Horizontal
    for y in 0..length {
        let mut curr_height: i32 = -1;
        for x in 0..width {
            let tree = forest[y][x] as i32;
            if curr_height < tree {
                curr_height = tree;
                visible_trees.insert((y, x));
            }
        }
        let max_height: i32 = curr_height;
        curr_height = -1;
        for x in (0..width).rev() {
            let tree = forest[y][x] as i32;
            if curr_height < tree {
                curr_height = tree;
                visible_trees.insert((y, x));
                //Early exit, since you know all trees from that side will be blocked
                if curr_height == max_height {
                    break;
                }
            }
        }
    }

    //Vertical
    for x in 0..width {
        let mut curr_height: i32 = -1;
        for y in 0..length {
            let tree = forest[y][x] as i32;
            if curr_height < tree {
                curr_height = tree;
                visible_trees.insert((y, x));
            }
        }
        let max_height: i32 = curr_height;
        curr_height = -1;
        for y in (0..length).rev() {
            let tree = forest[y][x] as i32;
            if curr_height < tree {
                curr_height = tree;
                visible_trees.insert((y, x));
                //Early exit, since you know all trees from that side will be blocked
                if curr_height == max_height {
                    break;
                }
            }
        }
    }
    visible_trees.len() as u32
}

fn part_two(input_file_path: &str) -> usize {
    let file = File::open(input_file_path).expect("file not found");
    let contents = BufReader::new(file);

    let mut forest: Vec<Vec<u8>> = vec![];
    let mut iter = contents.lines();
    while let Some(Ok(line)) = iter.next() {
        let mut row = vec![];
        for tree_char in line.chars() {
            let tree_height = tree_char.to_digit(10).unwrap();
            row.push(tree_height as u8);
        }
        forest.push(row);
    }
    let (length, width) = (forest.len(), forest[0].len());

    let mut visibility: Vec<Vec<Vec<usize>>> = vec![
        vec![
            vec![0; 4];
            width
        ];
        length
    ];

    let mut compute = |outer_iter, inner_iter: Box<dyn Iterator<Item = usize>>, outer_is_x, direction: usize| {
        let get_tree = |outer: usize, &inner: &usize| -> u8 {
            if outer_is_x {
                forest[outer][inner]
            } else {
                forest[inner][outer]
            }
        };

        fn get_visible<'a>(
            outer: usize,
            &inner: &usize,
            visibility: &'a mut Vec<Vec<Vec<usize>>>,
            outer_is_x: &bool,
        ) -> &'a mut Vec<usize> {
            if *outer_is_x {
                &mut visibility[outer][inner]
            } else {
                &mut visibility[inner][outer]
            }
        }

            let _inner_iter: Vec<usize> = inner_iter.collect();
        for outer in outer_iter {
            let mut row_visiblity = vec![1; 10];
            for inner in &_inner_iter {
                let tree = get_tree(outer, inner) as usize;
                let vis =  get_visible(outer, inner, &mut visibility, &outer_is_x);
                vis[direction]= row_visiblity[tree];
                //Update Visiblity
                for (i, val) in row_visiblity.iter_mut().enumerate().rev() {
                    if i > tree {
                        *val += 1;
                    } else {
                        *val = 1;
                    }
                }
            }
        }
    };
    compute(0..length, Box::new(1..width), false,0);
    compute(0..length, Box::new((0..width-1).rev()), false, 1);
    compute(0..width, Box::new(1..length), true, 2);
    compute(0..width, Box::new((0..length-1).rev()), true, 3);

    visibility
        .iter()
        .fold(0, |acc: usize, row: &Vec<Vec<usize>>| {
            acc.max(row.iter().fold(0, |acc, item| {
                acc.max(item.iter().fold(1, |acc, val| acc * val))
            }))
        })
}

