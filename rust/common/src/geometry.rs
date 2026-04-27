use std::ops::{Add, AddAssign, Mul, Sub, SubAssign};

/// A 2D point in a grid.
///
/// Puzzles often involve 2D grids. This struct provides standard coordinate math
/// and distance calculations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    /// Creates a new point.
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    /// Calculates the Manhattan distance between two points.
    ///
    /// # Example
    /// ```
    /// use aoc_rust_common::Point;
    /// let p1 = Point::new(0, 0);
    /// let p2 = Point::new(3, 4);
    /// assert_eq!(p1.manhattan_distance(&p2), 7);
    /// ```
    pub fn manhattan_distance(&self, other: &Self) -> u32 {
        ((self.x - other.x).abs() + (self.y - other.y).abs()) as u32
    }

    /// Applies Euclidean remainder to both coordinates. Useful for wrapping around a grid.
    ///
    /// # Example
    /// ```
    /// use aoc_rust_common::Point;
    /// let p = Point::new(-1, 10);
    /// assert_eq!(p.rem_euclid(7, 7), Point::new(6, 3));
    /// ```
    pub fn rem_euclid(&self, width: i32, height: i32) -> Self {
        Self {
            x: self.x.rem_euclid(width),
            y: self.y.rem_euclid(height),
        }
    }

    /// Returns the 4 cardinal neighbors (North, South, East, West).
    pub fn cardinal_neighbors(&self) -> [Self; 4] {
        [
            *self + Direction::North.delta(),
            *self + Direction::South.delta(),
            *self + Direction::East.delta(),
            *self + Direction::West.delta(),
        ]
    }
}

impl Add for Point {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl Sub for Point {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl SubAssign for Point {
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
    }
}

impl Mul<i32> for Point {
    type Output = Self;
    fn mul(self, rhs: i32) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

/// Standard 8-way directions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    North,
    South,
    East,
    West,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
}

impl Direction {
    /// Returns the coordinate delta for this direction.
    /// Assumes y-increases downwards (standard for most grid puzzles).
    pub fn delta(&self) -> Point {
        match self {
            Direction::North => Point::new(0, -1),
            Direction::South => Point::new(0, 1),
            Direction::East => Point::new(1, 0),
            Direction::West => Point::new(-1, 0),
            Direction::NorthEast => Point::new(1, -1),
            Direction::NorthWest => Point::new(-1, -1),
            Direction::SouthEast => Point::new(1, 1),
            Direction::SouthWest => Point::new(-1, 1),
        }
    }

    /// Returns the opposite direction.
    pub fn opposite(&self) -> Self {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
            Direction::West => Direction::East,
            Direction::NorthEast => Direction::SouthWest,
            Direction::NorthWest => Direction::SouthEast,
            Direction::SouthEast => Direction::NorthWest,
            Direction::SouthWest => Direction::NorthEast,
        }
    }
}
