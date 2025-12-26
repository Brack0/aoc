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
    WordSearch::from(input).count_xmas()
}

fn p2(input: &[String]) -> usize {
    WordSearch::from(input).count_x_mas()
}

struct WordSearch {
    cells: Vec<Vec<char>>,
    height: usize,
    width: usize,
}

impl From<&[String]> for WordSearch {
    fn from(input: &[String]) -> Self {
        let cells: Vec<Vec<char>> = input.iter().map(|s| s.chars().collect()).collect();

        WordSearch {
            height: input.len(),
            width: cells[0].len(),
            cells,
        }
    }
}

impl WordSearch {
    fn count_xmas(&self) -> usize {
        (0..self.height)
            .flat_map(|y| (0..self.width).map(move |x| (x, y)))
            .filter(|&(x, y)| self.cells[y][x] == 'X')
            .map(|(x, y)| {
                ALL_DIRECTIONS
                    .iter()
                    .filter(|&&(dx, dy)| self.matches_mas(x, y, dx, dy))
                    .count()
            })
            .sum()
    }

    fn matches_mas(&self, x: usize, y: usize, dx: isize, dy: isize) -> bool {
        [('M', 1), ('A', 2), ('S', 3)]
            .iter()
            .all(|&(expected, offset)| {
                x.checked_add_signed(dx * offset)
                    .zip(y.checked_add_signed(dy * offset))
                    .is_some_and(|(nx, ny)| {
                        nx < self.width && ny < self.height && self.cells[ny][nx] == expected
                    })
            })
    }

    fn count_x_mas(&self) -> usize {
        (1..self.height - 1)
            .flat_map(|y| (1..self.width - 1).map(move |x| (x, y)))
            .filter(|&(x, y)| self.is_x_mas(x, y))
            .count()
    }

    fn is_x_mas(&self, x: usize, y: usize) -> bool {
        if self.cells[y][x] != 'A' {
            return false;
        }

        let top_left = self.cells[y - 1][x - 1];
        let top_right = self.cells[y - 1][x + 1];
        let bottom_left = self.cells[y + 1][x - 1];
        let bottom_right = self.cells[y + 1][x + 1];

        let diag1 = matches!((top_left, bottom_right), ('M', 'S') | ('S', 'M'));
        let diag2 = matches!((top_right, bottom_left), ('M', 'S') | ('S', 'M'));

        diag1 && diag2
    }
}

const ALL_DIRECTIONS: [(isize, isize); 8] = [
    (1, 0),
    (-1, 0),
    (0, 1),
    (0, -1),
    (1, 1),
    (-1, -1),
    (1, -1),
    (-1, 1),
];

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn p1_basic_usecase() {
        let input = vec![
            String::from("MMMSXXMASM"),
            String::from("MSAMXMSMSA"),
            String::from("AMXSXMAAMM"),
            String::from("MSAMASMSMX"),
            String::from("XMASAMXAMM"),
            String::from("XXAMMXXAMA"),
            String::from("SMSMSASXSS"),
            String::from("SAXAMASAAA"),
            String::from("MAMMMXMMMM"),
            String::from("MXMXAXMASX"),
        ];

        let result = p1(&input);

        assert_eq!(result, 18);
    }

    #[test]
    fn p2_basic_usecase() {
        let input = vec![
            String::from("MMMSXXMASM"),
            String::from("MSAMXMSMSA"),
            String::from("AMXSXMAAMM"),
            String::from("MSAMASMSMX"),
            String::from("XMASAMXAMM"),
            String::from("XXAMMXXAMA"),
            String::from("SMSMSASXSS"),
            String::from("SAXAMASAAA"),
            String::from("MAMMMXMMMM"),
            String::from("MXMXAXMASX"),
        ];

        let result = p2(&input);

        assert_eq!(result, 9);
    }
}
