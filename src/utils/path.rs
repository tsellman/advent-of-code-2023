use crate::utils::point::{Bounds, Point};

/// A 2D path of Points
#[derive(Debug, Clone)]
pub struct Path {
    points: Vec<Point>,
}

impl Path {
    pub fn new(points: Vec<Point>) -> Path {
        Path { points }
    }

    /// Determine the outermost boundaries of any points on this path
    pub fn bounds(&self) -> Bounds {
        let mut min = Point::new(0, 0);
        let mut max = Point::new(0, 0);

        for p in &self.points {
            if p.y < min.y { min.y = p.y; }
            if p.y > max.y { max.y = p.y; }
            if p.x < min.x { min.x = p.x; }
            if p.x > max.x { max.x = p.x; }
        }

        Bounds { min, max }
    }

    pub fn points(&self) -> &[Point] {
        &self.points
    }

    /// The length covered by traversing from point-to-point along this path
    pub fn len(&self) -> i64 {
        self.points.windows(2)
            .map(|p| (p[1].x - p[0].x).abs() + (p[1].y - p[0].y).abs())
            .sum()
    }
}

