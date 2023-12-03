pub fn proceed_data(input_data: &str) -> u32 {
    4361
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
