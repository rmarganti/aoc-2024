use std::{collections::HashMap, env, fs};

fn main() {
    let input = read_file_from_args();
    let (mut left_list, mut right_list) = parse_lists(&input);
    left_list.sort();
    right_list.sort();

    let total_distance = part_one(&left_list, &right_list);
    println!("Total distance: {}", total_distance);

    let total_similarity_scores = part_two(&left_list, &right_list);
    println!("Total similarity scores: {}", total_similarity_scores);
}

fn read_file_from_args() -> String {
    let filename = env::args()
        .nth(1)
        .expect("Please provide a filename as an argument");

    fs::read_to_string(filename).expect("Failed to read file")
}

fn parse_lists(input: &str) -> (Vec<u32>, Vec<u32>) {
    let (mut left_list, mut right_list): (Vec<u32>, Vec<u32>) = input
        .trim()
        .lines()
        .map(|line| {
            let line = line.trim();
            let mut pieces = line.split_whitespace();

            let left: u32 = pieces
                .next()
                .and_then(|x| x.parse().ok())
                .expect("Invalid first item");

            let right: u32 = pieces
                .next()
                .and_then(|x| x.parse().ok())
                .expect("Invalid second item");

            (left, right)
        })
        .unzip();

    left_list.sort();
    right_list.sort();

    (left_list, right_list)
}

fn part_one(left_list: &[u32], right_list: &[u32]) -> u32 {
    left_list
        .iter()
        .zip(right_list)
        .map(|(l, &r)| l.abs_diff(r))
        .sum()
}

fn part_two(list_one: &[u32], list_two: &[u32]) -> u32 {
    // Count occurrences of each value in list_two
    let mut value_counts = HashMap::<u32, u32>::new();

    for &val in list_two {
        *value_counts.entry(val).or_insert(0) += 1;
    }

    // Calculate similarity scores
    list_one
        .iter()
        .map(|&val| {
            // Get the count of this value in list_two, defaulting to 0
            let found_count = *value_counts.get(&val).unwrap_or(&0);
            found_count * val
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = r#"
        3   4
        4   3
        2   5
        1   3
        3   9
        3   3
    "#;

    #[test]
    fn part_one_works_on_the_example_input() {
        let (left_list, right_list) = parse_lists(EXAMPLE_INPUT);
        let total_distance = part_one(&left_list, &right_list);

        assert_eq!(total_distance, 11)
    }

    #[test]
    fn part_two_works_on_the_example_input() {
        let (left_list, right_list) = parse_lists(EXAMPLE_INPUT);
        let total_similarity_scores = part_two(&left_list, &right_list);

        assert_eq!(total_similarity_scores, 31)
    }
}
