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
        self.data[0].len()
    }

    pub fn iter_coords(&self) -> PuzzleIterator {
        PuzzleIterator {
            puzzle: self,
            row: 0,
            col: 0,
        }
    }

    pub fn count_xmas_instances_at(&self, x: usize, y: usize) -> usize {
        let mut count = 0;

        for (dx, dy) in &DIRECTIONS {
            if self.has_xmas_in_direction(x, y, *dx, *dy) {
                count += 1;
            }
        }

        count
    }

    fn has_xmas_in_direction(&self, x: usize, y: usize, dx: i8, dy: i8) -> bool {
        let mut current_coord = (x, y);

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
}

impl ops::Index<(usize, usize)> for Puzzle {
    type Output = char;

    fn index(&self, (row, col): (usize, usize)) -> &Self::Output {
        &self.data[col][row]
    }
}

pub struct PuzzleIterator<'a> {
    puzzle: &'a Puzzle,
    row: usize,
    col: usize,
}

impl<'a> Iterator for PuzzleIterator<'a> {
    type Item = Coord;

    fn next(&mut self) -> Option<Self::Item> {
        if self.row < self.puzzle.height() {
            let result = Some((self.col, self.row));

            self.col += 1;
            if self.col >= self.puzzle.width() {
                self.col = 0;
                self.row += 1;
            }

            result
        } else {
            None
        }
    }
}

pub type Coord = (usize, usize);
