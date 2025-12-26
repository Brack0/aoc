use std::{
    cmp::Ordering,
    collections::HashSet,
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
    PrintQueue::from(input)
        .valid_updates()
        .map(|update| middle_page(update))
        .sum()
}

fn p2(input: &[String]) -> u32 {
    PrintQueue::from(input)
        .corrected_updates()
        .map(|corrected| middle_page(&corrected))
        .sum()
}

fn middle_page(pages: &[u32]) -> u32 {
    pages[pages.len() / 2]
}

struct PrintQueue {
    ordering: HashSet<(u32, u32)>,
    updates: Vec<Vec<u32>>,
}

impl From<&[String]> for PrintQueue {
    fn from(input: &[String]) -> Self {
        let split_idx = input.iter().position(String::is_empty).unwrap_or(0);

        let ordering: HashSet<_> = input[..split_idx]
            .iter()
            .filter_map(|line| {
                let (a, b) = line.split_once('|')?;
                Some((a.parse().ok()?, b.parse().ok()?))
            })
            .collect();

        let updates: Vec<_> = input[split_idx + 1..]
            .iter()
            .map(|line| line.split(',').filter_map(|s| s.parse().ok()).collect())
            .collect();

        PrintQueue { ordering, updates }
    }
}

impl PrintQueue {
    fn valid_updates(&self) -> impl Iterator<Item = &Vec<u32>> {
        self.updates.iter().filter(|update| self.is_valid(update))
    }

    fn is_valid(&self, update: &[u32]) -> bool {
        update
            .windows(2)
            .all(|pages| self.ordering.contains(&(pages[0], pages[1])))
    }

    fn corrected_updates(&self) -> impl Iterator<Item = Vec<u32>> {
        self.updates
            .iter()
            .filter(|update| !self.is_valid(update))
            .map(|update| self.sort_clone(update))
    }

    fn sort_clone(&self, vec: &[u32]) -> Vec<u32> {
        let mut sorted = vec.to_vec();
        sorted.sort_by(|a, b| {
            if self.ordering.contains(&(*a, *b)) {
                Ordering::Less
            } else if self.ordering.contains(&(*b, *a)) {
                Ordering::Greater
            } else {
                Ordering::Equal
            }
        });
        sorted
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn p1_basic_usecase() {
        let input = vec![
            String::from("47|53"),
            String::from("97|13"),
            String::from("97|61"),
            String::from("97|47"),
            String::from("75|29"),
            String::from("61|13"),
            String::from("75|53"),
            String::from("29|13"),
            String::from("97|29"),
            String::from("53|29"),
            String::from("61|53"),
            String::from("97|53"),
            String::from("61|29"),
            String::from("47|13"),
            String::from("75|47"),
            String::from("97|75"),
            String::from("47|61"),
            String::from("75|61"),
            String::from("47|29"),
            String::from("75|13"),
            String::from("53|13"),
            String::new(),
            String::from("75,47,61,53,29"),
            String::from("97,61,53,29,13"),
            String::from("75,29,13"),
            String::from("75,97,47,61,53"),
            String::from("61,13,29"),
            String::from("97,13,75,29,47"),
        ];

        let result = p1(&input);

        assert_eq!(result, 143);
    }

    #[test]
    fn p2_basic_usecase() {
        let input = vec![
            String::from("47|53"),
            String::from("97|13"),
            String::from("97|61"),
            String::from("97|47"),
            String::from("75|29"),
            String::from("61|13"),
            String::from("75|53"),
            String::from("29|13"),
            String::from("97|29"),
            String::from("53|29"),
            String::from("61|53"),
            String::from("97|53"),
            String::from("61|29"),
            String::from("47|13"),
            String::from("75|47"),
            String::from("97|75"),
            String::from("47|61"),
            String::from("75|61"),
            String::from("47|29"),
            String::from("75|13"),
            String::from("53|13"),
            String::new(),
            String::from("75,47,61,53,29"),
            String::from("97,61,53,29,13"),
            String::from("75,29,13"),
            String::from("75,97,47,61,53"),
            String::from("61,13,29"),
            String::from("97,13,75,29,47"),
        ];

        let result = p2(&input);

        assert_eq!(result, 123);
    }
}
