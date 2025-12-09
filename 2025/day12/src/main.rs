use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

fn main() -> io::Result<()> {
    let input = read_input()?;

    let p1_result = p1(&input);

    println!("Part 1 result: {p1_result}");

    println!("Part 2 result: 1 star");

    Ok(())
}

fn read_input() -> io::Result<Vec<String>> {
    let file = File::open("input/raw.txt")?;
    BufReader::new(file).lines().collect::<Result<_, _>>()
}

fn p1(input: &[String]) -> usize {
    let mut presents = Vec::new();
    let mut regions = Vec::new();
    let mut parsing_regions = false;

    for (i, line) in input.iter().enumerate() {
        if line.contains('x') {
            parsing_regions = true;
        }

        if parsing_regions {
            regions.push(Region::from(line));
        } else if line.contains(':') {
            presents.push(Present::from([&input[i + 1], &input[i + 2], &input[i + 3]]));
        }
    }

    regions
        .iter()
        .map(|region| naive_solve(region, &presents))
        .filter(|&can_fit| can_fit)
        .count()
}

struct Present {
    area: usize,
}

impl From<[&String; 3]> for Present {
    fn from(lines: [&String; 3]) -> Self {
        let mut area = 0;
        for line in lines {
            for char in line.chars() {
                area += usize::from(char == '#');
            }
        }
        Present { area }
    }
}

struct Region {
    width: usize,
    length: usize,
    expected_presents: Vec<usize>,
}

impl From<&String> for Region {
    fn from(line: &String) -> Self {
        let parts: Vec<&str> = line.split(':').collect();
        let dimensions: Vec<usize> = parts[0].split('x').map(|s| s.parse().unwrap()).collect();
        let width = dimensions[0];
        let length = dimensions[1];
        let expected_presents: Vec<usize> = parts[1]
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        Region {
            width,
            length,
            expected_presents,
        }
    }
}

fn naive_solve(region: &Region, presents: &[Present]) -> bool {
    let total_present_area: usize = region
        .expected_presents
        .iter()
        .zip(presents)
        .map(|(amount, present)| present.area * amount)
        .sum();
    total_present_area <= region.width * region.length
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn p1_basic_usecase() {
        let input = vec![
            "0:".to_string(),
            "###".to_string(),
            "##.".to_string(),
            "##.".to_string(),
            String::new(),
            "1:".to_string(),
            "###".to_string(),
            "##.".to_string(),
            ".##".to_string(),
            String::new(),
            "2:".to_string(),
            ".##".to_string(),
            "###".to_string(),
            "##.".to_string(),
            String::new(),
            "3:".to_string(),
            "##.".to_string(),
            "###".to_string(),
            "##.".to_string(),
            String::new(),
            "4:".to_string(),
            "###".to_string(),
            "#..".to_string(),
            "###".to_string(),
            String::new(),
            "5:".to_string(),
            "###".to_string(),
            ".#.".to_string(),
            "###".to_string(),
            String::new(),
            "4x4: 0 0 0 0 2 0".to_string(),
            "12x5: 1 0 1 0 2 2".to_string(),
            "12x5: 1 0 1 0 3 2".to_string(),
        ];

        let result = p1(&input);

        assert_eq!(result, 2);
    }
}
