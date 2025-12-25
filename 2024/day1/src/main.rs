use std::{
    collections::HashMap,
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

fn p1(input: &[String]) -> u32 {
    let mut left = vec![];
    let mut right = vec![];

    for line in input {
        let mut parts = line.split_whitespace();
        let l: u32 = parts.next().unwrap().parse().unwrap();
        let r: u32 = parts.next().unwrap().parse().unwrap();
        left.push(l);
        right.push(r);
    }

    left.sort_unstable();
    right.sort_unstable();

    left.iter()
        .zip(right.iter())
        .map(|(l, r)| l.abs_diff(*r))
        .sum()
}

fn p2(input: &[String]) -> u32 {
    let mut left = vec![];
    let mut right = HashMap::new();

    for line in input {
        let mut parts = line.split_whitespace();
        let l: u32 = parts.next().unwrap().parse().unwrap();
        let r: u32 = parts.next().unwrap().parse().unwrap();
        left.push(l);
        *right.entry(r).or_insert(0) += 1;
    }

    left.iter()
        .map(|l| l * right.get(l).unwrap_or(&0))
        .sum::<u32>()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn p1_basic_usecase() {
        let input = vec![
            String::from("3   4"),
            String::from("4   3"),
            String::from("2   5"),
            String::from("1   3"),
            String::from("3   9"),
            String::from("3   3"),
        ];

        let result = p1(&input);

        assert_eq!(result, 11);
    }

    #[test]
    fn p2_basic_usecase() {
        let input = vec![
            String::from("3   4"),
            String::from("4   3"),
            String::from("2   5"),
            String::from("1   3"),
            String::from("3   9"),
            String::from("3   3"),
        ];

        let result = p2(&input);

        assert_eq!(result, 31);
    }
}
