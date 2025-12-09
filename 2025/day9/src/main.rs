use std::{
    collections::HashMap,
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
    Floor::from(input).find_maximum_rectangle()
}

fn p2(input: &[String]) -> u64 {
    Floor::from(input).find_maximum_rectangle_inside()
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Tile {
    x: u32,
    y: u32,
}

impl Tile {
    fn from(x: u32, y: u32) -> Self {
        Tile { x, y }
    }

    fn is_strictly_inside(self, rectangle: &Rectangle) -> bool {
        self.x > rectangle.top_left.x
            && self.x < rectangle.bottom_right.x
            && self.y > rectangle.top_left.y
            && self.y < rectangle.bottom_right.y
    }

    fn diagonal_neighbors(self) -> [Tile; 4] {
        [
            Self::from(self.x - 1, self.y - 1),
            Self::from(self.x + 1, self.y - 1),
            Self::from(self.x - 1, self.y + 1),
            Self::from(self.x + 1, self.y + 1),
        ]
    }
}

struct Rectangle {
    top_left: Tile,
    bottom_right: Tile,
}

impl Rectangle {
    fn from(a: Tile, b: Tile) -> Self {
        let x_min = a.x.min(b.x);
        let x_max = a.x.max(b.x);
        let y_min = a.y.min(b.y);
        let y_max = a.y.max(b.y);

        Rectangle {
            top_left: Tile::from(x_min, y_min),
            bottom_right: Tile::from(x_max, y_max),
        }
    }

    fn area(&self) -> u64 {
        let width = self.bottom_right.x.abs_diff(self.top_left.x) + 1;
        let height = self.bottom_right.y.abs_diff(self.top_left.y) + 1;
        u64::from(width) * u64::from(height)
    }
}

struct Floor {
    red_tiles: Vec<Tile>,
}

impl From<&[String]> for Floor {
    fn from(input: &[String]) -> Self {
        let red_tiles: Vec<Tile> = input
            .iter()
            .map(|line| {
                let (x, y) = line.split_once(',').unwrap();
                Tile::from(x.parse().unwrap(), y.parse().unwrap())
            })
            .collect();

        Floor { red_tiles }
    }
}

impl Floor {
    fn find_maximum_rectangle(&self) -> u64 {
        self.rectangle_candidates()
            .iter()
            .map(|(_, rectangle)| rectangle.area())
            .max()
            .unwrap()
    }

    fn find_maximum_rectangle_inside(&self) -> u64 {
        let mut candidates: Vec<(u64, Rectangle)> = self.rectangle_candidates();

        candidates.sort_unstable_by(|a, b| b.0.cmp(&a.0));

        let visited = &mut HashMap::new();

        candidates
            .iter()
            .find(|(_, rect)| self.is_rectangle_inside_polygon(rect, visited))
            .map_or(0, |(area, _)| *area)
    }

    fn rectangle_candidates(&self) -> Vec<(u64, Rectangle)> {
        let n = self.red_tiles.len();
        let mut candidates = Vec::with_capacity((n * (n - 1)) / 2);

        for (i, a) in self.red_tiles.iter().enumerate() {
            for b in self.red_tiles.iter().skip(i + 1) {
                let rect = Rectangle::from(*a, *b);
                let area = rect.area();
                candidates.push((area, rect));
            }
        }

        candidates
    }

    fn is_rectangle_inside_polygon(
        &self,
        rectangle: &Rectangle,
        visited: &mut HashMap<Tile, bool>,
    ) -> bool {
        self.iter_interior_red_tiles(rectangle)
            .chain(self.iter_horizontal_projections(rectangle))
            .chain(self.iter_vertical_projections(rectangle))
            .all(|tile| self.is_green_tile(tile, visited))
    }

    fn iter_interior_red_tiles<'a>(
        &'a self,
        rectangle: &'a Rectangle,
    ) -> impl Iterator<Item = Tile> + 'a {
        self.red_tiles
            .iter()
            .filter_map(move |red_tile| {
                if red_tile.is_strictly_inside(rectangle) {
                    Some(red_tile.diagonal_neighbors())
                } else {
                    None
                }
            })
            .flatten()
    }

    fn iter_horizontal_projections<'a>(
        &'a self,
        rectangle: &'a Rectangle,
    ) -> impl Iterator<Item = Tile> + 'a {
        self.red_tiles
            .iter()
            .filter_map(move |red_tile| {
                if red_tile.y > rectangle.top_left.y && red_tile.y < rectangle.bottom_right.y {
                    if red_tile.x <= rectangle.top_left.x {
                        return Some([
                            Tile::from(rectangle.top_left.x, red_tile.y + 1),
                            Tile::from(rectangle.top_left.x, red_tile.y - 1),
                        ]);
                    } else if red_tile.x >= rectangle.bottom_right.x {
                        return Some([
                            Tile::from(rectangle.bottom_right.x, red_tile.y + 1),
                            Tile::from(rectangle.bottom_right.x, red_tile.y - 1),
                        ]);
                    }
                }
                None
            })
            .flatten()
    }

    fn iter_vertical_projections<'a>(
        &'a self,
        rectangle: &'a Rectangle,
    ) -> impl Iterator<Item = Tile> + 'a {
        self.red_tiles
            .iter()
            .filter_map(move |red_tile| {
                if red_tile.x > rectangle.top_left.x && red_tile.x < rectangle.bottom_right.x {
                    if red_tile.y <= rectangle.top_left.y {
                        return Some([
                            Tile::from(red_tile.x + 1, rectangle.top_left.y),
                            Tile::from(red_tile.x - 1, rectangle.top_left.y),
                        ]);
                    } else if red_tile.y >= rectangle.bottom_right.y {
                        return Some([
                            Tile::from(red_tile.x + 1, rectangle.bottom_right.y),
                            Tile::from(red_tile.x - 1, rectangle.bottom_right.y),
                        ]);
                    }
                }
                None
            })
            .flatten()
    }

    fn is_green_tile(&self, tile: Tile, visited: &mut HashMap<Tile, bool>) -> bool {
        visited.get(&tile).copied().unwrap_or_else(|| {
            let inside = self.is_inside_polygon(tile);
            visited.insert(tile, inside);
            inside
        })
    }

    fn is_inside_polygon(&self, tile: Tile) -> bool {
        let mut inside = false;
        let n = self.red_tiles.len();

        for i in 0..n {
            let Tile { x: x1, y: y1 } = self.red_tiles[i];
            let Tile { x: x2, y: y2 } = self.red_tiles[(i + 1) % n];

            if (y1 > tile.y) != (y2 > tile.y) {
                let x_intersect = (i64::from(x2) - i64::from(x1))
                    * (i64::from(tile.y) - i64::from(y1))
                    / (i64::from(y2) - i64::from(y1))
                    + i64::from(x1);
                if i64::from(tile.x) < x_intersect {
                    inside = !inside;
                }
            }
        }
        inside
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn p1_basic_usecase() {
        let input = vec![
            "7,1".to_string(),
            "11,1".to_string(),
            "11,7".to_string(),
            "9,7".to_string(),
            "9,5".to_string(),
            "2,5".to_string(),
            "2,3".to_string(),
            "7,3".to_string(),
        ];
        assert_eq!(p1(&input), 50); // (2,5) to (11,1) = 10 * 5
    }

    #[test]
    fn p2_basic_usecase() {
        let input = vec![
            "7,1".to_string(),
            "11,1".to_string(),
            "11,7".to_string(),
            "9,7".to_string(),
            "9,5".to_string(),
            "2,5".to_string(),
            "2,3".to_string(),
            "7,3".to_string(),
        ];
        assert_eq!(p2(&input), 24); // (9,5) to (2,3) = 8 * 3
    }

    #[test]
    fn p2_real_usecase() {
        let input = read_input().unwrap();
        assert_eq!(p2(&input), 1_540_060_480);
    }
}
