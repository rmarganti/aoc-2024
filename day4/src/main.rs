mod puzzle;

use puzzle::Puzzle;

fn main() {
    let input = shared::read_file_from_args();
    let puzzle = parse_input(&input);

    let result_one = part_one(&puzzle);
    println!("Part 1: {}", result_one);

    let result_two = part_two(&puzzle);
    println!("Part 2: {}", result_two);
}

fn part_one(puzzle: &Puzzle) -> usize {
    puzzle
        .iter_coords()
        .fold(0, |acc, (x, y)| acc + puzzle.count_xmas_instances_at(x, y))
}

fn part_two(input: &Puzzle) -> usize {
    0
}

fn parse_input(input: &str) -> Puzzle {
    Puzzle::new(
        input
            .lines()
            .map(|line| line.trim().chars().collect())
            .collect(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"
        MMMSXXMASM
        MSAMXMSMSA
        AMXSXMAAMM
        MSAMASMSMX
        XMASAMXAMM
        XXAMMXXAMA
        SMSMSASXSS
        SAXAMASAAA
        MAMMMXMMMM
        MXMXAXMASX
    "#;

    #[test]
    fn test_part_one() {
        let puzzle = parse_input(INPUT.trim());
        let result = part_one(&puzzle);

        assert_eq!(result, 18);
    }

    #[test]
    fn test_part_two() {
        let puzzle = parse_input(INPUT.trim());
        let result = part_two(&puzzle);

        assert_eq!(result, 0);
    }
}
