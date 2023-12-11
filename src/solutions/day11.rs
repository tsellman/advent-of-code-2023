use std::collections::BTreeSet;
use std::iter::zip;

use crate::solutions::Harness;

pub struct Day11 {}

impl Harness for Day11 {
    fn part_1(&self, input: &str, visualise: bool) -> i64 {
        // parse the data
        let raw = parse_universe(input);
        if visualise { _visualise(&raw); }
        // expand the universe
        let universe = expand_universe(&raw, 2);
        if visualise { _visualise(&universe); }
        // sum the paths between galaxies
        sum_paths(&universe.galaxies)
    }

    fn part_2(&self, input: &str, _visualise: bool) -> i64 {
        // parse the data
        let raw = parse_universe(input);
        // expand the universe
        let universe = expand_universe(&raw, 1000000);
        // sum the paths between galaxies
        sum_paths(&universe.galaxies)
    }
}

// ----------------

fn expand_universe(universe: &Universe, scale: i64) -> Universe {
    let (xs, width) = expand(universe.width, &universe.galaxies, |g| g.x, scale);
    let (ys, height) = expand(universe.height, &universe.galaxies, |g| g.y, scale);

    let galaxies = zip(xs, ys)
        .map(|(x, y)| Galaxy { x, y })
        .collect();

    Universe { height, width, galaxies }
}

fn expand<F: Fn(&Galaxy) -> i64>(max: i64, galaxies: &[Galaxy], dimension: F, scale: i64) -> (Vec<i64>, i64) {
    let points: Vec<i64> = galaxies.iter().map(|g| dimension(g)).collect();

    // figure out which lines are blank
    let mut blanks = BTreeSet::new();
    blanks.extend(0..max);
    for p in &points { blanks.remove(&p); }

    // expands all the points
    let factor = scale - 1;
    let expanded = points.iter()
        .map(|p| p + blanks.iter().filter(|&b| b < p).count() as i64 * factor)
        .collect();

    (expanded, max + blanks.len() as i64 * factor)
}

fn sum_paths(galaxies: &[Galaxy]) -> i64 {
    let mut sum = 0;
    for i in 0..galaxies.len() {
        for j in i + 1..galaxies.len() {
            sum += manhattan_distance(&galaxies[i], &galaxies[j]);
        }
    }
    sum
}

fn manhattan_distance(a: &Galaxy, b: &Galaxy) -> i64 {
    (b.y - a.y).abs() + (b.x - a.x).abs()
}

// -------------------------------------------------------------------------------------------------
// model

#[derive(Debug, Clone, Eq, PartialEq)]
struct Galaxy {
    x: i64,
    y: i64,
}

impl Galaxy {
    fn of(x: usize, y: usize) -> Galaxy {
        Galaxy { x: x as i64, y: y as i64 }
    }
}

struct Universe {
    width: i64,
    height: i64,
    galaxies: Vec<Galaxy>,
}

// -------------------------------------------------------------------------------------------------
// parsing

fn parse_universe(data: &str) -> Universe {
    let mut width = 0;
    let mut height = 0;
    let mut galaxies = Vec::new();
    for (y, line) in data.lines().enumerate() {
        width = line.len();
        height += 1;
        for (x, c) in line.chars().enumerate() {
            if c == '#' { galaxies.push(Galaxy::of(x, y)); }
        }
    }

    Universe { width: width as i64, height, galaxies }
}

// -------------------------------------------------------------------------------------------------
// visualisation

fn _visualise(universe: &Universe) {
    for y in 0..universe.height {
        for x in 0..universe.width {
            let p = Galaxy { x, y };
            if universe.galaxies.contains(&p) { print!("#"); } else { print!("."); }
        }
        println!();
    }
    println!();
}
