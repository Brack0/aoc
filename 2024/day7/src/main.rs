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
        .map(Equation::from)
        .map(|eq| eq.with_operations(vec![Operation::Add, Operation::Multiply]))
        .filter(Equation::can_be_solved)
        .map(|eq| eq.result)
        .sum()
}

fn p2(input: &[String]) -> u64 {
    input
        .iter()
        .map(Equation::from)
        .map(|eq| eq.with_operations(vec![Operation::Add, Operation::Multiply, Operation::Concat]))
        .filter(Equation::can_be_solved)
        .map(|eq| eq.result)
        .sum()
}

enum Operation {
    Add,
    Multiply,
    Concat,
}

impl Operation {
    fn apply(&self, a: u64, b: u64) -> u64 {
        match self {
            Operation::Add => a + b,
            Operation::Multiply => a * b,
            Operation::Concat => {
                let mut b_copy = b;
                let mut multiplier = 1;
                while b_copy > 0 {
                    multiplier *= 10;
                    b_copy /= 10;
                }
                a * multiplier + b
            }
        }
    }
}

struct Equation {
    result: u64,
    numbers: Vec<u64>,
    operations: Vec<Operation>,
}

impl From<&String> for Equation {
    fn from(line: &String) -> Self {
        let mut parts = line.split(':');

        let result: u64 = parts
            .next()
            .unwrap()
            .trim()
            .parse()
            .expect("Failed to parse result");

        let numbers: Vec<u64> = parts
            .next()
            .unwrap()
            .split_whitespace()
            .map(|num_str| num_str.parse().expect("Failed to parse number"))
            .collect();
        let operations = vec![];

        Equation {
            result,
            numbers,
            operations,
        }
    }
}

impl Equation {
    fn with_operations(mut self, operations: Vec<Operation>) -> Self {
        self.operations = operations;
        self
    }

    fn can_be_solved(&self) -> bool {
        self.can_be_solved_backtrack(0, 0)
    }

    fn can_be_solved_backtrack(&self, value: u64, index: usize) -> bool {
        if value > self.result {
            return false;
        }
        if index >= self.numbers.len() {
            return value == self.result;
        }

        let current_number = self.numbers[index];
        self.operations.iter().any(|operation| {
            self.can_be_solved_backtrack(operation.apply(value, current_number), index + 1)
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn p1_basic_usecase() {
        let input = vec![
            String::from("190: 10 19"),
            String::from("3267: 81 40 27"),
            String::from("83: 17 5"),
            String::from("156: 15 6"),
            String::from("7290: 6 8 6 15"),
            String::from("161011: 16 10 13"),
            String::from("192: 17 8 14"),
            String::from("21037: 9 7 18 13"),
            String::from("292: 11 6 16 20"),
        ];

        let result = p1(&input);

        assert_eq!(result, 3749);
    }

    #[test]
    fn p2_basic_usecase() {
        let input = vec![
            String::from("190: 10 19"),
            String::from("3267: 81 40 27"),
            String::from("83: 17 5"),
            String::from("156: 15 6"),
            String::from("7290: 6 8 6 15"),
            String::from("161011: 16 10 13"),
            String::from("192: 17 8 14"),
            String::from("21037: 9 7 18 13"),
            String::from("292: 11 6 16 20"),
        ];

        let result = p2(&input);
        assert_eq!(result, 11387);
    }
}
