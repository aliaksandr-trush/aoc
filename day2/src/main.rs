use std::{error::Error, fs};

use day2::{calculate_power, proceed_data, Color, Game};

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, world!");
    let mut rule: Game = Game::new(0);
    rule.insert(Color::Red, 12);
    rule.insert(Color::Green, 13);
    rule.insert(Color::Blue, 14);
    let input_data = fs::read_to_string("./input.txt")?;
    let res = proceed_data(&input_data, &rule)?;
    println!("Result: {}", res);
    let res_power = calculate_power(&input_data)?;
    println!("Power result: {}", res_power);
    Ok(())
}
