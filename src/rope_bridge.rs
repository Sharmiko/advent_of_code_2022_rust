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


fn calculate_tail_movement(tail: &(i32, i32), head: &(i32, i32)) -> (i32, i32) {
    let mut diff = (head.0 - tail.0, head.1 - tail.1);

    diff.0 = diff.0.signum();
    diff.1 = diff.1.signum();

    diff
}


fn  get_operation(direction: &str) -> (Option<&str>, Option<i32>) {
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

    (axis, increment)
}


fn mark_visited_positions(
    axis: &str,
    increment: i32,
    step_size: i32,
    head_pos: &mut (i32, i32),
    tail_pos: &mut (i32, i32),
    visited_positons: &mut HashSet<(i32, i32)>
) {
    for _ in 0..step_size {
        match axis {
            "x" => head_pos.0 += increment,
            "y" => head_pos.1 += increment,
            _ => ()
        };
        if !is_near_head(&tail_pos, &head_pos) {
            let diff = calculate_tail_movement(&tail_pos, &head_pos);
            tail_pos.0 += diff.0;
            tail_pos.1 += diff.1;
            visited_positons.insert(tail_pos.clone());
        }
    }
}


fn mark_visited_positions_v2(
    axis: &str,
    increment: i32,
    step_size: i32,
    rope_knots: &mut Vec<(i32 , i32)>,
    visited_positions: &mut HashSet<(i32, i32)>
) {

    for _ in 0..step_size {

        match axis {
            "x" => rope_knots[0].0 += increment,
            "y" => rope_knots[0].1 += increment,
            _ => ()
        };

        for idx in 1..rope_knots.len() {
            if !is_near_head(&rope_knots[idx], &rope_knots[idx - 1]) {
                let diff = calculate_tail_movement(&rope_knots[idx], &rope_knots[idx - 1]);
                rope_knots[idx].0 += diff.0;
                rope_knots[idx].1 += diff.1;
                if idx == 9 {
                    visited_positions.insert(rope_knots[idx]);
                }
            }
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

        let (axis, increment) = get_operation(direction);

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
    let buf_reader = get_data();

    let mut rope_knots: Vec<(i32, i32)>  = vec![(1, 1); 10];
    let mut visited_positions: HashSet<(i32, i32)> = HashSet::new();
    visited_positions.insert(*rope_knots.last().unwrap());

    for line in buf_reader.lines() {
        let line = line.unwrap();
        let (direction, step_size) = line.split_once(" ").unwrap();
        let step_size = step_size.parse::<i32>().unwrap();

        let (axis, increment) = get_operation(direction);

        mark_visited_positions_v2(
            axis.unwrap(),
            increment.unwrap(),
            step_size,
            &mut rope_knots,
            &mut visited_positions
        );
    }

    println!("Positions visited: {}", visited_positions.len());
}
