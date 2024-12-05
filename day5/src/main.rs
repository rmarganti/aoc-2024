use std::cmp;

fn main() {
    let input = shared::read_file_from_args();
    let puzzle = Puzzle::from(input.as_str());

    let part_one = part_one(&puzzle);
    println!("part one: {}", part_one);

    let part_two = part_two(&puzzle);
    println!("part one: {}", part_two);
}

fn part_one(puzzle: &Puzzle) -> usize {
    puzzle
        .updates
        .iter()
        .filter(|update| update.is_valid(&puzzle.rules))
        .map(middle_value)
        .sum()
}

fn part_two(puzzle: &Puzzle) -> usize {
    puzzle
        .updates
        .iter()
        .filter(|update| !update.is_valid(&puzzle.rules))
        .map(|update| update.clone_and_sort(&puzzle.rules))
        .map(|update| middle_value(&update))
        .sum()
}

fn middle_value(update: &Update) -> usize {
    update.pages[(update.pages.len() - 1) / 2]
}

#[derive(Debug)]
struct Puzzle {
    rules: Vec<PageRule>,
    updates: Vec<Update>,
}

impl From<&str> for Puzzle {
    fn from(input: &str) -> Self {
        let pieces = input.trim().split_once("\n\n").expect("Invalid input");

        let rules: Vec<(usize, usize)> = pieces
            .0
            .split("\n")
            .map(|rule| {
                let parts = rule.trim().split_once("|").expect("Invalid rule");
                let part_one = parts.0.parse::<usize>().expect("Invalid rule");
                let part_two = parts.1.parse::<usize>().expect("Invalid rule");
                (part_one, part_two)
            })
            .collect();

        let updates = pieces
            .1
            .trim()
            .lines()
            .map(|line| {
                let pages = line
                    .trim()
                    .split(',')
                    .map(|page| page.parse().expect("Invalid page"))
                    .collect();

                Update { pages }
            })
            .collect();

        Puzzle { rules, updates }
    }
}

type PageRule = (usize, usize);

#[derive(Clone, Debug)]
struct Update {
    pages: Vec<usize>,
}

impl Update {
    fn is_valid(&self, rules: &[PageRule]) -> bool {
        self.pages.iter().enumerate().all(|(page_idx, &page)| {
            rules.iter().filter(|rule| rule.0 == page).all(|rule| {
                self.pages
                    .iter()
                    .position(|&n| n == rule.1)
                    .map(|idx| page_idx < idx)
                    .unwrap_or(true)
            })
        })
    }

    fn clone_and_sort(&self, rules: &[PageRule]) -> Update {
        let mut pages = self.pages.clone();

        pages.sort_by(|&a, &b| {
            // Find a rule that applies to the pages being compared
            let rule = rules.iter().find(|rule| rule.0 == a && rule.1 == b);

            match rule {
                Some(_) => cmp::Ordering::Less,
                None => cmp::Ordering::Equal,
            }
        });

        Update { pages }
    }
}

impl<'a> From<&'a Vec<usize>> for Update {
    fn from(pages: &'a Vec<usize>) -> Self {
        Update {
            pages: pages.clone(),
        }
    }
}

#[cfg(test)]

mod tests {
    use super::*;

    const INPUT: &str = r#"
        47|53
        97|13
        97|61
        97|47
        75|29
        61|13
        75|53
        29|13
        97|29
        53|29
        61|53
        97|53
        61|29
        47|13
        75|47
        97|75
        47|61
        75|61
        47|29
        75|13
        53|13

        75,47,61,53,29
        97,61,53,29,13
        75,29,13
        75,97,47,61,53
        61,13,29
        97,13,75,29,47
    "#;

    #[test]
    fn test_part_one() {
        let puzzle = Puzzle::from(INPUT);
        let result = part_one(&puzzle);

        assert_eq!(result, 143);
    }

    #[test]
    fn test_part_two() {
        let puzzle = Puzzle::from(INPUT);
        let result = part_two(&puzzle);

        assert_eq!(result, 123);
    }
}
