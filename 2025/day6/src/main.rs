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
    Problems::from_horizontal_numbers(input).solve()
}

fn p2(input: &[String]) -> u64 {
    Problems::from_vertical_numbers(input).solve()
}

enum Operation {
    Add,
    Multiply,
}

struct Problems {
    numbers: Vec<Vec<u64>>,
    operations: Vec<Operation>,
}

impl Problems {
    fn from_horizontal_numbers(input: &[String]) -> Self {
        let num_cols = input.first().map_or(0, |l| l.split_whitespace().count());
        let mut numbers: Vec<Vec<u64>> = vec![vec![]; num_cols];

        for line in &input[..input.len() - 1] {
            for (col, num) in line.split_whitespace().enumerate() {
                numbers[col].push(num.parse::<u64>().unwrap());
            }
        }

        let operations = input
            .last()
            .unwrap()
            .split_whitespace()
            .map(|op| match op {
                "+" => Operation::Add,
                "*" => Operation::Multiply,
                _ => panic!("Unknown operation: {op}"),
            })
            .collect();

        Problems {
            numbers,
            operations,
        }
    }

    fn from_vertical_numbers(input: &[String]) -> Self {
        let num_cols = input.first().map_or(0, |l| l.split_whitespace().count());
        let mut numbers: Vec<Vec<u64>> = vec![vec![]; num_cols];
        let mut index = 0usize;

        for x in (0..input[0].len()).rev() {
            let mut number = 0u64;
            for line in &input[..input.len() - 1] {
                let c = line.chars().nth(x).unwrap();
                if c.is_whitespace() {
                    continue;
                }
                number = number * 10 + c.to_string().parse::<u64>().unwrap();
            }
            if number == 0 {
                index += 1;
                continue;
            }
            numbers[index].push(number);
        }

        let operations = input
            .last()
            .unwrap()
            .split_whitespace()
            .rev()
            .map(|op| match op {
                "+" => Operation::Add,
                "*" => Operation::Multiply,
                _ => panic!("Unknown operation: {op}"),
            })
            .collect();

        Problems {
            numbers,
            operations,
        }
    }

    fn solve(&self) -> u64 {
        self.numbers
            .iter()
            .zip(self.operations.iter())
            .map(|(nums, op)| match op {
                Operation::Add => nums.iter().sum::<u64>(),
                Operation::Multiply => nums.iter().product::<u64>(),
            })
            .sum()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn p1_basic_usecase() {
        let input = vec![
            "1 2 3".to_string(),
            "4 5 6".to_string(),
            "+ * +".to_string(),
        ];
        assert_eq!(p1(&input), 24); // (1 + 4) + (2 * 5) + (3 + 6)
    }

    #[test]
    fn p1_example_usecase() {
        let input = vec![
            "123 328  51 64 ".to_string(),
            " 45 64  387 23 ".to_string(),
            "  6 98  215 314".to_string(),
            "*   +   *   +  ".to_string(),
        ];
        assert_eq!(p1(&input), 4_277_556); // cf. reasoning in README.md
    }

    #[test]
    fn p2_basic_usecase() {
        let input = vec![
            "12 34 5".to_string(),
            "67 8  9".to_string(),
            "+  *  +".to_string(),
        ];
        assert_eq!(p2(&input), 254); // (5 + 9) + (4 * 38) + (27 + 12)
    }

    #[test]
    fn p2_example_usecase() {
        let input = vec![
            "123 328  51 64 ".to_string(),
            " 45 64  387 23 ".to_string(),
            "  6 98  215 314".to_string(),
            "*   +   *   +  ".to_string(),
        ];
        assert_eq!(p2(&input), 3_263_827); // cf. reasoning in README.md
    }
}
