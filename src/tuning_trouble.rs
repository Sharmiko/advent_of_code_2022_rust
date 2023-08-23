use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::collections::HashSet;


fn get_data() -> BufReader<File> {
    let file = File::open("./data/tuning_trouble_input.txt").expect("Could not read the file.");
    BufReader::new(file)
}


pub fn find_market(marker_length: usize) {
    let mut buf_reader = get_data();
    let mut data = String::new();
    buf_reader.read_to_string(&mut data).expect("Could not read to string.");

    let bytes = data.as_bytes();
    let mut char_set: HashSet<&u8> = HashSet::new();
    for (idx, byte) in bytes.into_iter().take(data.len() - marker_length).enumerate() {
        char_set.insert(byte);
        for i in 1..marker_length {
            char_set.insert(&bytes[idx + i]);
        }

        if char_set.len() == marker_length {
            println!("Total characters processed: {}", idx + marker_length);
            break
        } else {
            char_set.clear();
        }
    }
}


pub fn main_part01() {
    find_market(4);
}

pub fn main_part02() {
    find_market(14);
}
