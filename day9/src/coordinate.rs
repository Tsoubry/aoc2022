use std::cmp::Eq;
use std::hash::Hash;

use crate::Direction;

#[derive(Debug, new, Copy, Clone, PartialEq, Eq)]
pub struct Coordinate(isize, isize);

impl Coordinate {
    pub fn walk(&self, direction: &Direction) -> Self {
        match direction {
            Direction::D => Self::new(self.0, self.1 - 1),
            Direction::U => Self::new(self.0, self.1 + 1),
            Direction::L => Self::new(self.0 - 1 as isize, self.1),
            Direction::R => Self::new(self.0 + 1, self.1),
        }
    }

    pub fn walk_diagonally(&self, direction: &Direction, head: &Self) -> Self {
        match direction {
            Direction::D => Self::new(head.0, self.1 - 1),
            Direction::U => Self::new(head.0, self.1 + 1),
            Direction::L => Self::new(self.0 - 1 as isize, head.1),
            Direction::R => Self::new(self.0 + 1, head.1),
        }
    }

    pub fn walk_towards(&self, direction: &Direction) -> Self {
        match direction {
            Direction::D => Self::new(self.0 + 1, self.1 - 1),
            Direction::U => Self::new(self.0 + 1, self.1 + 1),
            Direction::L => Self::new(self.0 - 1 as isize, self.1 + 1), // right
            Direction::R => Self::new(self.0 + 1 as isize, self.1 - 1),
        }
    }

    pub fn check_coordinate_around(&self, other: &Self) -> bool {
        [self.0 - 1, self.0, self.0 + 1].contains(&other.0)
            && [self.1 - 1, self.1, self.1 + 1].contains(&other.1)
    }

    pub fn get_x(&self) -> isize {
        self.0
    }

    pub fn get_y(&self) -> isize {
        self.1
    }
}

impl Hash for Coordinate {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
        self.1.hash(state);
    }
}
