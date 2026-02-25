use std::{
    collections::{HashMap, HashSet},
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
    Map::from(input).antinodes().len()
}

fn p2(input: &[String]) -> usize {
    Map::from(input).antinodes_with_harmonics().len()
}

type Position = (usize, usize);
type Vector = (isize, isize);

/// Generate a vector from `left` to `right`
fn vector((left, right): (Position, Position)) -> Vector {
    (
        right.0.checked_signed_diff(left.0).unwrap(),
        right.1.checked_signed_diff(left.1).unwrap(),
    )
}

struct Map {
    height: usize,
    width: usize,
    antennas: HashMap<char, Vec<Position>>,
}

impl From<&[String]> for Map {
    fn from(lines: &[String]) -> Self {
        let mut antennas: HashMap<char, Vec<Position>> = HashMap::new();
        let height = lines.len();
        let width = lines[0].len();

        lines.iter().enumerate().for_each(|(y, line)| {
            line.chars().enumerate().for_each(|(x, frequency)| {
                if frequency != '.' {
                    antennas.entry(frequency).or_default().push((x, y));
                }
            });
        });

        Map {
            height,
            width,
            antennas,
        }
    }
}

impl Map {
    fn antinodes(&self) -> HashSet<Position> {
        self.antenna_pairs()
            .flat_map(|pair| self.create_antinodes_pair(pair))
            .collect()
    }

    fn antinodes_with_harmonics(&self) -> HashSet<Position> {
        self.antenna_pairs()
            .flat_map(|pair| self.create_antinodes_harmonics(pair))
            .collect()
    }

    fn create_antinodes_pair(&self, pair: (Position, Position)) -> impl Iterator<Item = Position> {
        let (dx, dy) = vector(pair);

        [
            self.create_antinode(pair.0, (dx, dy), 1),
            self.create_antinode(pair.1, (-dx, -dy), 1),
        ]
        .into_iter()
        .flatten()
    }

    fn create_antinodes_harmonics(
        &self,
        pair: (Position, Position),
    ) -> impl Iterator<Item = Position> {
        let (dx, dy) = vector(pair);

        self.create_antinodes_harmonics_half(pair.0, (dx, dy))
            .chain(self.create_antinodes_harmonics_half(pair.1, (-dx, -dy)))
    }

    fn create_antinodes_harmonics_half(
        &self,
        antenna: (usize, usize),
        vector: Vector,
    ) -> impl Iterator<Item = Position> {
        (0..).map_while(move |offset_mul| self.create_antinode(antenna, vector, offset_mul))
    }

    fn create_antinode(
        &self,
        antenna: Position,
        (dx, dy): Vector,
        offset_mul: isize,
    ) -> Option<Position> {
        let candidate = antenna
            .0
            .checked_sub_signed(dx * offset_mul)
            .zip(antenna.1.checked_sub_signed(dy * offset_mul));

        match candidate {
            Some((x, y)) if x < self.width && y < self.height => Some((x, y)),
            _ => None,
        }
    }

    fn antenna_pairs(&self) -> impl Iterator<Item = (Position, Position)> {
        self.antennas.iter().flat_map(|antenna| {
            let positions = antenna.1;

            positions
                .iter()
                .enumerate()
                .flat_map(|(i, &p0)| positions.iter().skip(i + 1).map(move |&p1| (p0, p1)))
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn two_identical_antennas() {
        let input = vec![
            String::from(".........."),
            String::from(".........."),
            String::from(".........."),
            String::from("....a....."),
            String::from(".........."),
            String::from(".....a...."),
            String::from(".........."),
            String::from(".........."),
            String::from(".........."),
            String::from(".........."),
        ];

        let result = p1(&input);

        assert_eq!(result, 2);
    }

    #[test]
    fn three_identical_antennas() {
        let input = vec![
            String::from(".........."),
            String::from(".........."),
            String::from(".........."),
            String::from("....a....."),
            String::from("........a."),
            String::from(".....a...."),
            String::from(".........."),
            String::from(".........."),
            String::from(".........."),
            String::from(".........."),
        ];

        let result = p1(&input);

        assert_eq!(result, 4);
    }

    #[test]
    fn p1_basic_usecase() {
        let input = vec![
            String::from("............"),
            String::from("........0..."),
            String::from(".....0......"),
            String::from(".......0...."),
            String::from("....0......."),
            String::from("......A....."),
            String::from("............"),
            String::from("............"),
            String::from("........A..."),
            String::from(".........A.."),
            String::from("............"),
            String::from("............"),
        ];

        let result = p1(&input);

        assert_eq!(result, 14);
    }

    #[test]
    fn t_frequency_antennas() {
        let input = vec![
            String::from("T........."),
            String::from("...T......"),
            String::from(".T........"),
            String::from(".........."),
            String::from(".........."),
            String::from(".........."),
            String::from(".........."),
            String::from(".........."),
            String::from(".........."),
            String::from(".........."),
        ];

        let result = p2(&input);

        assert_eq!(result, 9);
    }

    #[test]
    fn p2_basic_usecase() {
        let input = vec![
            String::from("............"),
            String::from("........0..."),
            String::from(".....0......"),
            String::from(".......0...."),
            String::from("....0......."),
            String::from("......A....."),
            String::from("............"),
            String::from("............"),
            String::from("........A..."),
            String::from(".........A.."),
            String::from("............"),
            String::from("............"),
        ];

        let result = p2(&input);

        assert_eq!(result, 34);
    }
}
