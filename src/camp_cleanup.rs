use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::collections::HashSet;


fn get_data() -> BufReader<File> {
    let file  = File::open("./data/camp_cleanup_input.txt").expect("Could not read the file.");
    BufReader::new(file)
}


fn parse_range(range_str: &str) -> HashSet<i32> {
    let mut range_parts = range_str.split("-").map(|s| s.parse::<i32>().unwrap());
    (range_parts.next().unwrap()..range_parts.next().unwrap() + 1).collect()
}


pub fn main_part01() {
    let buf_reader = get_data();

    let mut fully_contains: i32 = 0;
    for line in buf_reader.lines() {
        let line = line.unwrap();
        let mut parts= line.split(",");
        let first_elf = parts.next().unwrap();
        let second_elf = parts.next().unwrap();

        let first_elf_sections: HashSet<i32> = parse_range(first_elf);
        let second_elf_sections: HashSet<i32> = parse_range(second_elf);

        if first_elf_sections.is_subset(&second_elf_sections) || first_elf_sections.is_superset(&second_elf_sections) {
            fully_contains += 1;
        }
    }
    println!("Fully contains count: {}", fully_contains);
}


pub fn main_part02() {
    let buf_reader = get_data();

    let mut overlaps: i32 = 0;
    for line in buf_reader.lines() {
        let line = line.unwrap();
        let mut parts= line.split(",");
        let first_elf = parts.next().unwrap();
        let second_elf = parts.next().unwrap();

        let first_elf_sections: HashSet<i32> = parse_range(first_elf);
        let second_elf_sections: HashSet<i32> = parse_range(second_elf);

        if !first_elf_sections.is_disjoint(&second_elf_sections) {
            overlaps += 1;
        }
    }
    println!("Overlap count: {}", overlaps);
}
