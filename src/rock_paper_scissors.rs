use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;


fn get_data() -> BufReader<File> {
    let file  = File::open("./data/rock_paper_scissors_input.txt").expect("Could not read the file.");
    BufReader::new(file)
}


pub fn main_part01() {
    let buf_reader = get_data();
    let mut score:i32 = 0;

    for line in buf_reader.lines() {
        let line = line.unwrap();
        let byte = line.as_bytes();
        let opponent = byte[0] - 65;
        let mut player = byte[2] - 88;

        if (opponent + 1) % 3 == player {
            score += 6;
        } else if opponent == player {
            score += 3;
        }

        score += (player + 1) as i32;
    }

    println!("Final player score: {}", score);
}


pub fn main_part02() {
    let buf_reader = get_data();
    let mut score:i32 = 0;

    for line in buf_reader.lines() {
        let line = line.unwrap();
        let byte = line.as_bytes();
        let opponent = byte[0] - 65;
        let mut player = byte[2] - 88;

        match player {
            2 => {
                player = (opponent + 1) % 3;
                score += 6;
            }
            1 => {
                player = opponent;
                score += 3;
            }
            _ => {
                player = (opponent + 2) % 3;
            }
        }

        score += (player + 1) as i32;
    }

    println!("Part 2 final score: {}", score);
}