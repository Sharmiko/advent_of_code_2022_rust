use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::collections::HashSet;


fn get_data() -> BufReader<File> {
    let file  = File::open("./data/rucksack_reorganization_input.txt").expect("Could not read the file.");
    BufReader::new(file)
}


pub fn main_part01() {
    let buf_reader = get_data();

    let mut priorities_sum: u32 = 0;
    for line in buf_reader.lines() {
        let line = line.unwrap();
        let one_part_size = line.len() / 2;
        let bytes = line.as_bytes();
        let mut common_items = HashSet::new();

        for idx in 0..line.len() {
            if idx < one_part_size {
                common_items.insert(bytes[idx]);
            } else {
                if common_items.contains(&bytes[idx]) {
                    if bytes[idx] > 96 {
                        priorities_sum += (bytes[idx] - 96) as u32;
                    } else {
                        priorities_sum += (bytes[idx] - 38) as u32;
                    }
                    break;
                }
            }
        }

    }
    println!("Priorities sum: {}", priorities_sum);
}
