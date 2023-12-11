use std::collections::{HashSet, VecDeque};

use Direction::*;

use crate::solutions::Harness;
use crate::utils::grid::{Grid};
use crate::utils::point::{Direction, Point};

pub struct Day10 {}

impl Harness for Day10 {
    fn part_1(&self, input: &str, _visualise: bool) -> i64 {
        let maze = parse_maze(input);
        let start = maze.grid.find_first(|v| v.value == 'S').unwrap();

        let path = follow_loop(&maze, &start);
        (path.len() / 2) as i64
    }

    fn part_2(&self, input: &str, visualise: bool) -> i64 {
        let maze = parse_maze(input);
        let start = maze.grid.find_first(|v| v.value == 'S').unwrap();
        let path = follow_loop(&maze, &start);

        let path_points = path.iter().map(|p| p.clone()).collect::<HashSet<_>>();

        let mut right = right_side_of(&path, &maze);
        right.retain(|p| !path_points.contains(p));
        flood(&mut right, &path_points, &maze);

        let bounds = outer_bounds(&path);
        let inner = if is_outer_region(&right, &bounds) {
            let mut left = maze.all_points();
            left.retain(|p| !path_points.contains(p) && !right.contains(p));
            left
        } else {
            right
        };

        if visualise { _visualise(&maze, &path_points, &inner); }

        inner.len() as i64
    }
}

// ----------------

/// Follow the loop within the maze, from the given start point
fn follow_loop(maze: &Maze, start: &Point) -> Vec<Point> {
    let mut path = Vec::new();
    path.push(start.clone());

    let mut skip = HashSet::new();
    skip.insert(start.clone());

    let mut pos = start;
    loop {
        let choices = maze.choices(pos);
        let next = choices.iter()
            .filter_map(|&c| maze.follow(pos, c))
            .filter(|p| !skip.contains(p))
            .next();

        match next {
            None => break,
            Some(n) => {
                skip.insert(n.clone());
                path.push(n);
                pos = path.get(path.len() - 1).unwrap();
            }
        };

        if pos == start { break; }
    }

    path
}

fn outer_bounds(path: &[Point]) -> (Point, Point) {
    let x_min = path.iter().map(|p| p.x).min().unwrap();
    let x_max = path.iter().map(|p| p.x).max().unwrap();
    let y_min = path.iter().map(|p| p.y).min().unwrap();
    let y_max = path.iter().map(|p| p.y).max().unwrap();

    (Point::new(x_min, y_min), Point::new(x_max, y_max))
}

fn is_outside(point: &Point, bounds: &(Point, Point)) -> bool {
    point.x <= bounds.0.x || point.x >= bounds.1.x
        || point.y <= bounds.0.y || point.y >= bounds.1.y
}

fn is_outer_region(region: &HashSet<Point>, bounds: &(Point, Point)) -> bool {
    for p in region { if is_outside(p, bounds) { return true; } }
    false
}

fn flood(region: &mut HashSet<Point>, path: &HashSet<Point>, maze: &Maze) {
    let mut visited: HashSet<Point> = HashSet::new();
    let mut queue = region.iter().map(|p| p.clone())
        .collect::<VecDeque<_>>();

    while let Some(point) = queue.pop_front() {
        // find adjacent points
        let mut adjacent = maze.grid.adjacent(&point);
        // remove any we already processed
        adjacent.retain(|p| !(region.contains(&p) || visited.contains(p) || path.contains(p)));

        for p in &adjacent { region.insert(p.clone()); }
        visited.insert(point);
        queue.extend(adjacent);
    }
}

/// Find all the points immediately to the right of the given path
fn right_side_of(path: &Vec<Point>, maze: &Maze) -> HashSet<Point> {
    let mut right = HashSet::new();
    let mut push = |point: Option<Point>| {
        if point.is_some_and(|p| !path.contains(&p)) {
            right.insert(point.unwrap());
        }
    };

    for points in path.windows(2) {
        let prev = &points[0];
        let point = &points[1];

        let dir = calc_dir(point, prev);
        let dirs = match (maze.value(point), dir) {
            ('|', North | South) => vec![dir.rotate_clockwise()],
            ('─', East | West) => vec![dir.rotate_clockwise()],
            ('┐', North) => vec![North, East],
            ('┐', East) => vec![South, West],
            ('┘', South) => vec![North, West],
            ('┘', East) => vec![South, East],
            ('┌', North) => vec![South, East],
            ('┌', West) => vec![North, West],
            ('└', South) => vec![South, West],
            ('└', East) => vec![North, East],
            _ => vec![]
        };

        match dirs[..] {
            [d] => { push(maze.grid.travel(point, d)); }
            [d1, d2] => {
                push(maze.grid.travel(point, d1));
                push(maze.grid.travel(point, d2));
                push(maze.grid.travel_diag(point, d1, d2));
            }
            _ => {}
        }
    }

    right
}

fn calc_dir(point: &Point, prev: &Point) -> Direction {
    if point.x < prev.x { West } else if point.x > prev.x { East } else if point.y < prev.y { North } else { South }
}

// -------------------------------------------------------------------------------------------------
// model

struct MazePoint {
    value: char,
    adjacent: Vec<Direction>,
}

struct Maze {
    grid: Grid<MazePoint>,
}

impl Maze {
    fn new(adjacency: Vec<Vec<MazePoint>>) -> Maze {
        Maze { grid: Grid::<MazePoint>::new(adjacency) }
    }

    fn follow(&self, p: &Point, direction: Direction) -> Option<Point> {
        let moved = self.grid.travel(p, direction);
        moved.filter(|n| self.choices(n).contains(&direction.invert()))
    }

    fn choices(&self, at: &Point) -> &Vec<Direction> {
        self.grid.get(at)
            .map(|p| &p.adjacent)
            .unwrap()
    }

    fn value(&self, at: &Point) -> char {
        self.grid.get(at)
            .map(|p| p.value)
            .unwrap()
    }

    fn all_points(&self) -> HashSet<Point> {
        self.grid.points().collect()
    }
}

// -------------------------------------------------------------------------------------------------
// parsing

fn parse_maze(input: &str) -> Maze {
    let adjacent = input.lines()
        .map(|line| parse_line(line))
        .collect();

    Maze::new(adjacent)
}

fn parse_line(line: &str) -> Vec<MazePoint> {
    let mut result = Vec::new();

    for c in line.chars() {
        let (adjacent, value) = match c {
            '.' => (vec![], c),
            '|' => (vec![North, South], '│'),
            '-' => (vec![East, West], '─'),
            'L' => (vec![North, East], '└'),
            'J' => (vec![North, West], '┘'),
            '7' => (vec![South, West], '┐'),
            'F' => (vec![East, South], '┌'),
            'S' => (vec![North, East, South, West], 'S'),
            _ => panic!("Unknown symbol: {}", c)
        };
        result.push(MazePoint { adjacent, value });
    }

    result
}

// -------------------------------------------------------------------------------------------------
// visualisation

fn _visualise(maze: &Maze, path: &HashSet<Point>, inside: &HashSet<Point>) {
    maze.grid.visualise(|v, p| {
        if path.contains(p) { v.value.to_string() } else if inside.contains(p) { "@".to_owned() } else { ".".to_owned() }
    });
    println!();
}