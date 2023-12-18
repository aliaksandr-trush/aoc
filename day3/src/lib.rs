#[derive(Debug, Clone)]
struct Position {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone)]
struct Symbol {
    value: String,
    position: Position,
}

impl Symbol {
    fn new() -> Self {
        Symbol {
            value: String::new(),
            position: Position { x: 0, y: 0 },
        }
    }

    fn is_special_around(&self, specials: &Vec<Symbol>) -> bool {
        let x1 = match self.position.x {
            0 => 0,
            x => x - 1,
        };
        let x2 = self.position.x + self.value.len() + 1;
        let y1 = match self.position.y {
            0 => 0,
            y => y - 1,
        };
        let y2 = self.position.y + 1;
        for sym in specials {
            if x1 <= sym.position.x
                && sym.position.x <= x2
                && y1 <= sym.position.y
                && sym.position.y <= y2
            {
                return true;
            }
        }
        false
    }

    fn to_int(&self) -> u32 {
        self.value.parse().unwrap()
    }
}

// fn is_special(ch: &char) -> bool {
//     let specials = ['#', '$', '@', '%', '&', '*', '+', '-', '/', '='];
//     if specials.iter().any(|&c| c == *ch) {
//         return true;
//     }
//     false
// }

fn find_valid_numbers(input_data: &str) -> Vec<u32> {
    let mut specials: Vec<Symbol> = Vec::new();
    let mut numbers: Vec<Symbol> = Vec::new();

    for (i, line) in input_data.lines().enumerate() {
        let mut temp_symbol: Symbol = Symbol::new();
        for (j, ch) in line.chars().enumerate() {
            if ch == '.' {
                if !temp_symbol.value.is_empty() {
                    numbers.push(temp_symbol.clone());
                    temp_symbol = Symbol::new();
                }
            } else if ch.is_numeric() {
                if temp_symbol.value.is_empty() {
                    temp_symbol.position.x = j;
                    temp_symbol.position.y = i;
                }
                temp_symbol.value += &ch.to_string();
            } else {
                specials.push(Symbol {
                    value: ch.to_string(),
                    position: Position { x: j, y: i },
                });
                if !temp_symbol.value.is_empty() {
                    numbers.push(temp_symbol.clone());
                    temp_symbol = Symbol::new();
                };
            }
        }
        if !temp_symbol.value.is_empty() {
            numbers.push(temp_symbol.clone());
        }
    }
    let numbers_count = numbers.len();
    let special_count = specials.len();
    println!("Numbers: {numbers_count}, Specials: {special_count}");

    numbers
        .iter()
        .filter(|n| n.is_special_around(&specials))
        .map(|n| n.to_int())
        .collect()
}

pub fn proceed_data(input_data: &str) -> u32 {
    let numbers: Vec<u32> = find_valid_numbers(input_data);
    numbers.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init_tests() {
        let init_data = "
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        assert_eq!(proceed_data(init_data), 4361);
    }
}
