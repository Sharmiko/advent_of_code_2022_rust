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

fn mark_visited_positions(
    axis: &str,
    increment: i32,
    step_size: i32,
    head_pos: &mut (i32, i32),
    tail_pos: &mut (i32, i32),
    visited_positons: &mut HashSet<(i32, i32)>
) {
    let mut prev_head = tail_pos.clone();
    for _ in 0..step_size {
        prev_head = *head_pos;
        match axis {
            "x" => head_pos.0 += increment,
            "y" => head_pos.1 += increment,
            _ => ()
        };
        if !is_near_head(&tail_pos, &head_pos) {
            *tail_pos = prev_head;
            visited_positons.insert(tail_pos.clone());
        }
    }
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

        let mut axis = None;
        let mut increment = None;
        match direction {
            "L" => {
                axis = Some("x");
                increment = Some(-1);
            },
            "R" => {
                axis = Some("x");
                increment = Some(1);
            },
            "U" => {
                axis = Some("y");
                increment = Some(1);
            },
            "D" =>  {
                axis = Some("y");
                increment = Some(-1);
            },
            _ => ()
        };


        mark_visited_positions(
            axis.unwrap(),
            increment.unwrap(),
            step_size,
            &mut head_pos,
            &mut tail_pos,
            &mut visited_positions
        );
    }

    println!("Positions visited: {}", visited_positions.len());
}


pub fn main_part02() {
    todo!()
}
