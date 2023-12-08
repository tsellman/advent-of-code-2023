use std::collections::HashMap;

use crate::solutions::Harness;

pub struct Day8 {}

impl Harness for Day8 {
    fn part_1(&self, input: &str, _visualise: bool) -> i64 {
        let navigation = parse_file(input);
        let steps = follow_navigation(&navigation, "AAA", |n| n == "ZZZ");
        steps
    }

    fn part_2(&self, input: &str, _visualise: bool) -> i64 {
        let navigation = parse_file(input);

        let steps_for_starts = navigation.maze.keys().cloned()
            .filter(|k| k.ends_with("A"))
            .map(|s| follow_navigation(&navigation, &s, |n| n.ends_with("Z")))
            .collect::<Vec<_>>();

        lowest_common_multiple(&steps_for_starts)
    }
}

// ----------------

fn follow_navigation<F>(navigation: &Navigation, start: &str, finish: F) -> i64
    where F: Fn(&str) -> bool
{
    let mut pos = start;
    let mut step = 0;

    while !finish(pos) {
        let options = &navigation.maze.get(pos).unwrap();
        pos = next_step(step, &navigation.path, &options);
        step += 1;
    }

    step
}

fn next_step<'a>(step: i64, path: &str, next: &'a (String, String)) -> &'a str {
    let idx = if step < path.len() as i64 { step } else { step % path.len() as i64 };
    let c = path.chars().nth(idx as usize).unwrap();
    if c == 'L' { &next.0 } else { &next.1 }
}

/// https://www.geeksforgeeks.org/lcm-of-given-array-elements/
fn lowest_common_multiple(input: &[i64]) -> i64 {
    let mut items = input.iter().cloned().collect::<Vec<i64>>();

    let mut result = 1;
    let mut divisor = 2;

    loop {
        let mut counter = 0;
        let mut divisible = false;

        for i in items.iter_mut() {
            if *i == 0 { return 0; }
            if *i == 1 { counter += 1; }
            if *i % divisor == 0 {
                divisible = true;
                *i /= divisor;
            }
        }

        if divisible {
            result *= divisor;
        } else {
            divisor += 1;
        }

        if counter == items.len() { return result; }
    }
}

// -------------------------------------------------------------------------------------------------
// model

struct Navigation {
    path: String,
    maze: HashMap<String, (String, String)>,
}

// -------------------------------------------------------------------------------------------------
// parsing

fn parse_file(input: &str) -> Navigation {
    let mut path = String::new();
    let mut maze = HashMap::new();

    for (i, line) in input.lines().enumerate() {
        if i == 0 {
            path = line.to_owned();
        } else if line.is_empty() {
            continue;
        } else {
            let (key, next) = line.split_once("=").unwrap();
            let options = next.trim().split_once(",")
                .map(|(a, b)| (a[1..=3].to_owned(), b[1..=3].to_owned()))
                .unwrap();

            maze.insert(key.trim().to_owned(), options);
        }
    }

    Navigation { path, maze }
}