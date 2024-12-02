use std::{cmp, env, fs};

type Report = Vec<u8>;
type ReportList = Vec<Report>;

fn main() {
    let input = read_file_from_args();
    let reports = parse_reports(&input);

    let part_one = part_one(&reports);
    println!("Part 1: {}", part_one);
}

fn read_file_from_args() -> String {
    let filename = env::args()
        .nth(1)
        .expect("Please provide a filename as an argument");

    fs::read_to_string(filename).expect("Failed to read file")
}

fn parse_reports(input: &str) -> ReportList {
    input
        .trim()
        .lines()
        .map(|line| {
            line.trim()
                .split_whitespace()
                .map(|level| level.parse().expect("Unable to parse level"))
                .collect()
        })
        .collect()
}

fn part_one(reports: &ReportList) -> usize {
    reports
        .iter()
        .filter(|report| is_valid_report(*report))
        .count()
}

#[derive(Clone)]
enum ReportType {
    Unknown,
    Increasing,
    Decreasing,
}

fn is_valid_report(report: &Report) -> bool {
    let mut report_type = ReportType::Unknown;
    let mut previous: Option<&u8> = None;

    for level in report {
        match previous {
            None => previous = Some(level),
            Some(prev) => {
                let detected_report_type = match prev.cmp(level) {
                    cmp::Ordering::Less => ReportType::Increasing,
                    cmp::Ordering::Equal => report_type.clone(),
                    cmp::Ordering::Greater => ReportType::Decreasing,
                };

                match (&report_type, &detected_report_type) {
                    (ReportType::Unknown, _) => report_type = detected_report_type,
                    (ReportType::Increasing, ReportType::Increasing) => {}
                    (ReportType::Decreasing, ReportType::Decreasing) => {}
                    _ => return false,
                }

                let difference = prev.abs_diff(*level);

                if difference < 1 || difference > 3 {
                    return false;
                }

                previous = Some(level);
            }
        }
    }

    true
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE_INPUT: &str = r#"
        7 6 4 2 1
        1 2 7 8 9
        9 7 6 2 1
        1 3 2 4 5
        8 6 4 4 1
        1 3 6 7 9
    "#;

    #[test]
    fn part_one_works_on_the_example_input() {
        let reports = parse_reports(EXAMPLE_INPUT);
        let valid_reports = part_one(&reports);
        assert_eq!(valid_reports, 2);
    }
}
