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
        let bytes = line.as_bytes();
        let mid_point = line.len() / 2;
        let common_items: HashSet<u8> = bytes.iter().take(mid_point).cloned().collect();

        for &byte in bytes.iter().skip(mid_point) {
            if common_items.contains(&byte) {
                priorities_sum += if byte > 96 { byte - 96 } else { byte - 38} as u32;
                break;
            }
        }

    }
    println!("Priorities sum: {}", priorities_sum);
}


pub fn main_part02() {
    let buf_reader = get_data();

    let mut priorities_sum: u32 = 0;
    let mut common_items  = HashSet::new();
    let mut second_items = HashSet::new();
    for (idx, line) in buf_reader.lines().enumerate() {
        if idx % 3  == 0 {
            common_items = line.unwrap().as_bytes().iter().cloned().collect();
        } else if idx % 3 == 1 {
            let line = line.unwrap();
            let bytes = line.as_bytes();
            for byte in bytes.iter() {
                if common_items.contains(&byte) {
                    second_items.insert(byte.clone());
                }
            }
        } else if idx % 3 == 2 {
            let line = line.unwrap();
            let bytes = line.as_bytes();
            for &byte in bytes.iter() {
                if second_items.contains(&byte) {
                    priorities_sum += if byte > 96 { byte - 96 } else { byte - 38} as u32;
                    common_items.clear();
                    second_items.clear();
                    break;
                }
            }
        }
    }

    println!("Priorities sum: {}", priorities_sum);
}
