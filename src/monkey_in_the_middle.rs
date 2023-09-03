use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::cell::RefCell;
use std::collections::HashMap;


#[derive(Debug)]
struct Monkey {
    items: RefCell<Vec<i64>>,
    operation_type: char,
    operation_value: Option<i64>,
    modulo_divisor: i64,
    positive_target: i32,
    negative_target: i32
}


impl Monkey {

    fn from_string_data(monkey_data: &str) -> Self {
        let mut lines = monkey_data.split("\n").skip(1);
        let items = lines.next().unwrap().strip_prefix("  Starting items: ").unwrap();
        let operations = lines.next().unwrap().strip_prefix("  Operation: new = old ").unwrap();
        let modulo_divisor = lines.next().unwrap().strip_prefix("  Test: divisible by ").unwrap();
        let positive_target = lines.next().unwrap().strip_prefix("    If true: throw to monkey ").unwrap();
        let negative_target = lines.next().unwrap().strip_prefix("    If false: throw to monkey ").unwrap();

        let (operation_type, operation_value) = operations.split_once(" ").unwrap();
        let operation_value = operation_value.parse::<i64>();

        Monkey {
            items: RefCell::new(items.split(", ").into_iter().map(|e| e.parse::<i64>().unwrap()).collect()),
            operation_type: operation_type.chars().nth(0).unwrap(),
            operation_value: if operation_value.is_ok() { Some(operation_value.unwrap()) } else { None },
            modulo_divisor: modulo_divisor.parse::<i64>().unwrap(),
            positive_target: positive_target.parse::<i32>().unwrap(),
            negative_target: negative_target.parse::<i32>().unwrap()
        }
    }

    fn _worry_level(&self, item: i64) -> i64 {
        let mut worry_level = 0;
        let operation_value = if self.operation_value.is_some() { self.operation_value.unwrap() } else { item };
        match self.operation_type {
            '*' => {
                worry_level = item * operation_value;
            },
            '+' => {
                worry_level = item + operation_value
            }
            _  => panic!("Received invalid operator.")
        }

        worry_level
    }

    fn worry_level(&self, item: i64) -> i64{
        self._worry_level(item) / 3
    }

    fn worry_level_v2(&self, item: i64) -> i64 {
        self._worry_level(item)
    }

    fn target_monkey(&self, worry_level: i64) -> usize {
        if worry_level % self.modulo_divisor == 0 {
            return self.positive_target as usize;
        }

        self.negative_target as usize
    }
}


fn get_data() -> BufReader<File> {
    let file  = File::open("./data/monkey_in_the_middle_input.txt").expect("Could not read the file.");
    BufReader::new(file)
}


fn parse_monkeys() -> Vec<Monkey> {
    let buf_reader = get_data();
    let  data = buf_reader.lines()
        .map(|line| line.unwrap_or_default())
        .collect::<Vec<String>>()
        .join("\n");

    let mut monkeys: Vec<Monkey> = vec![];
    for monkey_data in data.split("\n\n") {
        let monkey = Monkey::from_string_data(monkey_data);
        monkeys.push(monkey);
    }

    monkeys
}


pub fn main_part01() {
    let monkeys = parse_monkeys();

    let round_count = 20;
    let mut inspection_count: HashMap<i32, i64> = HashMap::new();
    let monkey_count = monkeys.len();

    for _ in 0..round_count {
        for i in 0..monkey_count {
            while let Some(item) = monkeys[i].items.borrow_mut().pop() {
                let worry_level = monkeys[i].worry_level(item);
                let entry = inspection_count.entry(i as i32).or_insert(0);
                *entry += 1;
                let target_monkey = monkeys[i].target_monkey(worry_level);
                monkeys[target_monkey].items.borrow_mut().push(worry_level);
            }
        }
    }

    let mut values: Vec<&i64> = inspection_count.values().collect();
    values.sort();

    let monkey_business = values[values.len() - 1] * values[values.len() - 2];

    println!("{:?}", monkey_business);
}


pub fn main_part02() {
    let monkeys = parse_monkeys();

    let round_count = 10000;
    let mut inspection_count: HashMap<i32, i64> = HashMap::new();
    let monkey_count = monkeys.len();

    let mut super_modulo = 1;
    for monkey in &monkeys {
        super_modulo *= monkey.modulo_divisor;
    }

    for _ in 0..round_count {
        for i in 0..monkey_count {
            while let Some(item) = monkeys[i].items.borrow_mut().pop() {
                let worry_level = monkeys[i].worry_level_v2(item) % super_modulo;
                let entry = inspection_count.entry(i as i32).or_insert(0);
                *entry += 1;
                let target_monkey = monkeys[i].target_monkey(worry_level);
                monkeys[target_monkey].items.borrow_mut().push(worry_level);
            }
        }
    }

    let mut values: Vec<&i64> = inspection_count.values().collect();
    values.sort();

    let monkey_business = values[values.len() - 1] * values[values.len() - 2];

    println!("{:?}", monkey_business);
}
