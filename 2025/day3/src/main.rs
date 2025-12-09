use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

fn main() -> io::Result<()> {
    let input = read_input()?;

    let p1_result = p1(&input);

    println!("Part 1 result: {p1_result}");

    let p2_result = p2(&input);

    println!("Part 2 result: {p2_result}");

    Ok(())
}

fn read_input() -> io::Result<Vec<String>> {
    let file = File::open("input/raw.txt")?;
    BufReader::new(file).lines().collect::<Result<_, _>>()
}

fn p1(input: &[String]) -> u64 {
    input
        .iter()
        .map(|line| Bank::from(line).get_largest_joltage(2))
        .sum()
}

fn p2(input: &[String]) -> u64 {
    input
        .iter()
        .map(|line| Bank::from(line).get_largest_joltage(12))
        .sum()
}

struct Bank {
    batteries: Vec<u8>,
}

impl From<&String> for Bank {
    fn from(s: &String) -> Self {
        let batteries = s.chars().map(|char| char as u8 - b'0').collect::<Vec<u8>>();

        Bank { batteries }
    }
}

impl Bank {
    fn get_largest_joltage(&self, size: usize) -> u64 {
        let mut largest_joltage = vec![0u8; size];

        for window in self.batteries.windows(size) {
            for (i, &digit) in window.iter().enumerate() {
                if digit > largest_joltage[i] {
                    // update digit and all following digits
                    largest_joltage[i..size].copy_from_slice(&window[i..size]);
                    break;
                }
            }
        }

        largest_joltage
            .iter()
            .fold(0u64, |acc, &digit| acc * 10 + u64::from(digit))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn largest_joltage_is_at_the_beginning() {
        let bank = Bank::from(&"987654321111111".to_string());

        assert_eq!(bank.get_largest_joltage(2), 98);
    }

    #[test]
    fn largest_joltage_is_at_the_end() {
        let bank = Bank::from(&"234234234234278".to_string());

        assert_eq!(bank.get_largest_joltage(2), 78);
    }

    #[test]
    fn largest_joltage_is_in_the_middle() {
        let bank = Bank::from(&"818181911112111".to_string());

        assert_eq!(bank.get_largest_joltage(2), 92);
    }

    #[test]
    fn largest_joltage_is_two_digits_at_the_edge() {
        let bank = Bank::from(&"81111111111119".to_string());

        assert_eq!(bank.get_largest_joltage(2), 89);
    }

    #[test]
    fn p1_basic_usecase() {
        let banks = [
            "987654321111111".to_string(),
            "81111111111119".to_string(),
            "234234234234278".to_string(),
            "818181911112111".to_string(),
        ];

        assert_eq!(p1(&banks), 357);
    }

    #[test]
    fn largest_joltage_with_twelve_digits_at_the_beginning() {
        let bank = Bank::from(&"987654321111111".to_string());

        assert_eq!(bank.get_largest_joltage(12), 987_654_321_111);
    }

    #[test]
    fn largest_joltage_with_twelve_digits_without_some_1s() {
        let bank = Bank::from(&"811111111111119".to_string());

        assert_eq!(bank.get_largest_joltage(12), 811_111_111_119);
    }

    #[test]
    fn largest_joltage_with_twelve_digits_excluding_some_digits() {
        let bank = Bank::from(&"234234234234278".to_string());

        assert_eq!(bank.get_largest_joltage(12), 434_234_234_278);
    }

    #[test]
    fn largest_joltage_with_twelve_digits_excluding_some_1s_in_the_middle() {
        let bank = Bank::from(&"818181911112111".to_string());

        assert_eq!(bank.get_largest_joltage(12), 888_911_112_111);
    }

    #[test]
    fn p2_basic_usecase() {
        let banks = [
            "987654321111111".to_string(),
            "811111111111119".to_string(),
            "234234234234278".to_string(),
            "818181911112111".to_string(),
        ];

        assert_eq!(p2(&banks), 3_121_910_778_619);
    }
}
