use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
    fs::File,
    io::{self, BufRead, BufReader},
};

use good_lp::{Expression, Solution, SolverModel, default_solver, variable, variables};

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
    input
        .iter()
        .map(|line| Machine::from(line).configure_lights())
        .sum::<u32>()
}

fn p2(input: &[String]) -> u32 {
    input
        .iter()
        .map(|line| Machine::from(line).configure_joltages())
        .sum::<u32>()
}

fn slice_to_bitmask(slice: &[bool]) -> u16 {
    slice
        .iter()
        .enumerate()
        .fold(0, |acc, (i, &on)| if on { acc | (1 << i) } else { acc })
}

fn indices_to_bitmask(indices: &[usize]) -> u16 {
    indices.iter().fold(0, |acc, &i| acc | (1 << i))
}

struct Machine {
    indicator_lights: Vec<bool>,
    buttons: Vec<Vec<usize>>,
    joltage_requirements: [Joltage; MAX_JOLTAGES],
}

type Joltage = u16;
const MAX_JOLTAGES: usize = 10;

impl Machine {
    /**
     * BFS to find the minimum number of button presses to match the indicator lights
     */
    fn configure_lights(&self) -> u32 {
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();

        let starting_lights = 0u16; // Bitmask
        let indicator_lights = slice_to_bitmask(&self.indicator_lights);
        let buttons = self
            .buttons
            .iter()
            .map(|indices| indices_to_bitmask(indices))
            .collect::<Vec<u16>>();

        visited.insert(starting_lights);
        queue.push_back((starting_lights, 0));

        while let Some((lights, buttons_pressed)) = queue.pop_front() {
            if lights == indicator_lights {
                return buttons_pressed;
            }

            for button in &buttons {
                let new_lights = lights ^ *button;
                if visited.insert(new_lights) {
                    queue.push_back((new_lights, buttons_pressed + 1));
                }
            }
        }

        u32::MAX
    }

    /**
     * Boring linear programming to find the minimum number of button presses to meet the joltage requirements
     */
    fn configure_joltages(&self) -> u32 {
        let mut vars = variables!();
        let button_vars = (0..self.buttons.len())
            .map(|_| vars.add(variable().integer().min(0)))
            .collect::<Vec<_>>();

        let mut problem = vars
            .minimise(button_vars.iter().sum::<Expression>())
            .using(default_solver);

        // Don't talk to me, you already ruined the fun of solving this with A*
        problem.set_parameter("log", "0");

        for (joltage_idx, &required_joltage) in self.joltage_requirements.iter().enumerate() {
            let mut expr = Expression::from(0);
            for (button_idx, button) in self.buttons.iter().enumerate() {
                if button.contains(&joltage_idx) {
                    expr += button_vars[button_idx];
                }
            }
            problem = problem.with(expr.eq(required_joltage));
        }

        let solution = problem.solve().unwrap();

        // Cast is safe as constraints above ensure non-negative integers
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        button_vars
            .iter()
            .map(|var| solution.value(*var) as u32)
            .sum()
    }

    /**
     * A* to find the minimum number of button presses to meet the joltage requirements
     * Unused because linear programming is mandatory
     * But I kept it because I like it and I need to train A*
     */
    #[allow(dead_code)]
    fn configure_joltages_a_star(&self) -> u32 {
        let mut open_set = BinaryHeap::new(); // Min-heap using Reverse on BinaryHeap
        let mut g_score = HashMap::new();

        let starting_joltages = [0u16; MAX_JOLTAGES];

        g_score.insert(starting_joltages, 0);
        open_set.push(Reverse((0, 0, starting_joltages)));

        while let Some(Reverse((_priority, buttons_pressed, joltages))) = open_set.pop() {
            if joltages == self.joltage_requirements {
                return buttons_pressed;
            }

            for button in &self.buttons {
                let new_joltages = self.press_joltages_button(joltages, button);

                if let Some(new_joltages) = new_joltages {
                    let new_buttons_pressed = buttons_pressed + 1;
                    if g_score
                        .get(&new_joltages)
                        .is_none_or(|&score| new_buttons_pressed < score)
                    {
                        g_score.insert(new_joltages, new_buttons_pressed);
                        let priority = new_buttons_pressed + self.joltage_heuristic(&new_joltages);
                        open_set.push(Reverse((priority, buttons_pressed + 1, new_joltages)));
                    }
                }
            }
        }

        u32::MAX
    }

    fn press_joltages_button(
        &self,
        current_joltages: [Joltage; MAX_JOLTAGES],
        button: &[usize],
    ) -> Option<[Joltage; MAX_JOLTAGES]> {
        let mut new_joltages = current_joltages;
        for &idx in button {
            new_joltages[idx] += 1;

            if new_joltages[idx] > self.joltage_requirements[idx] {
                return None;
            }
        }
        Some(new_joltages)
    }

    fn joltage_heuristic(&self, current: &[Joltage]) -> u32 {
        current
            .iter()
            .zip(self.joltage_requirements.iter())
            .map(|(&c, &r)| u32::from(c.abs_diff(r)))
            .sum()
    }
}

impl From<&String> for Machine {
    fn from(s: &String) -> Self {
        let parts = s.split_whitespace().collect::<Vec<&str>>();

        let indicator_lights = parts
            .first()
            .unwrap()
            .trim_start_matches('[')
            .trim_end_matches(']')
            .chars()
            .filter_map(|c| match c {
                '.' => Some(false),
                '#' => Some(true),
                _ => None,
            })
            .collect::<Vec<bool>>();

        let mut joltage_requirements = [0; MAX_JOLTAGES];
        parts
            .last()
            .unwrap()
            .trim_start_matches('{')
            .trim_end_matches('}')
            .split(',')
            .filter_map(|num_str| num_str.parse::<Joltage>().ok())
            .take(10)
            .enumerate()
            .for_each(|(index, joltage)| {
                joltage_requirements[index] = joltage;
            });

        let buttons = parts
            .iter()
            .skip(1)
            .take(parts.len() - 2)
            .map(|btn_str| {
                btn_str
                    .trim_start_matches('(')
                    .trim_end_matches(')')
                    .split(',')
                    .filter_map(|idx_str| idx_str.parse::<usize>().ok())
                    .collect::<Vec<usize>>()
            })
            .collect::<Vec<Vec<usize>>>();

        Machine {
            indicator_lights,
            buttons,
            joltage_requirements,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn p1_configure_lights_0() {
        let machine =
            Machine::from(&"[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}".to_string());

        assert_eq!(machine.configure_lights(), 2);
    }

    #[test]
    fn p1_configure_lights_1() {
        let machine = Machine::from(
            &"[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}".to_string(),
        );

        assert_eq!(machine.configure_lights(), 3);
    }

    #[test]
    fn p1_configure_lights_2() {
        let machine = Machine::from(
            &"[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}".to_string(),
        );

        assert_eq!(machine.configure_lights(), 2);
    }

    #[test]
    fn p1_basic_usecase() {
        let input = vec![
            "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}".to_string(),
            "[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}".to_string(),
            "[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}".to_string(),
        ];

        let result = p1(&input);

        assert_eq!(result, 7);
    }

    #[test]
    fn p2_configure_joltages_a_star_0() {
        let machine =
            Machine::from(&"[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}".to_string());

        assert_eq!(machine.configure_joltages_a_star(), 10);
    }

    #[test]
    fn p2_configure_joltages_a_star_1() {
        let machine = Machine::from(
            &"[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}".to_string(),
        );

        assert_eq!(machine.configure_joltages_a_star(), 12);
    }

    #[test]
    fn p2_configure_joltages_a_star_2() {
        let machine = Machine::from(
            &"[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}".to_string(),
        );

        assert_eq!(machine.configure_joltages_a_star(), 11);
    }

    #[test]
    fn p2_basic_usecase() {
        let input = vec![
            "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}".to_string(),
            "[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}".to_string(),
            "[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}".to_string(),
        ];

        let result = p2(&input);

        assert_eq!(result, 33);
    }
}
