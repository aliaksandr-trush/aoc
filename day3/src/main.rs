use day3::proceed_data;
use std::fs;

fn main() {
    println!("Hello, world!");
    let input_data = fs::read_to_string("./input.txt").unwrap();
    let res = proceed_data(&input_data);
    println!("Day3 result: {}", res);
}
