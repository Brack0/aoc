use std::{
    cmp::Ordering,
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

fn p1(input: &[String]) -> usize {
    input
        .iter()
        .map(Report::from)
        .filter(Report::is_safe)
        .count()
}

fn p2(input: &[String]) -> usize {
    input
        .iter()
        .map(Report::from)
        .filter(Report::is_safe_with_one_removal)
        .count()
}

struct Report {
    levels: Vec<u32>,
}

impl From<&String> for Report {
    fn from(line: &String) -> Self {
        let levels = line
            .split_whitespace()
            .map(|number| number.parse().unwrap())
            .collect();

        Report { levels }
    }
}

impl Report {
    fn is_safe(&self) -> bool {
        let start_ordering = self.levels[0].cmp(&self.levels[1]);

        self.levels.windows(2).all(|pair| {
            let ordering = pair[0].cmp(&pair[1]);
            ordering != Ordering::Equal
                && ordering == start_ordering
                && pair[0].abs_diff(pair[1]) <= 3
        })
    }

    fn is_safe_with_one_removal(&self) -> bool {
        if self.is_safe() {
            return true;
        }
        (0..self.levels.len()).any(|i| {
            let mut levels = self.levels.clone();
            levels.remove(i);
            Report { levels }.is_safe()
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn p1_basic_usecase() {
        let input = vec![
            String::from("7 6 4 2 1"),
            String::from("1 2 7 8 9"),
            String::from("9 7 6 2 1"),
            String::from("1 3 2 4 5"),
            String::from("8 6 4 4 1"),
            String::from("1 3 6 7 9"),
        ];

        let result = p1(&input);

        assert_eq!(result, 2);
    }

    #[test]
    fn p2_basic_usecase() {
        let input = vec![
            String::from("7 6 4 2 1"),
            String::from("1 2 7 8 9"),
            String::from("9 7 6 2 1"),
            String::from("1 3 2 4 5"),
            String::from("8 6 4 4 1"),
            String::from("1 3 6 7 9"),
        ];

        let result = p2(&input);

        assert_eq!(result, 4);
    }
}
