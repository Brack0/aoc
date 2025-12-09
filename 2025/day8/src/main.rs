use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    mem::swap,
};

fn main() -> io::Result<()> {
    let input = read_input()?;

    let p1_result = p1(&input, 1000);

    println!("Part 1 result: {p1_result}");

    let p2_result = p2(&input);

    println!("Part 2 result: {p2_result}");

    Ok(())
}

fn read_input() -> io::Result<Vec<String>> {
    let file = File::open("input/raw.txt")?;
    BufReader::new(file).lines().collect::<Result<_, _>>()
}

fn p1(input: &[String], max_connections: usize) -> usize {
    let junction_boxes = input.iter().map(JunctionBox::from).collect::<Vec<_>>();
    let mut circuits = Circuits::from(junction_boxes.len());
    let mut connections = Connections::from(&junction_boxes);

    connections.sort_by_distance();

    for (a, b, _) in connections.0.iter().take(max_connections) {
        circuits.connect(*a, *b);
    }

    circuits.iter_by_size().rev().take(3).product::<usize>()
}

fn p2(input: &[String]) -> u64 {
    let junction_boxes = input.iter().map(JunctionBox::from).collect::<Vec<_>>();
    let mut circuits = Circuits::from(junction_boxes.len());
    let mut connections = Connections::from(&junction_boxes);

    connections.sort_by_distance();

    for (index, (a, b, _)) in connections.0.iter().enumerate() {
        circuits.connect(*a, *b);
        // At least n-1 connections are needed to connect n junction boxes
        if index >= junction_boxes.len() - 1 && circuits.is_all_connected() {
            return (junction_boxes[*a].x) * (junction_boxes[*b].x);
        }
    }

    0
}

struct JunctionBox {
    x: u64,
    y: u64,
    z: u64,
}

impl From<&String> for JunctionBox {
    fn from(value: &String) -> Self {
        let nums: Vec<u64> = value.split(',').map(|num| num.parse().unwrap()).collect();
        JunctionBox {
            x: nums[0],
            y: nums[1],
            z: nums[2],
        }
    }
}

impl JunctionBox {
    fn distance(&self, other: &JunctionBox) -> u64 {
        let dx = self.x.abs_diff(other.x);
        let dy = self.y.abs_diff(other.y);
        let dz = self.z.abs_diff(other.z);
        dx * dx + dy * dy + dz * dz
    }
}

struct Connections(Vec<(usize, usize, u64)>);

impl From<&Vec<JunctionBox>> for Connections {
    fn from(value: &Vec<JunctionBox>) -> Self {
        let mut connections = Vec::with_capacity((value.len() * (value.len() - 1)) / 2);
        for (i, a_box) in value.iter().enumerate() {
            for (j, b_box) in value.iter().enumerate().skip(i + 1) {
                let distance = a_box.distance(b_box);
                connections.push((i, j, distance));
            }
        }

        Connections(connections)
    }
}

impl Connections {
    fn sort_by_distance(&mut self) {
        self.0.sort_by_key(|&(_, _, distance)| distance);
    }
}

/**
 * Union-Find structure
 */
struct Circuits {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl From<usize> for Circuits {
    /**
     * `MakeSet`
     */
    fn from(n: usize) -> Self {
        Circuits {
            // Memory optimization: preallocate parent and size vectors
            parent: (0..n).collect(),
            size: vec![1; n],
        }
    }
}

impl Circuits {
    /**
     * `Find`
     */
    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    /**
     * `Union`
     */
    fn connect(&mut self, x: usize, y: usize) {
        let (mut x, mut y) = (self.find(x), self.find(y));
        if x != y {
            if self.size[x] < self.size[y] {
                swap(&mut x, &mut y);
            }
            self.parent[y] = x;
            self.size[x] += self.size[y];
        }
    }

    fn sizes(&mut self) -> Vec<usize> {
        let mut circuits = vec![];
        for i in 0..self.parent.len() {
            if self.find(i) == i {
                circuits.push(self.size[i]);
            }
        }
        circuits
    }

    fn iter_by_size(&mut self) -> impl DoubleEndedIterator<Item = usize> {
        let mut circuits = self.sizes();
        circuits.sort_unstable();
        circuits.into_iter()
    }

    fn is_all_connected(&mut self) -> bool {
        self.sizes().len() == 1
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn p1_tiny_usecase() {
        let input = vec![
            "0,0,0".to_string(),
            "1,0,0".to_string(),
            "0,2,0".to_string(),
            "0,0,9".to_string(),
            "0,5,6".to_string(),
        ];

        let result = p1(&input, 2);

        assert_eq!(result, 3); // = 3 * 1 * 1
        // Explanation : <(0,0,0>,(1,0,0),(0,2,0)> * <(0,0,9)> * <(0,5,6)>
    }

    #[test]
    fn p1_basic_usecase() {
        let input = vec![
            "162,817,812".to_string(),
            "57,618,57".to_string(),
            "906,360,560".to_string(),
            "592,479,940".to_string(),
            "352,342,300".to_string(),
            "466,668,158".to_string(),
            "542,29,236".to_string(),
            "431,825,988".to_string(),
            "739,650,466".to_string(),
            "52,470,668".to_string(),
            "216,146,977".to_string(),
            "819,987,18".to_string(),
            "117,168,530".to_string(),
            "805,96,715".to_string(),
            "346,949,466".to_string(),
            "970,615,88".to_string(),
            "941,993,340".to_string(),
            "862,61,35".to_string(),
            "984,92,344".to_string(),
            "425,690,689".to_string(),
        ];

        let result = p1(&input, 10);

        assert_eq!(result, 40);
    }

    #[test]
    fn p2_tiny_usecase() {
        // Connection order : (a,b), (a,c), (a,d)
        let input = vec![
            "10,10,10".to_string(), // a
            "10,10,11".to_string(), // b
            "10,12,10".to_string(), // c
            "13,10,10".to_string(), // d
        ];

        let result = p2(&input);

        assert_eq!(result, 130); // Last connection <(10,10,10),(13,10,10)> => x1 * x2 => 130
    }

    #[test]
    fn p2_tiny_usecase_with_one_more_connection() {
        // Connection order : (a,b), (a,c), (b,c), (a,d)
        let input = vec![
            "10,10,10".to_string(), // a
            "10,11,10".to_string(), // b
            "10,10,11".to_string(), // c
            "30,10,10".to_string(), // d
        ];

        let result = p2(&input);

        assert_eq!(result, 300); // Last connection <(10,10,10),(30,10,10)> => x1 * x2 => 300
    }

    #[test]
    fn p2_basic_usecase() {
        let input = vec![
            "162,817,812".to_string(),
            "57,618,57".to_string(),
            "906,360,560".to_string(),
            "592,479,940".to_string(),
            "352,342,300".to_string(),
            "466,668,158".to_string(),
            "542,29,236".to_string(),
            "431,825,988".to_string(),
            "739,650,466".to_string(),
            "52,470,668".to_string(),
            "216,146,977".to_string(),
            "819,987,18".to_string(),
            "117,168,530".to_string(),
            "805,96,715".to_string(),
            "346,949,466".to_string(),
            "970,615,88".to_string(),
            "941,993,340".to_string(),
            "862,61,35".to_string(),
            "984,92,344".to_string(),
            "425,690,689".to_string(),
        ];

        let result = p2(&input);

        assert_eq!(result, 25272); // Last connection <(216,146,977),(117,168,530)> => x1 * x2 => 25272
    }
}
