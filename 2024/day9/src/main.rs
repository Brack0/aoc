use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
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
    let file = File::open(Path::new(env!("CARGO_MANIFEST_DIR")).join("input/raw.txt"))?;
    BufReader::new(file).lines().collect::<Result<_, _>>()
}

fn p1(_input: &[String]) -> usize {
    0
}

fn p2(_input: &[String]) -> usize {
    0
}

struct Disk {
    disk_map: Vec<usize>,
    blocks: String,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn p1_simple_example() {
        let input = vec![String::from("12345")];

        let result = p1(&input);

        assert_eq!(result, 60);
    }

    #[test]
    fn p1_basic_usecase() {
        let input = vec![String::from("2333133121414131402")];

        let result = p1(&input);

        assert_eq!(result, 1928);
    }

    #[test]
    fn p2_basic_usecase() {
        let input = vec![String::from("2333133121414131402")];

        let result = p2(&input);

        assert_eq!(result, 0);
    }
}
