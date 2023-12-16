use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::{Debug, Formatter};
use std::vec;

use Direction::*;

use crate::solutions::Harness;
use crate::utils::grid::Grid;
use crate::utils::point::{Direction, Point};

pub struct Day16 {}

impl Harness for Day16 {
    fn part_1(&self, input: &str, visualise: bool) -> i64 {
        let contraption = parse_contraption(input);
        let beam = Beam::new(Point::new(0, 0), East);
        energise(&contraption, &beam, visualise)
    }

    fn part_2(&self, input: &str, visualise: bool) -> i64 {
        let contraption = parse_contraption(input);

        let mut max = 0;
        for p in contraption.grid.points() {
            for d in contraption.start_options(&p) {
                let e = energise(&contraption, &Beam::new(p, d), visualise);
                if e > max { max = e; }
            }
        }
        max
    }
}

// ----------------

fn energise(contraption: &Contraption, beam: &Beam, visualise: bool) -> i64 {
    // track which cells are energised
    let mut energy: HashMap<Point, i64> = HashMap::new();
    // make sure we don't get stuck in loops
    let mut processed = HashSet::new();

    // all the beams currently in play
    let mut beams = VecDeque::new();
    // (initial beam)
    beams.push_front(beam.clone());

    while let Some(beam) = beams.pop_front() {
        if processed.contains(&beam) { continue; }

        energy.entry(beam.pos)
            .and_modify(|e| *e += 1)
            .or_insert(0);

        beams.extend(contraption.next(&beam));
        processed.insert(beam);
    }

    if visualise { _visualise(&contraption, &energy); }

    energy.len() as i64
}

// -------------------------------------------------------------------------------------------------
// model

#[derive(Hash, Eq, PartialEq, Clone)]
struct Beam {
    pos: Point,
    dir: Direction,
}

impl Beam {
    fn new(pos: Point, dir: Direction) -> Beam {
        Beam { pos, dir }
    }
}

impl Debug for Beam {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}){}", self.pos.x, self.pos.y, match self.dir {
            North => '^',
            South => 'V',
            West => '<',
            East => '>'
        })
    }
}

struct Contraption {
    grid: Grid<char>,
}

impl Contraption {
    /// Calculate the resulting beams after the input beam takes one step
    fn next(&self, beam: &Beam) -> Vec<Beam> {
        let v = self.grid.get(&beam.pos);
        if v.is_none() { return vec![]; }

        let dirs = match v.unwrap() {
            // continue
            '.' => vec![beam.dir],
            // split
            '|' => match beam.dir {
                North | South => vec![beam.dir],
                East | West => vec![North, South]
            },
            '-' => match beam.dir {
                North | South => vec![West, East],
                East | West => vec![beam.dir],
            },
            // bounce
            '/' => match beam.dir {
                North | South => vec![beam.dir.rotate_clockwise()],
                East | West => vec![beam.dir.rotate_anticlockwise()],
            },
            '\\' => match beam.dir {
                North | South => vec![beam.dir.rotate_anticlockwise()],
                East | West => vec![beam.dir.rotate_clockwise()],
            }
            _ => vec![]
        };

        dirs.into_iter()
            .filter_map(|d| {
                self.grid.travel(&beam.pos, d).map(|p| Beam::new(p, d))
            })
            .collect()
    }

    fn start_options(&self, p: &Point) -> Vec<Direction> {
        let mut result = Vec::new();
        if p.x == 0 { result.push(East); }
        if p.y == 0 { result.push(South); }
        if p.x == self.grid.width() - 1 { result.push(West); }
        if p.y == self.grid.height() - 1 { result.push(North); }
        result
    }
}

// -------------------------------------------------------------------------------------------------
// parsing

fn parse_contraption(input: &str) -> Contraption {
    let cells: Vec<Vec<char>> = input.lines()
        .map(|line| line.chars().collect())
        .collect();

    Contraption { grid: Grid::<char>::new(cells) }
}

// -------------------------------------------------------------------------------------------------
// visualisation

fn _visualise(contraption: &Contraption, energy: &HashMap<Point, i64>) {
    contraption.grid.visualise(|_, p| {
        if energy.contains_key(&p) { "#" } else { "." }
    });
}
