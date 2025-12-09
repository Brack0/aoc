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
    let mut tachyon_manifold = TachyonManifold::from(input.first().unwrap());

    input[1..]
        .iter()
        .map(DiagramStep::from)
        .for_each(|step| tachyon_manifold.step(&step));

    tachyon_manifold.splitters
}

fn p2(input: &[String]) -> u64 {
    let mut tachyon_manifold = TachyonManifold::from(input.first().unwrap());

    input[1..]
        .iter()
        .map(DiagramStep::from)
        .for_each(|step| tachyon_manifold.step(&step));
    
    tachyon_manifold.timelines.iter().sum::<u64>()
}

enum DiagramCell {
    Empty,
    Splitter,
}

impl From<char> for DiagramCell {
    fn from(value: char) -> Self {
        match value {
            '^' => DiagramCell::Splitter,
            _ => DiagramCell::Empty,
        }
    }
}

struct DiagramStep {
    cells: Vec<DiagramCell>,
}

impl From<&String> for DiagramStep {
    fn from(value: &String) -> Self {
        let cells = value.chars().map(DiagramCell::from).collect();

        DiagramStep { cells }
    }
}

struct TachyonManifold {
    timelines: Vec<u64>,
    splitters: u64,
}

impl From<&String> for TachyonManifold {
    fn from(value: &String) -> Self {
        let timelines = value
            .chars()
            .map(|c| match c {
                'S' => 1,
                _ => 0,
            })
            .collect();

        TachyonManifold {
            timelines,
            splitters: 0,
        }
    }
}

impl TachyonManifold {
    fn step(&mut self, step: &DiagramStep) {
        let mut new_timelines = vec![0; self.timelines.len()];

        for (i, &count) in self.timelines.iter().enumerate().filter(|&(_, &c)| c > 0) {
            match step.cells[i] {
                DiagramCell::Splitter => {
                    self.splitters += 1;

                    if i > 0 {
                        new_timelines[i - 1] += count;
                    }
                    if i + 1 < new_timelines.len() {
                        new_timelines[i + 1] += count;
                    }
                }
                DiagramCell::Empty => new_timelines[i] += count,
            }
        }

        self.timelines = new_timelines;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn p1_no_split() {
        let input = vec![
            ".S.".to_string(),
            "...".to_string(),
            "...".to_string(),
            "...".to_string(),
        ];
        let result = p1(&input);
        assert_eq!(result, 0);
    }

    #[test]
    fn p1_one_split() {
        let input = vec![
            ".S.".to_string(),
            "...".to_string(),
            ".^.".to_string(),
            "...".to_string(),
        ];
        let result = p1(&input);
        assert_eq!(result, 1);
    }

    #[test]
    fn p1_three_splits() {
        let input = vec![
            "..S..".to_string(),
            ".....".to_string(),
            "..^..".to_string(),
            ".....".to_string(),
            ".^.^.".to_string(),
            ".....".to_string(),
        ];
        let result = p1(&input);
        assert_eq!(result, 3);
    }

    #[test]
    fn p1_two_splits_one_avoided() {
        let input = vec![
            "..S...".to_string(),
            "......".to_string(),
            "..^...".to_string(),
            "......".to_string(),
            ".^..^.".to_string(), // Right splitter avoided
            "......".to_string(),
        ];
        let result = p1(&input);
        assert_eq!(result, 2);
    }

    #[test]
    fn p1_basic_usecase() {
        let input = vec![
            ".......S.......".to_string(),
            "...............".to_string(),
            ".......^.......".to_string(),
            "...............".to_string(),
            "......^.^......".to_string(),
            "...............".to_string(),
            ".....^.^.^.....".to_string(),
            "...............".to_string(),
            "....^.^...^....".to_string(),
            "...............".to_string(),
            "...^.^...^.^...".to_string(),
            "...............".to_string(),
            "..^...^.....^..".to_string(),
            "...............".to_string(),
            ".^.^.^.^.^...^.".to_string(),
            "...............".to_string(),
        ];
        let result = p1(&input);
        assert_eq!(result, 21);
    }

    #[test]
    fn p2_no_split() {
        let input = vec![
            ".S.".to_string(),
            "...".to_string(),
            "...".to_string(),
            "...".to_string(),
        ];
        let result = p2(&input);
        assert_eq!(result, 1);
    }

    #[test]
    fn p2_one_split() {
        let input = vec![
            ".S.".to_string(),
            "...".to_string(),
            ".^.".to_string(),
            "...".to_string(),
        ];
        let result = p2(&input);
        assert_eq!(result, 2);
    }

    #[test]
    fn p2_three_splits() {
        let input = vec![
            "..S..".to_string(),
            ".....".to_string(),
            "..^..".to_string(),
            ".....".to_string(),
            ".^.^.".to_string(),
            ".....".to_string(),
        ];
        let result = p2(&input);
        assert_eq!(result, 4);
    }

    #[test]
    fn p2_two_splits_one_avoided() {
        let input = vec![
            "..S...".to_string(),
            "......".to_string(),
            "..^...".to_string(),
            "......".to_string(),
            ".^..^.".to_string(), // Right splitter avoided
            "......".to_string(),
        ];
        let result = p2(&input);
        assert_eq!(result, 3);
    }

    #[test]
    fn p2_basic_usecase() {
        let input = vec![
            ".......S.......".to_string(),
            "...............".to_string(),
            ".......^.......".to_string(),
            "...............".to_string(),
            "......^.^......".to_string(),
            "...............".to_string(),
            ".....^.^.^.....".to_string(),
            "...............".to_string(),
            "....^.^...^....".to_string(),
            "...............".to_string(),
            "...^.^...^.^...".to_string(),
            "...............".to_string(),
            "..^...^.....^..".to_string(),
            "...............".to_string(),
            ".^.^.^.^.^...^.".to_string(),
            "...............".to_string(),
        ];
        let result = p2(&input);
        assert_eq!(result, 40);
    }
}
