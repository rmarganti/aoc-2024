use std::collections::{HashMap, HashSet};

fn main() {
    let input = shared::read_file_from_args();
    let state = State::from(input.as_str());

    let part_one = part_one(&state);
    println!("part one: {}", part_one);

    let part_two = part_two(&state);
    println!("part one: {}", part_two);
}

/// How many unique positions does the Guard visit before exiting the map?
fn part_one(state: &State) -> usize {
    let result = state.map.execute(&state.guard);

    match result {
        MapResult::Exited { unique_visits } => unique_visits.len(),
        MapResult::Loop => panic!("unexpected loop in part_one"),
    }
}

/// How many positions can we place a new obstacle and get the Guard
/// stuck in a loop? We cannot put a new obstacle on the initial position
/// of the Guard, because he would notice.
fn part_two(state: &State) -> usize {
    let initial_positions = match state.map.execute(&state.guard) {
        MapResult::Exited { unique_visits } => unique_visits,
        MapResult::Loop => panic!("unexpected loop in part_two"),
    };

    let loop_count = initial_positions
        .iter()
        .skip(1)
        .filter(|position| {
            let new_map = state.map.with_obstacle(position.0, position.1);
            let result = new_map.execute(&state.guard);

            match result {
                MapResult::Exited { .. } => false,
                MapResult::Loop => true,
            }
        })
        .count();

    loop_count
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
    fn execute(&self, guard: &Guard) -> MapResult {
        let mut current_guard_state = guard.clone();
        let mut unique_visits = HashSet::<(isize, isize)>::from([current_guard_state.position]);
        let mut unique_guard_states = HashSet::<(isize, isize, Direction)>::from([(
            current_guard_state.position.0,
            current_guard_state.position.1,
            current_guard_state.direction.clone(),
        )]);

        loop {
            let next_guard_state = current_guard_state.step(self);

            match next_guard_state {
                Some(next_guard_state) => {
                    unique_visits.insert(next_guard_state.position);

                    if unique_guard_states.contains(&(
                        next_guard_state.position.0,
                        next_guard_state.position.1,
                        next_guard_state.direction.clone(),
                    )) {
                        return MapResult::Loop;
                    }

                    unique_guard_states.insert((
                        next_guard_state.position.0,
                        next_guard_state.position.1,
                        next_guard_state.direction.clone(),
                    ));

                    current_guard_state = next_guard_state;
                }
                None => break,
            }
        }

        MapResult::Exited { unique_visits }
    }

    fn is_obstacle(&self, x: isize, y: isize) -> bool {
        match self.obstacles.get(&x) {
            Some(row) => row.contains_key(&y),
            None => false,
        }
    }

    fn with_obstacle(&self, x: isize, y: isize) -> Self {
        let mut obstacles = self.obstacles.clone();
        obstacles.entry(x).or_insert(HashMap::new()).insert(y, true);
        Map {
            width: self.width,
            height: self.height,
            obstacles,
        }
    }
}

enum MapResult {
    Exited {
        unique_visits: HashSet<(isize, isize)>,
    },
    Loop,
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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
        let state = State::from(INPUT);
        let result = part_two(&state);

        assert_eq!(result, 6);
    }
}
