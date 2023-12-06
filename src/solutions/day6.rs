use std::iter;

use crate::solutions::Harness;

pub struct Day6 {}

impl Harness for Day6 {
    fn part_1(&self, input: &str, _visualise: bool) -> i64 {
        let races = parse_races(input);

        races.iter()
            .map(|r| count_winners(&r))
            .product()
    }

    fn part_2(&self, input: &str, _visualise: bool) -> i64 {
        let race = parse_single_race(input);
        count_winners(&race)
    }
}


/// find all values of 'hold' (h) where the resulting distance (d) is
/// greater than the race record
fn count_winners(race: &Race) -> i64 {
    // 0 and race.time cannot win
    (1..race.time)
        .map(|h| h * (race.time - h))
        .filter(|&d| d > race.record)
        .count() as i64
}

// -------------------------------------------------------------------------------------------------
// model

#[derive(Debug)]
struct Race {
    time: i64,
    record: i64,
}

// -------------------------------------------------------------------------------------------------
// parsing

fn parse_races(input: &str) -> Vec<Race> {
    fn parse_numbers(value: &str) -> Vec<i64> {
        value.split_once(":").unwrap().1
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect()
    }

    _parse_file(input, parse_numbers)
}

fn parse_single_race(input: &str) -> Race {
    fn parse_number(value: &str) -> Vec<i64> {
        let num = value.split_once(":").unwrap().1
            .split_whitespace()
            .fold(String::new(), |a, b| a + b)
            .parse().unwrap();
        vec![num]
    }

    _parse_file(input, parse_number).remove(0)
}

fn _parse_file<F: Fn(&str) -> Vec<i64>>(input: &str, parse_numbers: F) -> Vec<Race> {
    let mut times: Vec<i64> = Vec::new();
    let mut distances: Vec<i64> = Vec::new();
    for line in input.lines() {
        if line.contains("Time:") { times = parse_numbers(line); }
        if line.contains("Distance:") { distances = parse_numbers(line); }
    }

    iter::zip(times, distances)
        .map(|(time, record)| Race { time, record })
        .collect()
}
