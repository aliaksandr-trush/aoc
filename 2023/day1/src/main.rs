use std::{error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let res = find_result(&fs::read_to_string("./input.txt")?);
    println!("Result: {}", res);
    Ok(())
}

pub fn find_result(input: &str) -> u32 {
    let mut res: u32 = 0;
    for line in input.lines() {
        let line_number = find_numbers(&replace_words_by_digits(line));
        res += line_number;
    }
    res
}

pub fn replace_words_by_digits(input: &str) -> String {
    input
        .replace("one", "o1e")
        .replace("two", "t2o")
        .replace("three", "th3ree")
        .replace("four", "fo4ur")
        .replace("five", "fi5ve")
        .replace("six", "si6x")
        .replace("seven", "sev7en")
        .replace("eight", "eigh8t")
        .replace("nine", "ni9ne")
}

fn find_numbers(line: &str) -> u32 {
    let mut first: Option<char> = None;
    let mut last: Option<char> = None;
    for ch in line.chars() {
        if ch.is_numeric() {
            if first.is_none() {
                first = Some(ch);
            }
            last = Some(ch);
        }
    }
    let res = String::from(first.unwrap()) + &last.unwrap().to_string();
    println!("Line {} number: {}", line, res);
    res.parse().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base_test() {
        let input_string = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        assert_eq!(find_result(input_string), 142);
    }

    #[test]
    fn real_numbers() {
        let input_string = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        assert_eq!(find_result(input_string), 281);
    }

    #[test]
    fn tricky_cases() {
        let input_string = "eighthree
sevenine";
        assert_eq!(find_result(input_string), 83 + 79);
    }
}
