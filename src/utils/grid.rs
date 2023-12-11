// -------------------------------------------------------------------------------------------------
// Grid

use crate::utils::point::{Bounds, Direction, Point};

/// A 2D array-based grid, it starts at `(0,0)` and extends only in positive `x` and `y` directions
#[derive(Clone)]
pub struct Grid<T> {
    bounds: Bounds,
    cells: Vec<Vec<T>>,
}

impl<T> Grid<T> {
    /// Construct a new grid from the given rows
    pub fn new<A>(cells: Vec<Vec<A>>) -> Grid<A> {
        let height = cells.len() as i64;
        let width = cells[0].len() as i64;
        let bounds = Bounds { min: Point::new(0, 0), max: Point::new(width - 1, height - 1) };

        Grid { bounds, cells }
    }

    pub fn height(&self) -> i64 {
        // bounds are inclusive, so need to add 1
        self.bounds.max.y + 1
    }

    pub fn width(&self) -> i64 {
        // bounds are inclusive, so need to add 1
        self.bounds.max.x + 1
    }

    pub fn bounds(&self) -> &Bounds {
        &self.bounds
    }

    /// Get the value at the given point, if it's within the grid bounds
    pub fn get(&self, p: &Point) -> Option<&T> {
        self.cells.get(p.y as usize)
            .and_then(|row| row.get(p.x as usize))
    }

    /// Get an iterator over all the Points in the grid (row-wise)
    pub fn points(&self) -> ByRowIterator<T> {
        ByRowIterator { grid: self, current: None }
    }

    /// Iterating row-wise, find the first point where the given predicate is true
    pub fn find_first<P: Fn(&T) -> bool>(&self, predicate: P) -> Option<Point> {
        self.points()
            .filter(|p| self.get(p).filter(|v| predicate(*v)).is_some())
            .next()
    }

    /// Get the point which is one position to the given direction of the given point (if valid)
    pub fn travel(&self, p: &Point, direction: Direction) -> Option<Point> {
        p.travel_bounded(direction, self.bounds())
    }

    /// Get the point which is one position to the diagonal of the given point
    pub fn travel_diag(&self, p: &Point, d1: Direction, d2: Direction) -> Option<Point> {
        self.travel(p, d1)
            .and_then(|n| self.travel(&n, d2))
    }

    /// Get all points which are directly adjacent to the given point (doesn't include diagonals)
    pub fn adjacent(&self, p: &Point) -> Vec<Point> {
        p.adjacent_bounded(self.bounds())
    }

    /// Display the grid, using the given function to decide what to show for each cell's value
    pub fn visualise<F, S>(&self, to_str: F)
        where S: AsRef<str>, F: Fn(&T, &Point) -> S
    {
        for y in 0..self.height() {
            for x in 0..self.width() {
                let p = Point::new(x, y);
                if let Some(v) = self.get(&p) {
                    print!("{}", to_str(v, &p).as_ref());
                } else { print!(" "); }
            }
            println!();
        }
    }
}

// -------------------------------------------------------------------------------------------------

pub struct ByRowIterator<'a, T> {
    grid: &'a Grid<T>,
    current: Option<Point>,
}

impl<'a, T> Iterator for ByRowIterator<'a, T> {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        let mut next = Point::new(0, 0);
        if let Some(Point { x, y }) = self.current {
            next.x = x + 1;
            next.y = y;
            if next.x >= self.grid.width() {
                next.x = 0;
                next.y += 1;
            }
            if next.y >= self.grid.height() { return None; }
        }
        self.current = Some(next);
        self.current
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let size = (self.grid.height() * self.grid.width()) as usize;
        (size, Some(size))
    }
}

// -------------------------------------------------------------------------------------------------
// tests

#[cfg(test)]
mod tests {
    use crate::utils::point::*;
    use crate::utils::point::Direction::*;

    use super::*;

    #[test]
    fn grid_travel() {
        let grid = Grid::<i32>::new(vec![vec![0; 3]; 4]);

        let point = Point::new(2, 3);
        assert_eq!(Some(Point::new(1, 3)), grid.travel(&point, West));
        assert_eq!(None, grid.travel(&point, East));
        assert_eq!(Some(Point::new(2, 2)), grid.travel(&point, North));
        assert_eq!(None, grid.travel(&point, South));

        let point = Point::new(0, 0);
        assert_eq!(None, grid.travel(&point, North));
        assert_eq!(Some(Point::new(1, 0)), grid.travel(&point, East));
        assert_eq!(Some(Point::new(0, 1)), grid.travel(&point, South));
        assert_eq!(None, grid.travel(&point, West));
    }

    #[test]
    fn height_and_width() {
        let grid = Grid::<i32>::new(vec![vec![0; 3]; 4]);
        assert_eq!(3, grid.width(), "Width should be 3");
        assert_eq!(4, grid.height(), "Height should be 4");
    }

    #[test]
    fn iterate_by_row() {
        let grid = Grid::<i32>::new(vec![vec![0; 3]; 4]);

        let points = grid.points().collect::<Vec<_>>();
        assert_eq!(points, vec![
            Point::new(0, 0), Point::new(1, 0), Point::new(2, 0),
            Point::new(0, 1), Point::new(1, 1), Point::new(2, 1),
            Point::new(0, 2), Point::new(1, 2), Point::new(2, 2),
            Point::new(0, 3), Point::new(1, 3), Point::new(2, 3),
        ])
    }

    #[test]
    fn iterate_by_row_rev() {
        let grid = Grid::<i32>::new(vec![vec![0; 3]; 4]);

        let mut points = grid.points().collect::<Vec<_>>();
        points.reverse();
        assert_eq!(points, vec![
            Point::new(2, 3), Point::new(1, 3), Point::new(0, 3),
            Point::new(2, 2), Point::new(1, 2), Point::new(0, 2),
            Point::new(2, 1), Point::new(1, 1), Point::new(0, 1),
            Point::new(2, 0), Point::new(1, 0), Point::new(0, 0),
        ])
    }
}