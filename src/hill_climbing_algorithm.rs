use std::fs::File;
use std::cmp::Ordering;
use std::io::BufReader;
use std::io::prelude::*;
use std::collections::{HashSet, BinaryHeap};


fn get_data() -> BufReader<File> {
    let file = File::open("./data/hill_climbing_algorithm_input.txt").expect("Could not read the file.");
    BufReader::new(file)
}


#[derive(PartialEq, Eq, Clone, Copy)]
struct Node {
    position: (usize, usize),
    cost: u32
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}


fn find_neighbours(grid: &Vec<Vec<i32>>, i: usize, j: usize) -> Vec<(usize, usize)> {
    let mut result: Vec<(usize, usize)> = vec![];
    if j > 0 {
        result.push((i, j - 1));
    }
    if j < grid[0].len() - 1 {
        result.push((i, j + 1));
    }
    if i > 0  {
        result.push((i - 1, j));
    }
    if i < grid.len() - 1 {
        result.push((i + 1, j));
    }

    result
}


fn parse_grid(buf_reader: BufReader<File>) -> (Vec<Vec<i32>>, (usize, usize), (usize, usize)) {
    let mut grid: Vec<Vec<i32>> = vec![];
    let mut start_pos = (0, 0);
    let mut end_pos = (0, 0);

    for (row, line) in buf_reader.lines().enumerate() {
        let line = line.unwrap();
        let mut temp: Vec<i32> = vec![];
        for (col, mut char) in line.chars().enumerate() {
            match char {
                'S' => {
                    start_pos = (row, col);
                    char = 'a';
                },
                'E' => {
                    end_pos = (row, col);
                    char = 'z';
                },
                _ => ()
            }
            temp.push((char as u32) as i32);
        }
        grid.push(temp);
    }

    (grid, start_pos, end_pos)
}


pub fn main_part01() {
    let buf_reader = get_data();
    let (grid, start_pos, end_pos) = parse_grid(buf_reader);


    let mut pq =  BinaryHeap::new();
    let mut visited = HashSet::new();

    pq.push(Node {
        cost: 0,
        position: start_pos
    });
    visited.insert(start_pos);


    let mut fewest_steps = 0;
    while let Some(Node { position, cost }) = pq.pop() {
        if position == end_pos {
            fewest_steps = cost;
            break
        }

        let current_height = grid[position.0][position.1];
        let neighbours = find_neighbours(
            &grid,
            position.0,
            position.1
        );

        let candidates: Vec<&(usize, usize)> = neighbours
            .iter()
            .filter(|position| {
                let height = grid[position.0][position.1];
                height <= current_height || height == current_height + 1
            })
            .collect();

        for candidate in candidates {
            if visited.insert(*candidate) {
                pq.push(Node {
                    cost: cost + 1,
                    position: *candidate
                })
            }
        }

    }

    println!("Fewest steps to reach the goal: {}", fewest_steps);
}


pub fn main_part02() {
    let buf_reader = get_data();
    let (grid, start_pos, end_pos) = parse_grid(buf_reader);


    let mut pq =  BinaryHeap::new();
    let mut visited = HashSet::new();

    pq.push(Node {
        cost: 0,
        position: end_pos
    });
    visited.insert(start_pos);


    let mut fewest_steps = 0;
    while let Some(Node { position, cost }) = pq.pop() {
        let current_height = grid[position.0][position.1];
        if current_height == 97 {
            fewest_steps = cost;
            break;
        }

        let neighbours = find_neighbours(
            &grid,
            position.0,
            position.1
        );
        let candidates: Vec<&(usize, usize)> = neighbours
            .iter()
            .filter(|position| {
                let height = grid[position.0][position.1];
                height >= current_height || height == current_height - 1
            })
            .collect();

        for candidate in candidates {
            if visited.insert(*candidate) {
                pq.push(Node {
                    cost: cost + 1,
                    position: *candidate
                })
            }
        }

    }

    println!("Fewest steps to reach the goal: {}", fewest_steps);
}


