use std::{collections::HashMap, error::Error};

#[derive(Debug, Hash, PartialEq, Eq)]
pub enum Color {
    Red,
    Green,
    Blue,
}

impl Color {
    fn from(name: &str) -> Result<Self, &'static str> {
        match name {
            "red" => Ok(Color::Red),
            "green" => Ok(Color::Green),
            "blue" => Ok(Color::Blue),
            _ => Err("Unable to parse value"),
        }
    }
}

#[derive(Debug)]
pub struct Game {
    number: u32,
    results: HashMap<Color, u32>,
}

impl Game {
    pub fn new(number: u32) -> Self {
        let mut results: HashMap<Color, u32> = HashMap::new();
        results.insert(Color::Red, 0);
        results.insert(Color::Green, 0);
        results.insert(Color::Blue, 0);
        Self { number, results }
    }

    pub fn insert(&mut self, color: Color, number: u32) {
        if self.results[&color] < number {
            self.results.insert(color, number);
        }
    }
}

fn calculate_cubes(input_data: &str) -> Result<Vec<Game>, Box<dyn Error>> {
    let mut full_cal: Vec<Game> = Vec::new();

    for line in input_data.lines() {
        println!("Parsing line: {}", line);
        let (game_str, cubes_str) = line.split_once(":").unwrap();
        let game_number = game_str.split_once(" ").unwrap().1.parse::<u32>().unwrap();
        println!("Game number: {}", game_number);
        let mut game = Game::new(game_number);
        for game_str in cubes_str.split(";") {
            println!("Game str: {}", game_str);
            for case in game_str.trim().split(",") {
                let (num, col) = case.trim().split_once(" ").unwrap();
                let col_num = num.trim().parse::<u32>().unwrap();
                let color = Color::from(col)?;
                game.insert(color, col_num);
            }
        }
        full_cal.push(game);
    }
    Ok(full_cal)
}

pub fn proceed_data(input_data: &str, rule: &Game) -> Result<u32, Box<dyn Error>> {
    let cubes_count: Vec<Game> = calculate_cubes(input_data)?;
    println!("Calculatoins: {:?}", cubes_count);
    let mut valid_games: Vec<u32> = Vec::new();
    for game in cubes_count {
        if game.results[&Color::Red] <= rule.results[&Color::Red]
            && game.results[&Color::Blue] <= rule.results[&Color::Blue]
            && game.results[&Color::Green] <= rule.results[&Color::Green]
        {
            valid_games.push(game.number);
        }
    }
    println!("Valid games: {:?}", valid_games);
    Ok(valid_games.iter().sum())
}

pub fn calculate_power(input_data: &str) -> Result<u32, Box<dyn Error>> {
    let cubes_count = calculate_cubes(input_data)?;
    let mut res = 0;
    for game in cubes_count {
        res += game.results[&Color::Red] * game.results[&Color::Green] * game.results[&Color::Blue];
    }
    Ok(res)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn init_test() {
        let input_data = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
            .to_string();
        let mut rule = Game::new(0);
        rule.insert(Color::Red, 12);
        rule.insert(Color::Green, 13);
        rule.insert(Color::Blue, 13);
        assert_eq!(proceed_data(&input_data, &rule).unwrap(), 8);
    }

    #[test]
    fn powert_test() {
        let input_data = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!(calculate_power(&input_data).unwrap(), 2286);
    }
}
