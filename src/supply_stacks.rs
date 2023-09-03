use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;


use regex::Regex;


fn get_data() -> BufReader<File> {
    let file = File::open("./data/supply_stacks_input.txt").expect("Could not read the file.");
    BufReader::new(file)
}


fn parse_stacks(stack_data: &str) -> Vec<Vec<String>> {
    let stack_size = stack_data.chars().last().unwrap().to_digit(10).unwrap() as usize;
    let mut stacks : Vec<Vec<String>> = vec![vec![]; stack_size];

    for line in stack_data.lines() {
        let mut line = line.chars().peekable();

        let mut counter = 0;
        while line.peek().is_some() {
            let mut chunk: String = line.by_ref().take(4).collect();
            chunk = chunk.replace(" ", "");
            if chunk.starts_with("1") {
                break;
            }

            counter += 1;
            if chunk.len() > 0 {
                let chunk = chunk.strip_prefix("[").unwrap().strip_suffix("]").unwrap();
                stacks[counter - 1].insert(0, chunk.to_string());
            }

        }
    }

    stacks
}


fn parse_operations(operations_data: &str) -> Vec<(i32, i32, i32)>{
    let mut operations: Vec<(i32, i32, i32)> = vec![];

    let re = Regex::new(r"\d+").unwrap();
    for line in operations_data.lines() {
        let mut numbers = vec![];
        for capture in re.captures_iter(line) {
            let number: i32 = capture[0].parse().unwrap();
            numbers.push(number);
        }

        operations.push((numbers[0], numbers[1], numbers[2]));
    }

    operations
 }


pub fn main_part01() {
    let buf_reader = get_data();
    let  data = buf_reader.lines()
        .map(|line| line.unwrap_or_default())
        .collect::<Vec<String>>()
        .join("\n");
    let (stack_data, operations_data) = data.split_once("\n\n").unwrap();


    let mut stacks = parse_stacks(stack_data);
    let operations = parse_operations(operations_data);

    for moves in &operations {
        let (size, from, to) = moves;

        for _ in 0..*size {
            let element = stacks[*from as usize - 1].pop().unwrap();
            stacks[*to as usize - 1].push(element);
        }

    }

    let result: String = stacks.into_iter().map(|mut stack| stack.pop().unwrap()).collect();

    println!("{}", result);
}


pub fn main_part02() {
    let buf_reader = get_data();
    let  data = buf_reader.lines()
        .map(|line| line.unwrap_or_default())
        .collect::<Vec<String>>()
        .join("\n");
    let mut split = data.split("\n\n");
    let stack_data = split.next().unwrap();
    let operations_data = split.next().unwrap();


    let mut stacks = parse_stacks(stack_data);
    let operations = parse_operations(operations_data);

    for moves in &operations {
        let (size, from, to) = moves;

        let to_size = stacks[*to as usize - 1].len();
        for _ in 0..*size {
            let element = stacks[*from as usize - 1].pop().unwrap();
            stacks[*to as usize - 1].insert(to_size, element);
        }
    }


    let result: String = stacks.into_iter().map(|mut stack| stack.pop().unwrap()).collect();
    println!("{}", result);
}
