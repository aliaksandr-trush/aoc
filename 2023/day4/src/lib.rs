use std::{collections::HashMap, error::Error};

#[derive(Debug)]
struct Card {
    number: u32,
    valid_points: Vec<u32>,
    my_points: Vec<u32>,
}

fn parse_points(input: &str) -> Vec<u32> {
    input.trim().replace("  ", " ").split(" ").map(|x| x.parse::<u32>().unwrap()).collect()
}

impl Card {
    fn from(line: &str) -> Result<Self, Box<dyn Error>> {
        let (card_name, card_data) = line.split_once(":").unwrap();
        let card_number = card_name.split_once(" ").unwrap().1;
        println!("Card number {card_number}");
        let (valid_points, my_points) = card_data.split_once("|").unwrap();
        println!("Valid points {valid_points} and my points {my_points}");

        Ok(Card {
            number: card_number.trim().parse::<u32>()?,
            valid_points: parse_points(valid_points),
            my_points: parse_points(my_points),
        })
    }

    fn points(&self) -> u32 {
        let mut res: u32 = 0;
        for point in &self.my_points {
            if self.valid_points.iter().any(|i| i == point) {
                if res == 0 {
                    res = 1;
                } else {
                    res *= 2;
                }
            }
        }
        res
    }

    fn wins_number(&self) -> u32 {
        let mut res: u32 = 0;
        for point in &self.my_points {
            if self.valid_points.iter().any(|i| i == point) {
                res += 1;
            }
        }
        res
    }
}

pub fn proceed_data(input_data: &str) -> u32 {
    let data: Vec<Card> = input_data
        .lines()
        .map(|line| Card::from(line).unwrap())
        .collect();
    data.into_iter().map(|card| card.points()).into_iter().sum()
}

fn calculate_cards_number(
    input_data: &Vec<Card>,
    starting_card: &Card,
    cache: &mut HashMap<u32, u32>,
) -> u32 {
    let mut res: u32 = 1;
    let starting_number = starting_card.number;
    let starting_wins = starting_card.wins_number();
    println!("Starting card {starting_number} with wins {starting_wins}");
    if cache.contains_key(&starting_number) {
        return *cache.get(&starting_number).unwrap();
    }

    for card in input_data {
        let wins_number: u32 = card.wins_number();
        let card_number: u32 = card.number;
        if card_number > starting_number && card_number <= starting_number + starting_wins {
            // println!("Card number {card_number} Wins number: {wins_number}");
            println!("Proceeding card {card_number} {starting_wins} times");
            let child_data = calculate_cards_number(&input_data, &card, cache);

            res += child_data;
        }
    }
    cache.insert(starting_number, res);
    res
}

pub fn part2(input_data: &str) -> u32 {
    // number of wins = number of usages for next X cards
    // TODO: proceed with recursion
    // 30
    let cards: Vec<Card> = input_data
        .lines()
        .map(|line| Card::from(line).unwrap())
        .collect();
    // let first_card = cards.first().unwrap();
    // calculate_cards_number(&cards, first_card)
    let mut res: u32 = 0;
    let mut cache: HashMap<u32, u32> = HashMap::new();
    for card in &cards {
        res += calculate_cards_number(&cards, &card, &mut cache);
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init() {
        let input_data = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        assert_eq!(proceed_data(input_data), 13);
    }

    #[test]
    fn test_part2() {
        let input_data = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        assert_eq!(part2(input_data), 30);
    }

    #[test]
    fn test_from() {
        let input_line = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
        Card::from(input_line).expect("Invalid data");
    }
}
