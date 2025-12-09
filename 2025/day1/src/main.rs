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

fn p1(input: &[String]) -> u32 {
    let mut safe = Safe::from(50);

    safe.run_sequence(&input.iter().map(Rotation::from).collect::<Vec<Rotation>>());

    safe.stops_at_0
}

fn p2(input: &[String]) -> i32 {
    let mut safe = Safe::from(50);

    safe.run_sequence(&input.iter().map(Rotation::from).collect::<Vec<Rotation>>());

    safe.goes_over_0
}

struct Safe {
    position: i32,
    stops_at_0: u32,
    goes_over_0: i32,
}

impl From<i32> for Safe {
    fn from(position: i32) -> Self {
        Safe {
            position,
            stops_at_0: 0,
            goes_over_0: 0,
        }
    }
}

impl Safe {
    fn run_sequence(&mut self, sequence: &[Rotation]) {
        for rotation in sequence {
            self.rotate(rotation);
        }
    }

    fn rotate(&mut self, rotation: &Rotation) {
        match rotation.direction {
            RotationDirection::L => self.rotate_left(rotation.distance),
            RotationDirection::R => self.rotate_right(rotation.distance),
        }
    }

    fn rotate_left(&mut self, distance: i32) {
        self.goes_over_0 += distance / 100;

        if self.position <= distance.rem_euclid(100) && self.position != 0 {
            self.goes_over_0 += 1;
        }

        self.position = (self.position - distance).rem_euclid(100);

        if self.position == 0 {
            self.stops_at_0 += 1;
        }
    }

    fn rotate_right(&mut self, distance: i32) {
        self.goes_over_0 += distance / 100;

        if self.position + (distance.rem_euclid(100)) >= 100 {
            self.goes_over_0 += 1;
        }

        self.position = (self.position + distance).rem_euclid(100);

        if self.position == 0 {
            self.stops_at_0 += 1;
        }
    }
}

enum RotationDirection {
    L,
    R,
}

struct Rotation {
    direction: RotationDirection,
    distance: i32,
}

impl From<&String> for Rotation {
    fn from(value: &String) -> Self {
        let (direction, distance) = value.split_at(1);
        let direction = match direction {
            "L" => RotationDirection::L,
            "R" => RotationDirection::R,
            _ => panic!("Invalid rotation"),
        };
        let distance = distance.parse::<i32>().unwrap();

        Rotation {
            direction,
            distance,
        }
    }
}

#[cfg(test)]
mod p1 {
    use super::*;

    #[test]
    fn no_rotation_no_password_increment() {
        let input = vec![];
        assert_eq!(p1(&input), 0);
    }

    #[test]
    fn increment_password_half_turn_left() {
        let input = vec!["L50".to_string()];
        assert_eq!(p1(&input), 1);
    }

    #[test]
    fn increment_password_half_turn_right() {
        let input = vec!["R50".to_string()];
        assert_eq!(p1(&input), 1);
    }

    #[test]
    fn no_increment_password_full_turn_left() {
        let input = vec!["L100".to_string()];
        assert_eq!(p1(&input), 0);
    }

    #[test]
    fn basic_usecase() {
        let input = vec![
            "L68".to_string(),
            "L30".to_string(),
            "R48".to_string(),
            "L5".to_string(),
            "R60".to_string(),
            "L55".to_string(),
            "L1".to_string(),
            "L99".to_string(),
            "R14".to_string(),
            "L82".to_string(),
        ];
        assert_eq!(p1(&input), 3);
    }
}

#[cfg(test)]
mod p2 {
    use super::*;

    #[test]
    fn no_rotation_no_password_increment() {
        let input = vec![];
        assert_eq!(p2(&input), 0);
    }

    #[test]
    fn increment_password_half_turn_left() {
        let input = vec!["L50".to_string()];
        assert_eq!(p2(&input), 1);
    }

    #[test]
    fn increment_password_half_turn_right() {
        let input = vec!["R50".to_string()];
        assert_eq!(p2(&input), 1);
    }

    #[test]
    fn increment_password_full_turn_left() {
        let input = vec!["L100".to_string()];
        assert_eq!(p2(&input), 1);
    }

    #[test]
    fn basic_usecase() {
        let input = vec![
            "L68".to_string(),
            "L30".to_string(),
            "R48".to_string(),
            "L5".to_string(),
            "R60".to_string(),
            "L55".to_string(),
            "L1".to_string(),
            "L99".to_string(),
            "R14".to_string(),
            "L82".to_string(),
        ];
        assert_eq!(p2(&input), 6);
    }
}
