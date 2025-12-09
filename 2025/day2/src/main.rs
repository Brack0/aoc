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

fn p1(input: &[String]) -> u64 {
    let mut sum = 0;

    for range in input[0].split(',') {
        let (start, end) = range.split_once('-').unwrap();
        let start = start.parse::<u64>().unwrap();
        let end = end.parse::<u64>().unwrap();
        (start..=end)
            .filter(|&id| has_sequence_repeated_twice(id))
            .for_each(|id| sum += id);
    }

    sum
}

fn p2(input: &[String]) -> u64 {
    let mut sum = 0;

    for range in input[0].split(',') {
        let (start, end) = range.split_once('-').unwrap();
        let start = start.parse::<u64>().unwrap();
        let end = end.parse::<u64>().unwrap();
        (start..=end)
            .filter(|&id| has_sequence_repeated(id))
            .for_each(|id| sum += id);
    }

    sum
}

fn has_sequence_repeated_twice(id: u64) -> bool {
    let id_string = id.to_string();
    let (p1, p2) = id_string.split_at(id_string.len() / 2);

    p1 == p2
}

fn has_sequence_repeated(id: u64) -> bool {
    let id_string = id.to_string();
    let len = id_string.len();

    (1..=len / 2)
        .rev()
        .filter(|chunk_size| len.is_multiple_of(*chunk_size))
        .any(|chunk_size| {
            let first_chunk = &id_string.as_bytes()[..chunk_size];

            id_string
                .as_bytes()
                .chunks(chunk_size)
                .all(|chunk| chunk == first_chunk)
        })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_not_have_sequence_repeated_twice_for_54() {
        assert!(!has_sequence_repeated_twice(54));
    }

    #[test]
    fn should_have_sequence_repeated_twice_for_55() {
        assert!(has_sequence_repeated_twice(55));
    }

    #[test]
    fn should_have_sequence_repeated_twice_for_6464() {
        assert!(has_sequence_repeated_twice(6464));
    }

    #[test]
    fn should_have_sequence_repeated_twice_for_123123() {
        assert!(has_sequence_repeated_twice(123_123));
    }

    #[test]
    fn should_not_have_sequence_repeated_twice_for_101() {
        assert!(!has_sequence_repeated_twice(101));
    }

    #[test]
    fn p1_should_sum_invalid_in_11_22() {
        assert_eq!(p1(&["11-22".to_string()]), 33);
    }

    #[test]
    fn p1_should_sum_invalid_in_95_115() {
        assert_eq!(p1(&["95-115".to_string()]), 99);
    }

    #[test]
    fn p1_should_sum_invalid_in_998_1012() {
        assert_eq!(p1(&["998-1012".to_string()]), 1010);
    }

    #[test]
    fn p1_should_sum_invalid_in_1188511880_1188511890() {
        assert_eq!(p1(&["1188511880-1188511890".to_string()]), 1_188_511_885);
    }

    #[test]
    fn p1_should_sum_invalid_in_222220_222224() {
        assert_eq!(p1(&["222220-222224".to_string()]), 222_222);
    }

    #[test]
    fn p1_should_sum_invalid_in_1698522_1698528() {
        assert_eq!(p1(&["1698522-1698528".to_string()]), 0);
    }

    #[test]
    fn p1_should_sum_invalid_in_446443_446449() {
        assert_eq!(p1(&["446443-446449".to_string()]), 446_446);
    }

    #[test]
    fn p1_should_sum_invalid_in_38593856_38593862() {
        assert_eq!(p1(&["38593856-38593862".to_string()]), 38_593_859);
    }

    #[test]
    fn p1_basic_usecase() {
        assert_eq!(
            p1(&[
                "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124" .to_string()
            ]),
            1_227_775_554
        );
    }

    #[test]
    fn should_not_have_sequence_repeated_for_54() {
        assert!(!has_sequence_repeated(54));
    }

    #[test]
    fn should_have_sequence_repeated_for_55() {
        assert!(has_sequence_repeated(55));
    }

    #[test]
    fn should_have_sequence_repeated_for_6464() {
        assert!(has_sequence_repeated(6464));
    }

    #[test]
    fn should_have_sequence_repeated_for_123123() {
        assert!(has_sequence_repeated(123_123));
    }

    #[test]
    fn should_not_have_sequence_repeated_for_101() {
        assert!(!has_sequence_repeated(101));
    }

    #[test]
    fn should_have_sequence_repeated_for_12341234() {
        assert!(has_sequence_repeated(12_341_234));
    }

    #[test]
    fn should_have_sequence_repeated_for_123123123() {
        assert!(has_sequence_repeated(123_123_123));
    }

    #[test]
    fn should_have_sequence_repeated_for_1212121212() {
        assert!(has_sequence_repeated(1_212_121_212));
    }

    #[test]
    fn should_have_sequence_repeated_for_1111111() {
        assert!(has_sequence_repeated(1_111_111));
    }

    #[test]
    fn p2_should_sum_invalid_in_11_22() {
        assert_eq!(p2(&["11-22".to_string()]), 33);
    }

    #[test]
    fn p2_should_sum_invalid_in_95_115() {
        assert_eq!(p2(&["95-115".to_string()]), 210);
    }

    #[test]
    fn p2_should_sum_invalid_in_998_1012() {
        assert_eq!(p2(&["998-1012".to_string()]), 2009);
    }

    #[test]
    fn p2_should_sum_invalid_in_1188511880_1188511890() {
        assert_eq!(p2(&["1188511880-1188511890".to_string()]), 1_188_511_885);
    }

    #[test]
    fn p2_should_sum_invalid_in_222220_222224() {
        assert_eq!(p2(&["222220-222224".to_string()]), 222_222);
    }

    #[test]
    fn p2_should_sum_invalid_in_1698522_1698528() {
        assert_eq!(p2(&["1698522-1698528".to_string()]), 0);
    }

    #[test]
    fn p2_should_sum_invalid_in_446443_446449() {
        assert_eq!(p2(&["446443-446449".to_string()]), 446_446);
    }

    #[test]
    fn p2_should_sum_invalid_in_38593856_38593862() {
        assert_eq!(p2(&["38593856-38593862".to_string()]), 38_593_859);
    }

    #[test]
    fn p2_should_sum_invalid_in_565653_565659() {
        assert_eq!(p2(&["565653-565659".to_string()]), 565_656);
    }

    #[test]
    fn p2_should_sum_invalid_in_824824821_824824827() {
        assert_eq!(p2(&["824824821-824824827".to_string()]), 824_824_824);
    }

    #[test]
    fn p2_should_sum_invalid_in_2121212118_2121212124() {
        assert_eq!(p2(&["2121212118-2121212124".to_string()]), 2_121_212_121);
    }

    #[test]
    fn p2_basic_usecase() {
        assert_eq!(
            p2( &[
                "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124" .to_string()
            ]),            4_174_379_265
        );
    }
}
