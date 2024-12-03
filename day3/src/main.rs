use regex::Regex;

fn main() {
    let input = shared::read_file_from_args();

    let result_one = part_one(&input);
    println!("Part 1: {}", result_one);

    let result_two = part_two(&input);
    println!("Part 2: {}", result_two);
}

fn part_one(input: &str) -> usize {
    let re = Regex::new(r#"mul\((\d*),(\d*)\)"#).expect("Unable to parse RegEx");

    re.captures_iter(input)
        .map(|capture| {
            let a = capture[1]
                .parse::<usize>()
                .expect("Unable to parse first multiplier");
            let b = capture[2]
                .parse::<usize>()
                .expect("Unable to parse second multiplier");
            a * b
        })
        .sum()
}

fn part_two(input: &str) -> usize {
    let mut enabled = true;

    let re = Regex::new(r#"do\(\)|don't\(\)|mul\((\d*),(\d*)\)"#).unwrap();

    re.captures_iter(input)
        .map(|capture| match &capture[0] {
            "do()" => {
                enabled = true;
                0
            }
            "don't()" => {
                enabled = false;
                0
            }
            _ => {
                if enabled {
                    let a = capture[1]
                        .parse::<usize>()
                        .expect("Unable to parse first multiplier");
                    let b = capture[2]
                        .parse::<usize>()
                        .expect("Unable to parse second multiplier");
                    a * b
                } else {
                    0
                }
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const PART_ONE_EXAMPLE_INPUT: &str = r#"
        xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))
    "#;

    #[test]
    fn part_one_works_on_the_example_input() {
        let result = part_one(PART_ONE_EXAMPLE_INPUT.trim());
        assert_eq!(result, 161);
    }

    const PART_TWO_EXAMPLE_INPUT: &str = r#"
        xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))
    "#;

    #[test]
    fn part_two_works_on_the_example_input() {
        let result = part_two(PART_TWO_EXAMPLE_INPUT.trim());
        assert_eq!(result, 48);
    }
}
