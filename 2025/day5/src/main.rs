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

fn p1(input: &[String]) -> usize {
    Inventory::from(input).count_fresh_ingredients()
}

fn p2(input: &[String]) -> u64 {
    Inventory::from(input).count_unique_fresh_ids()
}

struct Inventory {
    id_ranges: Vec<(u64, u64)>,
    ingredient_ids: Vec<u64>,
}

impl From<&[String]> for Inventory {
    fn from(lines: &[String]) -> Self {
        let mut id_ranges = vec![];
        let mut ingredient_ids = vec![];
        let mut parsing_ingredients = false;

        for line in lines {
            if line.is_empty() {
                parsing_ingredients = true;
                continue;
            }
            if parsing_ingredients {
                ingredient_ids.push(line.parse().unwrap());
            } else {
                let parts = line.split_once('-').unwrap();
                id_ranges.push((parts.0.parse().unwrap(), parts.1.parse().unwrap()));
            }
        }

        Inventory {
            id_ranges,
            ingredient_ids,
        }
    }
}

impl Inventory {
    fn count_fresh_ingredients(&self) -> usize {
        self.ingredient_ids
            .iter()
            .filter(|&id| self.is_fresh(*id))
            .count()
    }

    fn is_fresh(&self, id: u64) -> bool {
        self.id_ranges
            .iter()
            .any(|&id_range| is_in_range(id, id_range))
    }

    fn count_unique_fresh_ids(&mut self) -> u64 {
        self.id_ranges.sort_by_key(|r| r.0);

        let mut merged_ranges: Vec<(u64, u64)> = Vec::new();
        merged_ranges.push(self.id_ranges[0]);

        for range in &self.id_ranges {
            let last = merged_ranges.last_mut().unwrap();

            if range.0 <= last.1 + 1 {
                last.1 = last.1.max(range.1);
            } else {
                merged_ranges.push(*range);
            }
        }

        merged_ranges
            .iter()
            .map(|(start, end)| end - start + 1)
            .sum()
    }
}

fn is_in_range(id: u64, id_range: (u64, u64)) -> bool {
    id >= id_range.0 && id <= id_range.1
}

#[cfg(test)]
mod test {
    use std::vec;

    use super::*;

    #[test]
    fn test_is_in_range() {
        assert!(is_in_range(5, (1, 10)));
        assert!(is_in_range(1, (1, 10)));
        assert!(is_in_range(10, (1, 10)));
        assert!(!is_in_range(0, (1, 10)));
        assert!(!is_in_range(11, (1, 10)));
    }

    #[test]
    fn test_is_fresh() {
        let input = vec![
            "3-5".to_string(),
            "10-14".to_string(),
            "16-20".to_string(),
            "12-18".to_string(),
            String::new(),
            "1".to_string(),
            "5".to_string(),
            "8".to_string(),
            "11".to_string(),
            "17".to_string(),
            "32".to_string(),
        ];

        let inventory = Inventory::from(input.as_slice());

        assert!(!inventory.is_fresh(1));
        assert!(inventory.is_fresh(5));
        assert!(!inventory.is_fresh(8));
        assert!(inventory.is_fresh(11));
        assert!(inventory.is_fresh(17));
        assert!(!inventory.is_fresh(32));
    }

    #[test]
    fn test_count_fresh_ingredients() {
        let input = vec![
            "3-5".to_string(),
            "10-14".to_string(),
            "16-20".to_string(),
            "12-18".to_string(),
            String::new(),
            "1".to_string(),
            "5".to_string(),
            "8".to_string(),
            "11".to_string(),
            "17".to_string(),
            "32".to_string(),
        ];

        assert_eq!(p1(&input), 3);
    }

    #[test]
    fn test_count_unique_fresh_ids() {
        let input = vec![
            "3-5".to_string(),
            "10-14".to_string(),
            "16-20".to_string(),
            "12-18".to_string(),
            String::new(),
            "1".to_string(),
            "5".to_string(),
            "8".to_string(),
            "11".to_string(),
            "17".to_string(),
            "32".to_string(),
        ];

        assert_eq!(p2(&input), 14);
    }
}
