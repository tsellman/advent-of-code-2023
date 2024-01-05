use std::collections::HashMap;

use crate::solutions::Harness;
use crate::utils::grid::Grid;
use crate::utils::point::{Direction, Point};
use crate::utils::point::Direction::*;

pub struct Day14 {}

impl Harness for Day14 {
    fn part_1(&self, input: &str, visualise: bool) -> i64 {
        let mut dish = parse_dish(input);
        if visualise { _visualise(&dish); }

        tilt_north(&mut dish);
        if visualise { _visualise(&dish); }

        calc_load(&dish)
    }

    fn part_2(&self, input: &str, visualise: bool) -> i64 {
        let mut dish = parse_dish(input);
        if visualise { _visualise(&dish); }

        // track seen rock positions until we find a repeating pattern
        let mut seen = HashMap::new();

        // start cycling
        let target = 1000000000;
        let mut count = 0;
        while count < target {
            // tilt the dish to complete 1 cycle
            tilt_north(&mut dish);
            tilt_west(&mut dish);
            tilt_south(&mut dish);
            tilt_east(&mut dish);
            count += 1;

            if visualise {
                println!("Cycle {}:", count);
                _visualise(&dish);
            }

            // grab the position of the rocks after this cycle
            let rocks = dish.find_all(is_rock);
            // see if it matches a previous position, and skip over that many cycles
            if let Some(prev) = seen.get(&rocks) {
                let remaining = target - count;
                let repeat_len = count - prev;
                count = target - (remaining % repeat_len);
            } else {
                seen.insert(rocks, count);
            }
        }

        calc_load(&dish)
    }
}

// ----------------

fn calc_load(dish: &Dish) -> i64 {
    dish.find_all(is_rock).iter()
        .map(|p| (dish.height() - p.y) as i64)
        .sum()
}

/// Tilt the grid north, so all the loose rocks roll
fn tilt_north(dish: &mut Dish) {
    let rocks = dish.find_all(is_rock);
    _roll_rocks(rocks, North, dish);
}

fn tilt_west(dish: &mut Dish) {
    let rocks = dish.find_all_columnwise(is_rock);
    _roll_rocks(rocks, West, dish);
}

fn tilt_south(dish: &mut Dish) {
    let mut rocks = dish.find_all(is_rock);
    rocks.reverse();
    _roll_rocks(rocks, South, dish);
}

fn tilt_east(dish: &mut Dish) {
    let mut rocks = dish.find_all_columnwise(is_rock);
    rocks.reverse();
    _roll_rocks(rocks, East, dish);
}

fn _roll_rocks(rocks: Vec<Point>, direction: Direction, dish: &mut Dish) {
    for rock in rocks {
        let rolled = roll(&rock, &dish, direction);
        dish.set(&rock, EMPTY);
        dish.set(&rolled, ROCK);
    }
}

/// Work out how far the given rock can roll in the given direction
fn roll(p: &Point, dish: &Dish, direction: Direction) -> Point {
    if !dish.get(p).is_some_and(|v| *v == ROCK) { return p.clone(); }

    let mut moved = p.clone();
    loop {
        if let Some(n) = dish.travel(&moved, direction) {
            if dish.get(&n).is_some_and(|v| *v == EMPTY) {
                moved = n;
            } else { break; }
        } else { break; }
    }
    moved
}

// -------------------------------------------------------------------------------------------------
// model
type Dish = Grid<char>;

const EMPTY: char = '.';
const ROCK: char = 'O';

fn is_rock(value: &char) -> bool {
    *value == ROCK
}

// -------------------------------------------------------------------------------------------------
// parsing

fn parse_dish(input: &str) -> Dish {
    let cells: Vec<Vec<char>> = input.lines()
        .map(|line| line.chars().collect())
        .collect();

    Dish::new(cells)
}

// -------------------------------------------------------------------------------------------------
// visualisation

fn _visualise(dish: &Dish) {
    dish.visualise(|v, _| v.to_string());
    println!();
}