use std::cmp::Eq;
use std::fmt;
use std::hash::Hash;

use crate::Direction;

#[derive(Debug, new, Copy, Clone, PartialEq, Eq)]
pub struct Coordinate(isize, isize);

impl fmt::Display for Coordinate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

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
            Direction::L => Self::new(self.0 - 1, head.1),
            Direction::R => Self::new(self.0 + 1, head.1),
        }
    }

    pub fn walk_towards(&self, head: &Self) -> Self {
        let new_x = if head.0 > self.0 {
            self.0 + 1
        } else if head.0 < self.0 {
            self.0 - 1
        } else {
            self.0
        };
        let new_y = if head.1 > self.1 {
            self.1 + 1
        } else if head.1 < self.1 {
            self.1 - 1
        } else {
            self.1
        };

        Self(new_x, new_y)
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

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_walk_towards() {
        let c1 = Coordinate::new(-2, 7);
        let expected = Coordinate::new(-2, 6);
        assert_eq!(expected, c1.walk_towards(&Coordinate::new(-2, 5)));
    }
}
