use std::fs;

use day4::{part2, proceed_data};

fn main() {
    let input_data = fs::read_to_string("./input.txt").unwrap();
    let res = proceed_data(&input_data);
    println!("Result {res}");
    let res2 = part2(&input_data);
    println!("Result {res2}");
}
