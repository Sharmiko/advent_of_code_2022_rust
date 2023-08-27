use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::collections::HashSet;


fn get_data() -> BufReader<File> {
    let file  = File::open("./data/rope_bridge_input.txt").expect("Could not read the file.");
    BufReader::new(file)
}


fn is_near_head(tail: &(i32, i32), head: &(i32, i32)) -> bool {
    (head.0 - tail.0).abs()  <= 1 && (head.1 - tail.1).abs() <= 1
}


pub fn main_part01() {
    let buf_reader = get_data();

    let mut head_pos = (1, 1);
    let mut tail_pos = (1, 1);
    let mut visited_positions: HashSet<(i32, i32)> = HashSet::new();
    visited_positions.insert(tail_pos);

    for line in buf_reader.lines() {
        let line = line.unwrap();
        let (direction, step_size) = line.split_once(" ").unwrap();
        let step_size = step_size.parse::<i32>().unwrap();
        match direction {
            "L" => {
                let mut prev_head = tail_pos;
                for _ in 0..step_size {
                    prev_head = head_pos;
                    head_pos.0 -= 1;
                    if !is_near_head(&tail_pos, &head_pos) {
                        tail_pos = prev_head;
                        visited_positions.insert(tail_pos);
                    }
                }
            },
            "R" => {
                let mut prev_head = tail_pos;
                for _ in 0..step_size {
                    prev_head = head_pos;
                    head_pos.0 += 1;
                    if !is_near_head(&tail_pos, &head_pos) {
                        tail_pos = prev_head;
                        visited_positions.insert(tail_pos);
                    }
                }
            },
            "U" => {
                let mut prev_head = tail_pos;
                for _ in 0..step_size {
                    prev_head = head_pos;
                    head_pos.1 += 1;
                    if !is_near_head(&tail_pos, &head_pos) {
                        tail_pos = prev_head;
                        visited_positions.insert(tail_pos);
                    }
                }
            },
            "D" =>  {
                let mut prev_head = tail_pos;
                for _ in 0..step_size {
                    prev_head = head_pos;
                    head_pos.1 -= 1;
                    if !is_near_head(&tail_pos, &head_pos) {
                        tail_pos = prev_head;
                        visited_positions.insert(tail_pos);
                    }
                }
            },
            _ => ()

        };

    }

    println!("Positions visited: {}", visited_positions.len());
}


pub fn main_part02() {
    todo!()
}
