use std::{env, fs};

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

fn read_file_from_args() -> String {
    let filename = env::args()
        .nth(1)
        .expect("Please provide a filename as an argument");

    fs::read_to_string(filename).expect("Failed to read file")
}

fn part_one(left_list: &[u32], right_list: &[u32]) -> u32 {
    left_list
        .iter()
        .zip(right_list)
        .map(|(l, &r)| l.abs_diff(r))
        .sum()
}

fn part_two(left_list: &[u32], right_list: &[u32]) -> u32 {
    left_list
        .iter()
        .map(|left_val| {
            let found_count = right_list.iter().filter(|&x| x == left_val).count() as u32;
            found_count * left_val
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
