use enumflags2::{bitflags, make_bitflags, BitFlags};
use std::{
    iter::Sum,
    ops::{Add, AddAssign},
};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Coordinates {
    /// zero-based row number
    pub y: i16,

    /// zero-based column number
    pub x: i16,
}

impl Default for Coordinates {
    fn default() -> Self {
        Self::zero()
    }
}

impl Coordinates {
    pub const fn zero() -> Coordinates {
        Coordinates { x: 0, y: 0 }
    }

    pub const fn new(x: i16, y: i16) -> Coordinates {
        Coordinates { x, y }
    }

    pub const fn new_with_column_first(y: i16, x: i16) -> Coordinates {
        Coordinates { x, y }
    }
}

impl const Add for Coordinates {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        self.x = self.x + rhs.x;
        self.y = self.y + rhs.y;
        self
    }
}

impl AddAssign<Self> for Coordinates {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y
    }
}

impl<'a> const Add<&'a Self> for Coordinates {
    type Output = Self;

    fn add(mut self, rhs: &'a Self) -> Self::Output {
        self.x = self.x + rhs.x;
        self.y = self.y + rhs.y;
        self
    }
}

impl<'a> AddAssign<&'a Self> for Coordinates {
    fn add_assign(&mut self, rhs: &'a Self) {
        self.x += rhs.x;
        self.y += rhs.y
    }
}

impl Sum for Coordinates {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.reduce(|x, y| x + y).unwrap_or(Coordinates::new(0, 0))
    }
}

#[bitflags]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
/// A collection of directions on a square grid
pub enum Direction {
    /// Row: -1, Column: 0
    North = 0b0000_0001,

    /// Row: 1, Column: 0
    South = 0b0000_0010,

    /// Row: 0, Column: 1
    East = 0b0000_0100,

    /// Row: 0, Column: -1
    West = 0b0000_1000,

    /// Row: 1, Column: 1
    NorthEast = 0b0001_0000,

    /// Row: -1, Column: -1
    NorthWest = 0b0010_0000,

    /// Row: 1, Column: 1
    SouthEast = 0b0100_0000,

    /// Row: 1, Column: -1
    SouthWest = 0b1000_0000,
}

pub type DirectionSet = BitFlags<Direction, u8>;

impl Direction {
    pub const ORTHOGONAL: [Direction; 4] = [ Self::North, Self::South, Self::East, Self::West];
    pub const DIAGONAL: [Direction; 4] = [ Self::NorthEast, Self::SouthEast, Self::NorthWest, Self::SouthWest];
    
    /// Return all possible variants in an array (as opposed to a bit set, as `all()` does)
    pub const fn all_as_array() -> [Direction; 8] {
        [
            Self::North,
            Self::South,
            Self::East,
            Self::West,
            Self::NorthEast,
            Self::NorthWest,
            Self::SouthEast,
            Self::SouthWest,
        ]
    }

    /// Return this direction as a coordinate pair, pointing in the given direction.
    pub const fn as_coordinates(&self) -> Coordinates {
        match self {
            Self::North => Coordinates::new(0, -1),
            Self::South => Coordinates::new(0, 1),
            Self::East => Coordinates::new(1, 0),
            Self::West => Coordinates::new(-1, 0),
            Self::NorthEast => Coordinates::new(1, 1),
            Self::NorthWest => Coordinates::new(-1, -1),
            Self::SouthEast => Coordinates::new(1, 1),
            Self::SouthWest => Coordinates::new(1, -1),
        }
    }

    /// If this is a diagonal direction (NorthEast, NorthWest, SouthEast, SouthWest),
    /// split it into its two components (North | East, North | West, South | East, South | West).
    /// Otherwise return self
    pub const fn components(&self) -> DirectionSet {
        match self {
            Direction::North => make_bitflags!(Direction::{North}),
            Direction::South => make_bitflags!(Direction::{South}),
            Direction::East => make_bitflags!(Direction::{East}),
            Direction::West => make_bitflags!(Direction::{West}),
            Direction::NorthEast => make_bitflags!(Direction::{North | East}),
            Direction::NorthWest => make_bitflags!(Direction::{North | West}),
            Direction::SouthEast => make_bitflags!(Direction::{South | East}),
            Direction::SouthWest => make_bitflags!(Direction::{South | West}),
        }
    }
}
