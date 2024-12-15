use std::collections::HashMap;

fn main() {
    let input = shared::read_file_from_args();
    let puzzle = Puzzle::from(input.as_str());

    let part_one = part_one(&puzzle);
    println!("part one: {}", part_one);

    let part_two = part_two(&puzzle);
    println!("part one: {}", part_two);
}

fn part_one(puzzle: &Puzzle) -> usize {
    filter_for_valid_test_values(puzzle, false)
        .data
        .keys()
        .sum()
}

fn part_two(puzzle: &Puzzle) -> usize {
    filter_for_valid_test_values(puzzle, true).data.keys().sum()
}

fn filter_for_valid_test_values(puzzle: &Puzzle, allow_concat: bool) -> Puzzle {
    let data = puzzle
        .data
        .iter()
        .filter(|(&key, val)| has_valid_operator_combo(key, val, allow_concat))
        .fold(HashMap::default(), |mut acc, (&key, val)| {
            acc.insert(key, val.to_vec());
            acc
        });

    Puzzle { data }
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
    inputs.iter().fold(vec![0], |acc, &val| {
        let mut new_acc = vec![];
        for &acc_val in acc.iter() {
            new_acc.push(acc_val + val);
            new_acc.push(acc_val * val);

            if allow_concat {
                new_acc.push(concat_number_strings(acc_val, val));
            }
        }
        new_acc
    })
}

fn concat_number_strings(a: usize, b: usize) -> usize {
    format!("{}{}", a, b)
        .parse()
        .expect("Unable to parse concatenated number")
}

#[derive(Debug)]
struct Puzzle {
    data: HashMap<usize, Vec<usize>>,
}

impl From<&str> for Puzzle {
    fn from(input: &str) -> Puzzle {
        let mut data = HashMap::default();

        for line in input.trim().lines() {
            let line = line.trim();
            let (test_value, inputs) = line.split_once(": ").expect("Invalid puzzle line");

            let test_value: usize = test_value.parse().expect("Unable to parse test value");
            let inputs: Vec<usize> = inputs
                .split_whitespace()
                .map(|val| val.parse().expect("Unable to parse input value"))
                .collect();

            data.insert(test_value, inputs);
        }

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
