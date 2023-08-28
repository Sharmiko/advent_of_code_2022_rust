use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;


fn get_data() -> BufReader<File> {
    let file  = File::open("./data/cathode_ray_tube_input.txt").expect("Could not read the file.");
    BufReader::new(file)
}


pub fn main_part01() {
    let buf_reader = get_data();

    let mut sum = 1;
    let mut cycle_count = 1;
    let mut signal_strengths: Vec<i32> = Vec::new();
    let mut interested_cycle = 20;

    for row in buf_reader.lines() {
        let row = row.unwrap();

        cycle_count += 1;
        if cycle_count % interested_cycle == 0{
            signal_strengths.push(sum * interested_cycle);
            interested_cycle += 40;
        }

        if row.starts_with("addx") {
            let value = row.strip_prefix("addx ").unwrap();
            sum += value.parse::<i32>().unwrap();
            cycle_count += 1;
            if cycle_count % interested_cycle == 0{
                signal_strengths.push(sum * interested_cycle);
                interested_cycle += 40;
            }
        }
    }

    println!("{:?}", signal_strengths.iter().sum::<i32>());
}


pub fn main_part02() {
    todo!()
}
