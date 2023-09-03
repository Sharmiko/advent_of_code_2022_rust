use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::collections::HashMap;


fn get_data() -> BufReader<File> {
    let file = File::open("./data/no_space_left_on_device_input.txt").expect("Could not read the file.");
    BufReader::new(file)
}


fn calculate_dir_sizes() -> HashMap<String, usize> {
    let buf_reader = get_data();
    let mut lines = buf_reader.lines();


    let mut stack: Vec<String> = vec![];
    let mut dir_size: HashMap<String, usize> = HashMap::new();
    while let Some(line) = lines.next() {
        let line = line.unwrap();

        if !line.starts_with("$") && !line.starts_with("dir") {
            let (file_size, _) = line.split_once(" ").unwrap();
            let file_size = file_size.parse::<usize>().unwrap();
            for dir in &stack {
                let size = dir_size.entry(dir.clone()).or_insert(0);
                *size += file_size;
            }
        } else if line.starts_with("$ cd "){
            let dest = line.strip_prefix("$ cd ").unwrap().to_string();
            if dest == ".." {
                stack.pop();
            } else {
                let path =  if stack.len() > 0 { format!("{}_{}", stack.last().unwrap(), dest) } else { dest } ;
                stack.push(path);
            }
        }
    }

    dir_size
}


pub fn main_part01() {
    let dir_size = calculate_dir_sizes();
    let sum: usize = dir_size.values().into_iter().filter(|&e| *e <= 100000).sum();
    println!("{:?}", sum);
}


pub fn main_part02() {
    let dir_size = calculate_dir_sizes();

    let needed_size = 30000000 - (70000000 - dir_size.get("/").unwrap());
    let mut smallest_to_delete: usize = usize::MAX;
    for val in dir_size.values() {
        if *val >= needed_size && *val < smallest_to_delete {
            smallest_to_delete = *val;
        }
    }

    println!("{:?}", smallest_to_delete);
}