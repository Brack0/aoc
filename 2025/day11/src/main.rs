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

const YOU: &str = "you";
const SVR: &str = "svr";
const DAC: &str = "dac";
const FFT: &str = "fft";
const END: &str = "out";

type Mask = u8;
const NO_MASK: Mask = 0;
const DAC_MASK: Mask = 1;
const FFT_MASK: Mask = 2;

fn p1(input: &[String]) -> usize {
    ServerRack::from(input).count_paths_from(YOU, NO_MASK, NO_MASK, &mut HashMap::new())
}

fn p2(input: &[String]) -> usize {
    ServerRack::from(input).count_paths_from(SVR, DAC_MASK | FFT_MASK, NO_MASK, &mut HashMap::new())
}

fn apply_mask(current_mask: Mask, device: &str) -> Mask {
    match device {
        DAC => current_mask | DAC_MASK,
        FFT => current_mask | FFT_MASK,
        _ => current_mask,
    }
}

struct ServerRack<'a> {
    device_connections: HashMap<&'a str, Vec<&'a str>>,
}

impl<'a> From<&'a [String]> for ServerRack<'a> {
    fn from(lines: &'a [String]) -> Self {
        let mut device_connections = HashMap::new();
        for line in lines {
            let parts: Vec<&str> = line.split(": ").collect();
            let device_name = parts[0];
            let connections: Vec<&str> = parts[1].split(' ').collect();
            device_connections.insert(device_name, connections);
        }

        ServerRack { device_connections }
    }
}

impl<'a> ServerRack<'a> {
    fn count_paths_from(
        &self,
        device: &'a str,
        mask: Mask,
        target_mask: Mask,
        visited: &mut HashMap<(&'a str, Mask), usize>,
    ) -> usize {
        if device == END {
            return usize::from((mask & target_mask) == target_mask);
        }

        let state = (device, mask);
        if let Some(&result) = visited.get(&state) {
            return result;
        }

        let next_mask = apply_mask(mask, device);

        let mut paths = 0;
        if let Some(children) = self.device_connections.get(device) {
            for &child in children {
                paths += self.count_paths_from(child, next_mask, target_mask, visited);
            }
        }

        visited.insert(state, paths);
        paths
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn p1_basic_usecase() {
        let input = vec![
            "aaa: you hhh".to_string(),
            "you: bbb ccc".to_string(),
            "bbb: ddd eee".to_string(),
            "ccc: ddd eee fff".to_string(),
            "ddd: ggg".to_string(),
            "eee: out".to_string(),
            "fff: out".to_string(),
            "ggg: out".to_string(),
            "hhh: ccc fff iii".to_string(),
            "iii: out".to_string(),
        ];

        let result = p1(&input);

        assert_eq!(result, 5);
    }

    #[test]
    fn p2_basic_usecase() {
        let input = vec![
            "svr: aaa bbb".to_string(),
            "aaa: fft".to_string(),
            "fft: ccc".to_string(),
            "bbb: tty".to_string(),
            "tty: ccc".to_string(),
            "ccc: ddd eee".to_string(),
            "ddd: hub".to_string(),
            "hub: fff".to_string(),
            "eee: dac".to_string(),
            "dac: fff".to_string(),
            "fff: ggg hhh".to_string(),
            "ggg: out".to_string(),
            "hhh: out".to_string(),
        ];

        let result = p2(&input);

        assert_eq!(result, 2);
    }
}
