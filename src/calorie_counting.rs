use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;


fn get_data() -> BufReader<File> {
    let file = File::open("./data/calorie_counting_input.txt").expect("Could not read the file.");
     BufReader::new(file)
}


pub fn main_part01() {
    let buf_reader = get_data();

    let mut current: i32 = 0;
    let mut max_calories: i32 = 0;
    for line in buf_reader.lines() {
        let calorie = line.unwrap();
        if calorie.len() == 0 {
            if current > max_calories {
                max_calories = current;
            }
            current = 0;
        } else {
            current += calorie.parse::<i32>().unwrap();
        }
    }

    println!("Max calories {}", max_calories);
}


pub fn main_part02() {
    let buf_reader = get_data();

    let mut top_three_calories = vec![0; 3];
    let mut current: i32 = 0;

    for line in buf_reader.lines() {
        let calorie = line.unwrap();
        if calorie.len() == 0 {
            for idx in 0..top_three_calories.len() {
                if current > top_three_calories[idx] {
                    top_three_calories[idx] = current;
                    top_three_calories.sort();
                    break
                }
            }
            current = 0;
        } else {
            current += calorie.parse::<i32>().unwrap();
        }
    }

    println!("Sum of top 3 calories: {}", top_three_calories.iter().sum::<i32>());
}