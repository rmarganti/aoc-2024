fn main() {
    let input = shared::read_file_from_args();
    let puzzle = Puzzle::from(input.as_str());

    let part_one = part_one(&puzzle);
    println!("part one: {}", part_one);

    let part_two = part_two(&puzzle);
    println!("part one: {}", part_two);
}

fn part_one(puzzle: &Puzzle) -> usize {
    solve_puzzle(puzzle, false)
}

fn part_two(puzzle: &Puzzle) -> usize {
    solve_puzzle(puzzle, true)
}

fn solve_puzzle(puzzle: &Puzzle, allow_concat: bool) -> usize {
    puzzle
        .data
        .iter()
        .filter(|(test_value, inputs)| has_valid_operator_combo(*test_value, inputs, allow_concat))
        .fold(0, |acc, (test_value, _)| acc + test_value)
}

fn has_valid_operator_combo(test_value: usize, inputs: &[usize], allow_concat: bool) -> bool {
    let output = all_possible_outputs(inputs, allow_concat);
    output.iter().any(|&val| val == test_value)
}

/// Find all possible equation output. For example:
///
/// input:
/// [2, 4, 6]
///
/// outputs (in order, ignore order of operation):
/// 2 + 4 + 6 = 12
/// 2 + 4 * 6 = 36
/// 2 * 4 + 6 = 14
/// 2 * 4 * 6 = 48
///
/// Final result:
/// vec![6, 9, 5, 6]
fn all_possible_outputs(inputs: &[usize], allow_concat: bool) -> Vec<usize> {
    inputs.iter().fold(vec![0], |outputs, &val| {
        let mut new_outputs = vec![];
        for &acc_val in outputs.iter() {
            new_outputs.push(acc_val + val);
            new_outputs.push(acc_val * val);

            if allow_concat {
                new_outputs.push(concat_number_strings(acc_val, val));
            }
        }
        new_outputs
    })
}

/// Concatenate two numbers together. For example:
/// 2, 4 -> 24
/// 10, 5 -> 105
fn concat_number_strings(a: usize, b: usize) -> usize {
    let mut offset = 1;

    while offset <= b {
        offset *= 10;
    }

    a * offset + b
}

#[derive(Debug)]
struct Puzzle {
    data: Vec<(usize, Vec<usize>)>,
}

impl From<&str> for Puzzle {
    fn from(input: &str) -> Puzzle {
        let data = input
            .trim()
            .lines()
            .map(|line| {
                let line = line.trim();
                let (test_value, inputs) = line.split_once(": ").expect("Invalid puzzle line");

                let test_value = test_value.parse().expect("Unable to parse test value");
                let inputs = inputs
                    .split_whitespace()
                    .map(|val| val.parse().expect("Unable to parse input value"))
                    .collect();

                (test_value, inputs)
            })
            .collect();

        Puzzle { data }
    }
}

#[cfg(test)]

mod tests {
    use super::*;

    const INPUT: &str = r#"
        190: 10 19
        3267: 81 40 27
        83: 17 5
        156: 15 6
        7290: 6 8 6 15
        161011: 16 10 13
        192: 17 8 14
        21037: 9 7 18 13
        292: 11 6 16 20
    "#;

    #[test]
    fn test_part_one() {
        let puzzle = Puzzle::from(INPUT);
        let result = part_one(&puzzle);

        assert_eq!(result, 3749);
    }

    #[test]
    fn test_part_two() {
        let puzzle = Puzzle::from(INPUT);
        let result = part_two(&puzzle);

        assert_eq!(result, 11387);
    }
}
