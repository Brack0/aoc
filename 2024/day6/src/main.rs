use std::{
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

fn p1(input: &[String]) -> usize {
    let lab = Lab::from(input);

    patrol(&lab, None).unwrap().len()
}

fn p2(input: &[String]) -> usize {
    let lab = Lab::from(input);

    let original_path = patrol(&lab, None).unwrap();

    original_path
        .iter()
        .filter(|&position| *position != lab.guard_start)
        .filter(|&position| patrol(&lab, Some(*position)).is_none())
        .count()
}

type Position = (isize, isize);

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn delta(self) -> (isize, isize) {
        match self {
            Direction::Up => (0, -1),
            Direction::Right => (1, 0),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
        }
    }

    fn rotate_right(self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

struct Lab {
    obstacles: HashSet<Position>,
    width: isize,
    height: isize,
    guard_start: Position,
}

impl From<&[String]> for Lab {
    fn from(input: &[String]) -> Self {
        let height = input.len().cast_signed();
        let width = input[0].len().cast_signed();
        let mut obstacles = HashSet::new();
        let mut start = (0, 0);

        for (y, line) in input.iter().enumerate() {
            for (x, char) in line.chars().enumerate() {
                let position = (x.cast_signed(), y.cast_signed());
                match char {
                    '#' => {
                        obstacles.insert(position);
                    }
                    '^' => {
                        start = position;
                    }
                    _ => {}
                }
            }
        }

        Lab {
            obstacles,
            width,
            height,
            guard_start: start,
        }
    }
}

impl Lab {
    fn in_bounds(&self, position: Position) -> bool {
        position.0 >= 0 && position.0 < self.width && position.1 >= 0 && position.1 < self.height
    }

    fn is_obstacle(&self, position: Position) -> bool {
        self.obstacles.contains(&position)
    }
}

fn patrol(lab: &Lab, extra_obstacle: Option<Position>) -> Option<HashSet<Position>> {
    let mut visited_positions = HashSet::new();
    let mut visited_states = HashSet::new();

    let mut position = lab.guard_start;
    let mut direction = Direction::Up;

    loop {
        if !visited_states.insert((position, direction)) {
            return None;
        }

        visited_positions.insert(position);

        let (dx, dy) = direction.delta();
        let next_position = (position.0 + dx, position.1 + dy);

        if !lab.in_bounds(next_position) {
            return Some(visited_positions);
        }

        if lab.is_obstacle(next_position) || Some(next_position) == extra_obstacle {
            direction = direction.rotate_right();
        } else {
            position = next_position;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn p1_basic_usecase() {
        let input = vec![
            String::from("....#....."),
            String::from(".........#"),
            String::from(".........."),
            String::from("..#......."),
            String::from(".......#.."),
            String::from(".........."),
            String::from(".#..^....."),
            String::from("........#."),
            String::from("#........."),
            String::from("......#..."),
        ];

        let result = p1(&input);
        assert_eq!(result, 41);
    }

    #[test]
    fn p2_basic_usecase() {
        let input = vec![
            String::from("....#....."),
            String::from(".........#"),
            String::from(".........."),
            String::from("..#......."),
            String::from(".......#.."),
            String::from(".........."),
            String::from(".#..^....."),
            String::from("........#."),
            String::from("#........."),
            String::from("......#..."),
        ];

        let result = p2(&input);
        assert_eq!(result, 6);
    }
}
