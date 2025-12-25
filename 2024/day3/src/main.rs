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
    Instructions::from(input)
        .mul
        .iter()
        .map(|(a, b)| a * b)
        .sum::<u32>()
}

fn p2(input: &[String]) -> u32 {
    Instructions::from(input)
        .conditional_mul
        .iter()
        .map(|(a, b)| a * b)
        .sum::<u32>()
}

struct Instructions {
    mul: Vec<(u32, u32)>,
    conditional_mul: Vec<(u32, u32)>,
}

impl From<&[String]> for Instructions {
    fn from(input: &[String]) -> Self {
        let mut mul = Vec::new();
        let mut conditional_mul = Vec::new();
        let mut in_conditional = true;

        for line in input {
            let bytes = line.as_bytes();
            let mut i = 0;

            while i < bytes.len() {
                if bytes[i..].starts_with(b"do()") {
                    in_conditional = true;
                    i += 4;
                    continue;
                }
                if bytes[i..].starts_with(b"don't()") {
                    in_conditional = false;
                    i += 7;
                    continue;
                }

                if bytes[i..].starts_with(b"mul(") {
                    i += 4;

                    let num1_start = i;
                    while i < bytes.len() && bytes[i].is_ascii_digit() && i - num1_start < 3 {
                        i += 1;
                    }

                    if i == num1_start || i >= bytes.len() || bytes[i] != b',' {
                        continue;
                    }
                    let num1_str = &line[num1_start..i];
                    i += 1;

                    let num2_start = i;
                    while i < bytes.len() && bytes[i].is_ascii_digit() && i - num2_start < 3 {
                        i += 1;
                    }

                    if i == num2_start || i >= bytes.len() || bytes[i] != b')' {
                        continue;
                    }
                    let num2_str = &line[num2_start..i];

                    if let (Ok(a), Ok(b)) = (num1_str.parse::<u32>(), num2_str.parse::<u32>()) {
                        mul.push((a, b));
                        if in_conditional {
                            conditional_mul.push((a, b));
                        }
                    }
                } else {
                    i += 1;
                }
            }
        }

        Instructions {
            mul,
            conditional_mul,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn p1_basic_usecase() {
        let input = vec![String::from(
            "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))",
        )];

        let result = p1(&input);

        assert_eq!(result, 161);
    }

    #[test]
    fn p2_basic_usecase() {
        let input = vec![String::from(
            "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))",
        )];

        let result = p2(&input);

        assert_eq!(result, 48);
    }
}
