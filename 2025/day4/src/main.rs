use std::{
    collections::{HashSet, VecDeque},
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
    Grid::from(input).count_accessible_paper_rolls()
}

fn p2(input: &[String]) -> usize {
    Grid::from(input).remove_all_accessible_paper_rolls()
}

enum Cell {
    PaperRoll,
    Empty,
}

impl Cell {
    fn is_paper_roll(&self) -> bool {
        matches!(self, Cell::PaperRoll)
    }
}

impl From<char> for Cell {
    fn from(c: char) -> Self {
        match c {
            '@' => Cell::PaperRoll,
            '.' => Cell::Empty,
            _ => panic!("Invalid character in grid"),
        }
    }
}

struct Grid {
    cells: Vec<Vec<Cell>>,
    width: usize,
    height: usize,
}

impl From<&[String]> for Grid {
    fn from(input: &[String]) -> Self {
        let cells = input
            .iter()
            .map(|line| line.chars().map(Cell::from).collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let height = cells.len();
        let width = if height > 0 { cells[0].len() } else { 0 };

        Grid {
            cells,
            width,
            height,
        }
    }
}

impl Grid {
    fn get(&self, x: usize, y: usize) -> &Cell {
        &self.cells[y][x]
    }

    fn can_remove_paper_roll(&self, (x, y, cell): (usize, usize, &Cell)) -> bool {
        cell.is_paper_roll()
            && self
                .adjacent_cells_iter(x, y)
                .filter(|(_, _, cell)| cell.is_paper_roll())
                .count()
                < 4
    }

    fn remove_paper_roll(&mut self, x: usize, y: usize) {
        self.cells[y][x] = Cell::Empty;
    }

    fn cells_iter(&self) -> impl Iterator<Item = (usize, usize, &Cell)> {
        self.cells
            .iter()
            .enumerate()
            .flat_map(|(y, row)| row.iter().enumerate().map(move |(x, cell)| (x, y, cell)))
    }

    fn adjacent_cells_iter(
        &self,
        x: usize,
        y: usize,
    ) -> impl Iterator<Item = (usize, usize, &Cell)> {
        [
            (x.wrapping_sub(1), y),                 // Left
            (x + 1, y),                             // Right
            (x, y.wrapping_sub(1)),                 // Up
            (x, y + 1),                             // Down
            (x.wrapping_sub(1), y.wrapping_sub(1)), // Up-Left
            (x + 1, y.wrapping_sub(1)),             // Up-Right
            (x.wrapping_sub(1), y + 1),             // Down-Left
            (x + 1, y + 1),                         // Down-Right
        ]
        .into_iter()
        .filter(|(x, y)| (*x < self.width) && (*y < self.height))
        .map(|(x, y)| (x, y, self.get(x, y)))
    }

    fn count_accessible_paper_rolls(&self) -> usize {
        self.cells_iter()
            .filter(|&cell| self.can_remove_paper_roll(cell))
            .count()
    }

    fn remove_all_accessible_paper_rolls(&mut self) -> usize {
        let mut count = 0;

        let mut paper_rolls_to_remove = VecDeque::new();
        let mut scheduled_cells = HashSet::new();

        self.cells_iter()
            .filter(|&cell| self.can_remove_paper_roll(cell))
            .for_each(|(x, y, _)| {
                paper_rolls_to_remove.push_back((x, y));
                scheduled_cells.insert((x, y));
            });

        while let Some((x, y)) = paper_rolls_to_remove.pop_front() {
            self.remove_paper_roll(x, y);
            count += 1;

            self.adjacent_cells_iter(x, y)
                .filter(|(x, y, cell)| {
                    self.can_remove_paper_roll((*x, *y, cell)) && scheduled_cells.insert((*x, *y))
                })
                .for_each(|(x, y, _)| paper_rolls_to_remove.push_back((x, y)));
        }

        count
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn count_access_paper_roll_in_tiniest_empty_grid() {
        let input = vec![".".to_string()];

        let result = p1(&input);

        assert_eq!(result, 0);
    }

    #[test]
    fn count_access_paper_roll_in_tiniest_filled_grid() {
        let input = vec!["@".to_string()];

        let result = p1(&input);

        assert_eq!(result, 1);
    }

    #[test]
    fn count_access_paper_roll_in_tiny_empty_grid() {
        let input = vec!["..".to_string(), "..".to_string()];

        let result = p1(&input);

        assert_eq!(result, 0);
    }

    #[test]
    fn count_access_paper_roll_in_tiny_filled_grid() {
        let input = vec!["@@".to_string(), "@@".to_string()];

        let result = p1(&input);

        assert_eq!(result, 4);
    }

    #[test]
    fn count_access_paper_roll_in_tiny_half_filled_grid() {
        let input = vec![".@".to_string(), "@.".to_string()];

        let result = p1(&input);

        assert_eq!(result, 2);
    }

    #[test]
    fn count_access_paper_roll_in_small_empty_grid() {
        let input = vec!["...".to_string(), "...".to_string(), "...".to_string()];

        let result = p1(&input);

        assert_eq!(result, 0);
    }

    #[test]
    fn count_access_paper_roll_in_small_filled_grid() {
        let input = vec!["@@@".to_string(), "@@@".to_string(), "@@@".to_string()];

        let result = p1(&input);

        assert_eq!(result, 4); // Only corners are accessible
    }

    #[test]
    fn count_access_paper_roll_in_small_half_filled_grid() {
        let input = vec!["@.@".to_string(), ".@.".to_string(), "@.@".to_string()];

        let result = p1(&input);

        assert_eq!(result, 4); // Only corners are accessible
    }

    #[test]
    fn count_access_paper_roll_in_small_random_grid() {
        let input = vec![".@@".to_string(), "@.@".to_string(), "@@.".to_string()];

        let result = p1(&input);

        assert_eq!(result, 6);
    }

    #[test]
    fn count_access_paper_roll_in_small_almost_full_grid() {
        let input = vec![".@@".to_string(), "@@@".to_string(), "@@.".to_string()];

        let result = p1(&input);

        assert_eq!(result, 2); // Only the two corners are accessible (top-right and bottom-left)
    }

    #[test]
    fn count_access_paper_roll_in_basic_grid() {
        let input = vec![
            "..@@.@@@@.".to_string(),
            "@@@.@.@.@@".to_string(),
            "@@@@@.@.@@".to_string(),
            "@.@@@@..@.".to_string(),
            "@@.@@@@.@@".to_string(),
            ".@@@@@@@.@".to_string(),
            ".@.@.@.@@@".to_string(),
            "@.@@@.@@@@".to_string(),
            ".@@@@@@@@.".to_string(),
            "@.@.@@@.@.".to_string(),
        ];

        let result = p1(&input);

        assert_eq!(result, 13);
    }

    #[test]
    fn count_all_removed_access_paper_roll_in_basic_grid_alt() {
        let input = vec![
            "..@@.@@@@.".to_string(),
            "@@@.@.@.@@".to_string(),
            "@@@@@.@.@@".to_string(),
            "@.@@@@..@.".to_string(),
            "@@.@@@@.@@".to_string(),
            ".@@@@@@@.@".to_string(),
            ".@.@.@.@@@".to_string(),
            "@.@@@.@@@@".to_string(),
            ".@@@@@@@@.".to_string(),
            "@.@.@@@.@.".to_string(),
        ];

        let result = p2(&input);

        assert_eq!(result, 43);
    }
}
