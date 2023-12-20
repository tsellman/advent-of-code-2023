use std::collections::{HashSet, VecDeque};

use crate::solutions::Harness;

use crate::utils::point::{Point, Vector, Bounds};

use crate::utils::point::Direction::{self, *};
use crate::utils::path::Path;

pub struct Day18 {}

impl Harness for Day18 {
    fn part_1(&self, input: &str, _visualise: bool) -> i64 {
        // parse the dig plan
        let instructions = parse_instructions(input);
        // trace the path of the digger
        let path = {
            let mut points = Vec::new();
            let mut pos = Point::new(0, 0);
            for inst in instructions {
                pos = dig(pos, &mut points, &inst);
            }
            Path::new(points)
        };

        // calc the volume of the lagoon
        let volume = {
            // create a border around the path's extremities
            let bounds = path.bounds().expand(1);

            // flood fill anything outside the lagoon path
            let filled = flood_fill(bounds.min, &bounds, &path);
            // lagoon_volume = total_volume - exterior_volume
            bounds.area() - filled.len() as i64
        };

        volume
    }

    fn part_2(&self, input: &str, _visualise: bool) -> i64 {
        let instructions = parse_instructions_from_colours(input);

        // trace the dig path, including only corners
        let path = Path::new(instructions.iter()
            .fold(vec![Point::new(0, 0)], |mut acc, inst| {
                let prev = acc.last().unwrap();
                let next = prev.apply(Vector::of_distance(inst.direction, inst.distance));
                acc.push(next);
                acc
            }));

        // use shoelace formula to calculate the interior area of the lagoon
        let s = shoelace(&path.points());

        // Pick's theorum to add the lagoon boundary
        s + (path.len() / 2) + 1
    }
}

// ----------------

/// Dig a single instruction
fn dig(start: Point, path: &mut Vec<Point>, instruction: &Instruction) -> Point {
    let finish = start.apply(Vector::of_distance(instruction.direction, instruction.distance));

    match instruction.direction {
        North => for y in (finish.y..start.y).rev() {
            path.push(Point::new(start.x, y));
        }
        South => for y in start.y + 1..=finish.y {
            path.push(Point::new(start.x, y));
        }
        West => for x in (finish.x..start.x).rev() {
            path.push(Point::new(x, start.y));
        }
        East => for x in start.x + 1..=finish.x {
            path.push(Point::new(x, start.y));
        }
    }

    finish
}

/// BFS flood fill from a starting point, to the intersections with the given path
fn flood_fill(start: Point, bounds: &Bounds, path: &Path) -> HashSet<Point> {
    let path = path.points().iter().collect::<HashSet<_>>();

    let mut result = HashSet::new();
    result.insert(start);

    let mut queue = VecDeque::new();
    queue.push_back(start);

    let mut visited = HashSet::new();
    while let Some(p) = queue.pop_front() {
        if visited.contains(&p) { continue; }
        visited.insert(p);

        let mut adjacent = p.adjacent();
        adjacent.retain(|a| !path.contains(a) && bounds.contains(a));
        result.extend(adjacent.clone());
        queue.extend(adjacent);
    }
    result
}

/// Shoelace formula to calculate area of a polygon
fn shoelace(vertices: &[Point]) -> i64 {
    if vertices.len() < 2 { return 0; }

    fn lace(a: &Point, b: &Point) -> i64 { (b.x + a.x) * (b.y - a.y) }

    let mut sum = 0;
    for v in vertices.windows(2) {
        sum += lace(&v[0], &v[1]);
    }
    sum += lace(&vertices[vertices.len() - 1], &vertices[0]);

    sum.abs() / 2
}

// -------------------------------------------------------------------------------------------------
// model

#[derive(Debug)]
struct Instruction {
    direction: Direction,
    distance: i64,
}

// -------------------------------------------------------------------------------------------------
// parsing

fn parse_instructions(input: &str) -> Vec<Instruction> {
    input.lines()
        .map(|line| {
            let parts = line.split_whitespace().collect::<Vec<_>>();

            let direction = match parts[0] {
                "U" => North,
                "D" => South,
                "L" => West,
                "R" => East,
                _ => panic!()
            };

            let distance = parts[1].parse().unwrap();

            Instruction { direction, distance }
        })
        .collect()
}

fn parse_instructions_from_colours(input: &str) -> Vec<Instruction> {
    input.lines()
        .map(|line| {
            let parts = line.split_whitespace().collect::<Vec<_>>();
            let colour = &parts[2][2..=parts[2].len() - 2];

            let direction = match colour.chars().last().unwrap() {
                '0' => East,
                '1' => South,
                '2' => West,
                '3' => North,
                _ => panic!()
            };

            let distance = i64::from_str_radix(&colour[..=colour.len() - 2], 16).unwrap();

            Instruction { direction, distance }
        })
        .collect()
}

// -------------------------------------------------------------------------------------------------
// visualisation

fn _visualise(bounds: &(Point, Point), path: &[Point]) {
    let path = path.iter().collect::<HashSet<_>>();

    for y in bounds.0.y..=bounds.1.y {
        for x in bounds.0.x..=bounds.1.x {
            if path.contains(&Point::new(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
    println!();
}