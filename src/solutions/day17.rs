use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};

use crate::solutions::Harness;
use crate::utils::grid::Grid;
use crate::utils::point::{Direction, Point};
use crate::utils::point::Direction::*;

pub struct Day17 {}

impl Harness for Day17 {
    fn part_1(&self, input: &str, _visualise: bool) -> i64 {
        let city = parse_city(input);

        // starting in top left
        let start = Point::new(0, 0);
        // aiming for bottom right
        let target = Point::new(city.width() - 1, city.height() - 1);

        search(&city, start, target, 0, 3)
    }

    fn part_2(&self, input: &str, _visualise: bool) -> i64 {
        let city = parse_city(input);

        let start = Point::new(0, 0);
        let target = Point::new(city.width() - 1, city.height() - 1);

        search(&city, start, target, 4, 10)
    }
}

// ----------------

/// Dijkstra search, modified to handle the constraints:
///     1. min and/or max steps per direction
///     2. can only go forward, left, or right from each position
fn search(city: &City, start: Point, target: Point, min_steps: i64, max_steps: i64) -> i64 {
    // track positions we've already seen
    let mut seen = HashSet::new();

    // our search queue, initialised with directions reachable from start
    let mut queue: BinaryHeap<State> = [North, East, South, West].into_iter()
        .filter_map(|direction| {
            if city.bounds().contains(&start.travel(direction)) {
                Some(State::new(0, Location::new(start, direction), 0))
            } else { None }
        })
        .collect();

    // search until queue exhausted
    while let Some(State { cost, location, steps }) = queue.pop() {
        // have we found the target?
        if location.point == target && steps >= min_steps {
            return cost;
        }

        // have we already seen this point with this number of steps?
        if !seen.insert((location.clone(), steps)) {
            continue;
        }

        // queue up the next search points
        // forward if max steps not exceeded
        if steps < max_steps {
            if let Some(next) = city.travel(&location.point, location.direction) {
                let cost = cost + city.get(&next).unwrap();
                queue.push(State::new(cost, Location::new(next, location.direction), steps + 1));
            }
        }
        // right and left if min steps reached
        if steps >= min_steps {
            for direction in [
                location.direction.rotate_clockwise(),
                location.direction.rotate_anticlockwise()
            ] {
                if let Some(next) = city.travel(&location.point, direction) {
                    let cost = cost + city.get(&next).unwrap();
                    queue.push(State::new(cost, Location::new(next, direction), 1));
                }
            }
        }
    }
    panic!("No solution found!")
}


// -------------------------------------------------------------------------------------------------
// model

#[derive(Debug, Clone, Hash, Eq, PartialEq, PartialOrd, Ord)]
struct Location {
    point: Point,
    direction: Direction,
}

impl Location {
    fn new(point: Point, direction: Direction) -> Location {
        Location { point, direction }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Ord)]
struct State {
    /// accumulated heat cost
    cost: i64,
    /// current location
    location: Location,
    /// number of steps taken in current direction
    steps: i64,
}

impl State {
    fn new(cost: i64, position: Location, steps: i64) -> State {
        State { cost, location: position, steps }
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(other.cost.cmp(&self.cost)) // lowest first ordering
    }
}

type City = Grid<i64>;

// -------------------------------------------------------------------------------------------------
// parsing

fn parse_city(input: &str) -> City {
    // Each city block is marked by a single digit that represents the amount
    // of heat loss if the crucible enters that block
    let blocks = input.lines()
        .map(|line| {
            line.chars().map(|b| b.to_digit(10).unwrap() as i64).collect()
        })
        .collect();

    City::new(blocks)
}