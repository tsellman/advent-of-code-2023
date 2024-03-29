// -------------------------------------------------------------------------------------------------
// point

use crate::utils::point::Direction::*;

/// A single point on a 2D grid
#[derive(Debug, Hash, Eq, PartialEq, Ord, PartialOrd, Copy, Clone)]
pub struct Point {
    pub x: i64,
    pub y: i64,
}

impl Point {
    pub fn new(x: i64, y: i64) -> Point {
        Point { x, y }
    }

    /// Get the point exactly one position in the given direction
    pub fn travel(&self, direction: Direction) -> Point {
        let shift = Vector::of(direction); // (what's our vector, Victor?)
        self.apply(shift)
    }

    /// Get the point in the given direction, if within the given points
    pub fn travel_bounded(&self, direction: Direction, bounds: &Bounds) -> Option<Point> {
        self.apply_bounded(Vector::of(direction), bounds)
    }

    /// Apply the given shift (expressed as a vector) to this point
    pub fn apply(&self, shift: Vector) -> Point {
        Point::new(self.x + shift.x, self.y + shift.y)
    }

    /// Apply the given shift to this point, but only return a new point if it would
    /// lie within the given bounds
    pub fn apply_bounded(&self, shift: Vector, bounds: &Bounds) -> Option<Point> {
        Some(self.apply(shift))
            .filter(|p| bounds.contains(p))
    }

    /// Get all points which are directly adjacent to the given point (doesn't include diagonals)
    pub fn adjacent(&self) -> Vec<Point> {
        [North, South, East, West].into_iter()
            .map(|d| self.travel(d))
            .collect()
    }

    pub fn adjacent_bounded(&self, bounds: &Bounds) -> Vec<Point> {
        [North, South, East, West].into_iter()
            .filter_map(|d| self.travel_bounded(d, bounds))
            .collect()
    }
}// -------------------------------------------------------------------------------------------------
// directions

/// A direction on a 2D grid
#[derive(Debug, Hash, Eq, PartialEq, Ord, PartialOrd, Copy, Clone)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    pub fn invert(&self) -> Direction {
        match self {
            North => South,
            East => West,
            South => North,
            West => East,
        }
    }

    pub fn rotate_clockwise(&self) -> Direction {
        match self {
            North => East,
            East => South,
            South => West,
            West => North,
        }
    }

    pub fn rotate_anticlockwise(&self) -> Direction {
        self.rotate_clockwise().invert()
    }
}

// -------------------------------------------------------------------------------------------------
// vector

/// A vector representing a change in 2D space
#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
pub struct Vector {
    pub x: i64,
    pub y: i64,
}

impl Vector {
    pub fn new(x: i64, y: i64) -> Vector {
        Vector { x, y }
    }

    pub fn of(direction: Direction) -> Vector {
        Vector::of_distance(direction, 1)
    }

    pub fn of_distance(direction: Direction, distance: i64) -> Vector {
        match direction {
            North => Vector::new(0, -1 * distance),
            East => Vector::new(distance, 0),
            South => Vector::new(0, distance),
            West => Vector::new(-1 * distance, 0)
        }
    }
}

// -------------------------------------------------------------------------------------------------
// bounds

/// Simple 2D rectangular grid boundaries
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Bounds {
    pub min: Point,
    pub max: Point,
}

impl Bounds {
    /// Contains, inclusive
    pub fn contains(&self, point: &Point) -> bool {
        point.x >= self.min.x && point.x <= self.max.x
            && point.y >= self.min.y && point.y <= self.max.y
    }

    pub fn expand(&self, amount: i64) -> Bounds {
        let min = Point::new(self.min.x - amount, self.min.y - amount);
        let max = Point::new(self.max.x + amount, self.max.y + amount);

        Bounds { min, max }
    }

    /// The area inside these bounds
    pub fn area(&self) -> i64 {
        (self.max.x + 1 - self.min.x) * (self.max.y + 1 - self.min.y)
    }
}

// -------------------------------------------------------------------------------------------------
// tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn point_travel() {
        let point = Point::new(3, 4);
        assert_eq!(Point::new(3, 3), point.travel(North));
        assert_eq!(Point::new(4, 4), point.travel(East));
        assert_eq!(Point::new(3, 5), point.travel(South));
        assert_eq!(Point::new(2, 4), point.travel(West));

        let point = Point::new(0, 0);
        assert_eq!(Point::new(0, -1), point.travel(North));
        assert_eq!(Point::new(1, 0), point.travel(East));
        assert_eq!(Point::new(0, 1), point.travel(South));
        assert_eq!(Point::new(-1, 0), point.travel(West));
    }
}
