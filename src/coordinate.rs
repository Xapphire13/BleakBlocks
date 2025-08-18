use std::ops::{Add, Sub};

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct Coordinate {
    pub row: u32,
    pub col: u32,
}

pub fn coordinate(row: u32, col: u32) -> Coordinate {
    Coordinate { row, col }
}

impl Add for Coordinate {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        coordinate(self.row + rhs.row, self.col + rhs.col)
    }
}

impl Sub for Coordinate {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        coordinate(self.row - rhs.row, self.col - rhs.col)
    }
}

impl Coordinate {
    pub fn get_neighbors(&self, row_bound: u32, col_bound: u32) -> Vec<Coordinate> {
        let &Coordinate { row, col } = self;
        let mut neighbors = vec![];

        // Up
        if row > 0 {
            neighbors.push(coordinate(row - 1, col));
        }

        // Down
        if row < row_bound - 1 {
            neighbors.push(coordinate(row + 1, col));
        }

        // Left
        if col > 0 {
            neighbors.push(coordinate(row, col - 1));
        }

        // Right
        if col < col_bound - 1 {
            neighbors.push(coordinate(row, col + 1));
        }

        neighbors
    }
}
