use std::collections::HashSet;

use crate::solutions::Harness;
use crate::utils::grid::Grid;
use crate::utils::point::Point;

pub struct Day21 {}

impl Harness for Day21 {
    fn part_1(&self, input: &str, visualise: bool) -> i64 {
        // read grid and find start point
        let garden = parse_grid(input);
        let start = garden.find_first(|v| *v == 'S').unwrap();

        // calculate reachable positions after n steps
        let mut reachable = HashSet::new();
        reachable.insert(start);

        for i in 1..=64 {
            if visualise { println!("{}", i); }
            reachable = valid_steps_from(&reachable, &garden);
            if visualise { _visualise(&garden, &reachable); }
        }

        reachable.len() as i64
    }

    fn part_2(&self, _input: &str, _visualise: bool) -> i64 {
        // not implemented
        0
    }
}

// -------------------------------------------------------------------------------------------------

fn is_rock(garden: &Garden, p: &Point) -> bool {
    garden.get(p).is_some_and(|v| *v == '#')
}

fn valid_steps_from(points: &HashSet<Point>, garden: &Garden) -> HashSet<Point> {
    points.iter()
        .flat_map(|p| garden.adjacent(p).into_iter()
            .filter(|a| !is_rock(garden, &a))
            .collect::<Vec<_>>())
        .collect()
}

// -------------------------------------------------------------------------------------------------
// data model

type Garden = Grid<char>;

// -------------------------------------------------------------------------------------------------
// parsing

fn parse_grid(input: &str) -> Garden {
    let grid = input.lines()
        .map(|line| line.chars().collect())
        .collect();

    Garden::new(grid)
}

// -------------------------------------------------------------------------------------------------
// visualisation

fn _visualise(garden: &Garden, reachable: &HashSet<Point>) {
    garden.visualise(|_, p| if reachable.contains(p) {
        "O"
    } else if is_rock(garden, p) {
        "#"
    } else {
        "."
    });
    println!();
}