use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;


pub fn main() {
    let file  = File::open("./data/rock_paper_scissors_input.txt").expect("Could not read the file.");
    let buf_reader = BufReader::new(file);

    let mut score:i32 = 0;
    let mut part2_score :i32 = 0;
    for line in buf_reader.lines() {
        let line = line.unwrap();
        let byte = line.as_bytes();
        let opponent = byte[0] - 65;
        let mut player = byte[2] - 88;

        // if (opponent + 1) % 3 == player {
        //     score += 6;
        // } else if opponent == player {
        //     score += 3;
        // }

        println!("{} {}", opponent, player);
        match player {
            2 => {
                player = (opponent + 1) % 3;
                part2_score += 6;
                println!("A -- {}", player);
            }
            1 => {
                player = opponent;
                part2_score += 3;
                println!("B -- {}", player);
            }
            _ => {
                player = (opponent + 2) % 3;
                println!("C -- {}", player);
            }
        }


        //score += (player + 1) as i32;
        part2_score += (player + 1) as i32;
        println!("SCOE {}", part2_score);
    }

    println!("Final player score: {}", score);
    println!("Part 2 final score: {}", part2_score);
}