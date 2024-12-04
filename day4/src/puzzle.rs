use std::ops;

pub struct Puzzle {
    data: Vec<Vec<char>>,
}

const SEARCH_STRING: &str = "XMAS";

const DIRECTIONS: [(i8, i8); 8] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];

impl Puzzle {
    pub fn new(data: Vec<Vec<char>>) -> Self {
        Self { data }
    }

    pub fn height(&self) -> usize {
        self.data.len()
    }

    pub fn width(&self) -> usize {
        if self.data.is_empty() {
            return 0;
        }

        self.data[0].len()
    }

    /// Create an iterator that will iterate over all
    /// valid coordinates in the Puzzle.
    pub fn iter_coords(&self) -> PuzzleCoordIterator {
        PuzzleCoordIterator {
            puzzle: self,
            next_y: 0,
            next_x: 0,
        }
    }

    /// Count the number of instances of the word "XMAS" that start at the
    /// given coordinates.
    pub fn count_xmas_instances_at(&self, coord: Coord) -> usize {
        let mut count = 0;

        for (dx, dy) in &DIRECTIONS {
            if self.has_xmas_in_direction(coord, *dx, *dy) {
                count += 1;
            }
        }

        count
    }

    /// Determine the word "XMAS" stats at the coords, moving in the direction
    /// defined by dx and dy.
    ///
    /// @param coord The starting coordinate
    /// @param dx The x direction to move in (1, 0, -1)
    /// @param dy The y direction to move in (1, 0, -1)
    fn has_xmas_in_direction(&self, coord: Coord, dx: i8, dy: i8) -> bool {
        let mut current_coord: Coord = (coord.0, coord.1);

        for search_char_idx in 0..SEARCH_STRING.len() {
            let expected_char = SEARCH_STRING
                .chars()
                .nth(search_char_idx)
                .expect("Empty search string");

            let found_char = self[current_coord];

            if found_char != expected_char {
                return false;
            }

            if (search_char_idx + 1) == SEARCH_STRING.len() {
                return true;
            }

            if (current_coord.0 as i16 + dx as i16) < 0
                || (current_coord.1 as i16 + dy as i16) < 0
                || current_coord.0 as i16 + dx as i16 >= self.width() as i16
                || current_coord.1 as i16 + dy as i16 >= self.height() as i16
            {
                return false;
            }

            current_coord = (
                (current_coord.0 as i16 + dx as i16) as usize,
                (current_coord.1 as i16 + dy as i16) as usize,
            );
        }

        false
    }

    /// Determine if there is an X formed by two instances of "MAS" at the
    /// given Coord. The Coord is defined by the center position of the 'A'.
    pub fn has_mas_cross_at(&self, coord: Coord) -> bool {
        if self[coord] != 'A'
            || coord.0 == 0
            || coord.1 == 0
            || coord.0 == self.width() - 1
            || coord.1 == self.height() - 1
        {
            return false;
        }

        let top_left_char = self[(coord.0 - 1, coord.1 - 1)];
        let top_right_char = self[(coord.0 + 1, coord.1 - 1)];
        let bottom_left_char = self[(coord.0 - 1, coord.1 + 1)];
        let bottom_right_char = self[(coord.0 + 1, coord.1 + 1)];

        let right_down: u8 = if top_left_char == 'M' && bottom_right_char == 'S' {
            1
        } else {
            0
        };

        let right_up: u8 = if bottom_left_char == 'M' && top_right_char == 'S' {
            1
        } else {
            0
        };

        let left_down: u8 = if top_right_char == 'M' && bottom_left_char == 'S' {
            1
        } else {
            0
        };

        let left_up: u8 = if bottom_right_char == 'M' && top_left_char == 'S' {
            1
        } else {
            0
        };

        right_down + right_up + left_down + left_up >= 2
    }
}

impl ops::Index<Coord> for Puzzle {
    type Output = char;

    fn index(&self, (x, y): Coord) -> &Self::Output {
        &self.data[y][x]
    }
}

/// Iterates over all valid coordinates in the Puzzle. It will iterate
/// over all x values in a row before moving to the next row.
pub struct PuzzleCoordIterator<'a> {
    puzzle: &'a Puzzle,
    next_x: usize,
    next_y: usize,
}

impl<'a> Iterator for PuzzleCoordIterator<'a> {
    type Item = Coord;

    fn next(&mut self) -> Option<Self::Item> {
        if self.next_y < self.puzzle.height() {
            let result = Some((self.next_x, self.next_y));

            self.next_x += 1;
            if self.next_x >= self.puzzle.width() {
                self.next_x = 0;
                self.next_y += 1;
            }

            result
        } else {
            None
        }
    }
}

pub type Coord = (usize, usize);
