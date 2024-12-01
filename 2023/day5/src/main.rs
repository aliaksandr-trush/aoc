use std::fs;

use day5::proceed_data;

fn main() {
    println!("Hello, world!");
    let input_data = fs::read_to_string("./input.txt").unwrap();
    let res = proceed_data(&input_data);
    println!("Results {res}");
}
