use std::collections::{HashMap, HashSet};

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
        .antenna_types()
        .iter()
        .fold(HashSet::new(), |mut acc, antenna_type| {
            let positions = puzzle.all_coords_for_antenna(*antenna_type);

            iterate_pairs(&positions)
                .flat_map(|pair| antinodes_for_pair(puzzle, pair))
                .for_each(|pos| {
                    acc.insert(pos);
                });

            acc
        })
        .len()
}

fn part_two(puzzle: &Puzzle) -> usize {
    puzzle
        .antenna_types()
        .iter()
        .fold(HashSet::new(), |mut acc, antenna_type| {
            let positions = puzzle.all_coords_for_antenna(*antenna_type);

            iterate_pairs(&positions)
                .flat_map(|pair| antinodes_for_pair_with_resonant_freqs(puzzle, pair))
                .for_each(|pos| {
                    acc.insert(pos);
                });

            acc
        })
        .len()
}

/// Create an iterator that yields all possible pairs of items in a Vec/array.
///
/// # Example
///
/// ```
/// let positions = vec![(0, 0), (1, 1), (2, 2)];
/// let mut pairs = iterate_pairs(&positions);
/// ```
///
/// The iterator will yield the following pairs:
/// - (0, 0), (1, 1)
/// - (0, 0), (2, 2)
/// - (1, 1), (2, 2)
/// ```
fn iterate_pairs<'a, T>(positions: &'a [T]) -> impl Iterator<Item = (&'a T, &'a T)> + 'a {
    positions.iter().enumerate().flat_map(|(i, position1)| {
        positions
            .iter()
            .skip(i + 1)
            .map(move |position2| (position1, position2))
    })
}

/// Find all positions that are exactly twice the distance from one point as they are from the
/// other point. Antinodes cannot exist outside the dimensions of the Puzzle.
///
/// # Example
///
/// `#` represents the antinodes for antenna `a`.
///
/// ..........
/// ...#......
/// #.........
/// ....a.....
/// ........a.
/// .....a....
/// ..#.......
/// ......#...
/// ..........
/// ..........
fn antinodes_for_pair(puzzle: &Puzzle, pair: (&XY, &XY)) -> Vec<XY> {
    let ((x1, y1), (x2, y2)) = pair;

    let dx = *x2 as isize - *x1 as isize;
    let dy = *y2 as isize - *y1 as isize;

    let mut antinodes = Vec::new();

    let x3 = *x1 as isize - dx;
    let y3 = *y1 as isize - dy;

    if x3 >= 0 && y3 >= 0 && x3 < puzzle.dimensions.0 as isize && y3 < puzzle.dimensions.1 as isize
    {
        antinodes.push((x3 as usize, y3 as usize));
    }

    let x4 = *x2 as isize + dx;
    let y4 = *y2 as isize + dy;

    if x4 >= 0 && y4 >= 0 && x4 < puzzle.dimensions.0 as isize && y4 < puzzle.dimensions.1 as isize
    {
        antinodes.push((x4 as usize, y4 as usize));
    }

    antinodes
}

/// Find all positions that are an interval of the distance between two points.
///
/// # Example
///
/// `#` represents the antinodes for antenna `T`.
///
/// T....#....
/// ...T......
/// .T....#...
/// .........#
/// ..#.......
/// ..........
/// ...#......
/// ..........
/// ....#.....
/// ..........
fn antinodes_for_pair_with_resonant_freqs(puzzle: &Puzzle, pair: (&XY, &XY)) -> Vec<XY> {
    let ((x1, y1), (x2, y2)) = pair;

    let dx = *x2 as isize - *x1 as isize;
    let dy = *y2 as isize - *y1 as isize;

    // Antinodes include the antennae themselves
    let mut antinodes = Vec::from([(*x1, *y1), (*x2, *y2)]);

    let mut x3 = *x1 as isize;
    let mut y3 = *y1 as isize;

    // Moving away from point 1
    loop {
        x3 -= dx;
        y3 -= dy;

        if x3 < 0
            || y3 < 0
            || x3 >= puzzle.dimensions.0 as isize
            || y3 >= puzzle.dimensions.1 as isize
        {
            break;
        }

        antinodes.push((x3 as usize, y3 as usize));
    }

    // Moving away from point 2
    x3 = *x2 as isize;
    y3 = *y2 as isize;

    loop {
        x3 += dx;
        y3 += dy;

        if x3 < 0
            || y3 < 0
            || x3 >= puzzle.dimensions.0 as isize
            || y3 >= puzzle.dimensions.1 as isize
        {
            break;
        }

        antinodes.push((x3 as usize, y3 as usize));
    }

    antinodes
}

type XY = (usize, usize);

#[derive(Debug)]
struct Puzzle {
    dimensions: XY,

    // A map of (x, y) to the antenna type (represented by a char)
    data: HashMap<XY, char>,
}

impl Puzzle {
    // Get the unique antenna types in the puzzle
    fn antenna_types(&self) -> HashSet<char> {
        self.data.values().copied().collect()
    }

    fn all_coords_for_antenna(&self, antenna: char) -> Vec<XY> {
        self.data
            .iter()
            .filter_map(|(pos, c)| if *c == antenna { Some(*pos) } else { None })
            .collect()
    }
}

impl From<&str> for Puzzle {
    fn from(input: &str) -> Puzzle {
        let lines = input.trim().lines().map(str::trim);

        let width = lines.clone().map(str::len).max().unwrap();
        let height = lines.clone().count();

        let data = lines
            .enumerate()
            .flat_map(|(y, line)| line.chars().enumerate().map(move |(x, c)| ((x, y), c)))
            .filter(|(_, c)| *c != '.')
            .fold(HashMap::new(), |mut acc, (pos, c)| {
                acc.insert(pos, c);
                acc
            });

        Puzzle {
            dimensions: (width, height),
            data,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"
        ............
        ........0...
        .....0......
        .......0....
        ....0.......
        ......A.....
        ............
        ............
        ........A...
        .........A..
        ............
        ............
    "#;

    #[test]
    fn test_part_one() {
        let puzzle = Puzzle::from(INPUT);
        let result = part_one(&puzzle);

        assert_eq!(result, 14);
    }

    #[test]
    fn test_part_two() {
        let puzzle = Puzzle::from(INPUT);
        let result = part_two(&puzzle);

        assert_eq!(result, 34);
    }
}
