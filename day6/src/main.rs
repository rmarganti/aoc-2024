use std::collections::{HashMap, HashSet};

fn main() {
    let input = shared::read_file_from_args();
    let state = State::from(input.as_str());

    let part_one = part_one(&state);
    println!("part one: {}", part_one);

    let part_two = part_two();
    println!("part one: {}", part_two);
}

fn part_one(state: &State) -> usize {
    let mut guard = state.guard.clone();
    // Keep a list of all the unique visited coordinates.
    let mut unique_points: HashSet<(isize, isize)> = [guard.position].into();

    loop {
        let next_guard_state = guard.step(&state.map);

        match next_guard_state {
            Some(next_guard_state) => {
                unique_points.insert(next_guard_state.position);
                guard = next_guard_state;
            }
            None => break,
        }
    }

    unique_points.len()
}

fn part_two() -> usize {
    todo!()
}

#[derive(Debug)]
struct State {
    map: Map,
    guard: Guard,
}

impl From<&str> for State {
    fn from(input: &str) -> Self {
        let mut obstacles = HashMap::new();
        let input = input.trim();

        let height = input.lines().count() as isize;
        let width = input
            .lines()
            .next()
            .map(|line| line.trim().chars().count() as isize)
            .expect("unexpected empty line");

        let mut direction = Direction::Down;
        let mut position: (isize, isize) = (0, 0);

        for (y, line) in input.lines().enumerate() {
            let line = line.trim();
            for (x, char) in line.chars().enumerate() {
                let x = x as isize;
                let y = y as isize;

                match char {
                    '^' => {
                        direction = Direction::Up;
                        position = (x, y);
                    }
                    'v' => {
                        direction = Direction::Down;
                        position = (x, y);
                    }
                    '<' => {
                        direction = Direction::Left;
                        position = (x, y);
                    }
                    '>' => {
                        direction = Direction::Right;
                        position = (x, y);
                    }
                    '#' => {
                        obstacles.entry(x).or_insert(HashMap::new()).insert(y, true);
                    }
                    _ => {}
                }
            }
        }

        State {
            guard: Guard {
                direction,
                position,
            },
            map: Map {
                width,
                height,
                obstacles,
            },
        }
    }
}

#[derive(Debug)]
struct Map {
    width: isize,
    height: isize,
    obstacles: HashMap<isize, HashMap<isize, bool>>,
}

impl Map {
    fn is_obstacle(&self, x: isize, y: isize) -> bool {
        match self.obstacles.get(&x) {
            Some(row) => row.contains_key(&y),
            None => false,
        }
    }
}

#[derive(Debug, Clone)]
struct Guard {
    direction: Direction,
    position: (isize, isize),
}

impl Guard {
    /// Create a new Guard instance after taking one step in the map.
    /// If the Guard hits an obstacle, it will change direction and try to move again.
    /// If the Guard exits the map, it will return `None`.
    fn step(&self, map: &Map) -> Option<Self> {
        let (x, y) = self.position;
        let (next_x, next_y) = match self.direction {
            Direction::Up => (x, y - 1),
            Direction::Down => (x, y + 1),
            Direction::Left => (x - 1, y),
            Direction::Right => (x + 1, y),
        };

        if next_x < 0 || next_x >= map.width || next_y < 0 || next_y >= map.height {
            return None;
        }

        if map.is_obstacle(next_x, next_y) {
            let new_direction = match self.direction {
                Direction::Up => Direction::Right,
                Direction::Right => Direction::Down,
                Direction::Down => Direction::Left,
                Direction::Left => Direction::Up,
            };

            let guard = Guard {
                position: (x, y),
                direction: new_direction,
            };

            return guard.step(map);
        }

        Some(Guard {
            position: (next_x, next_y),
            direction: self.direction.clone(),
        })
    }
}

#[derive(Debug, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[cfg(test)]

mod tests {
    use super::*;

    const INPUT: &str = r#"
        ....#.....
        .........#
        ..........
        ..#.......
        .......#..
        ..........
        .#..^.....
        ........#.
        #.........
        ......#...
    "#;

    #[test]
    fn test_part_one() {
        let state = State::from(INPUT);
        let result = part_one(&state);

        assert_eq!(result, 41);
    }

    #[test]
    fn test_part_two() {
        let result = part_two();

        assert_eq!(result, 6);
    }
}
