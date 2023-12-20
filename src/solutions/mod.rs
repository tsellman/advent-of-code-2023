mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;
mod day12;
mod day13;
mod day15;
mod day16;
mod day18;
mod day19;

/// Common structure for solution to a day's challenge
pub trait Harness {
    /// Calculate part 1 answer for the given input
    fn part_1(&self, input: &str, visualise: bool) -> i64;

    /// Calculate part 2 answer for the given input
    fn part_2(&self, input: &str, visualise: bool) -> i64;
}


/// Get the solution for a specific day
pub fn get_solution(day: u8) -> Box<dyn Harness> {
    match day {
        1 => Box::new(day1::Day1 {}),
        2 => Box::new(day2::Day2 {}),
        3 => Box::new(day3::Day3 {}),
        4 => Box::new(day4::Day4 {}),
        5 => Box::new(day5::Day5 {}),
        6 => Box::new(day6::Day6 {}),
        7 => Box::new(day7::Day7 {}),
        8 => Box::new(day8::Day8 {}),
        9 => Box::new(day9::Day9 {}),
        10 => Box::new(day10::Day10 {}),
        11 => Box::new(day11::Day11 {}),
        12 => Box::new(day12::Day12 {}),
        13 => Box::new(day13::Day13 {}),
        15 => Box::new(day15::Day15 {}),
        16 => Box::new(day16::Day16 {}),
        18 => Box::new(day18::Day18 {}),
        19 => Box::new(day19::Day19 {}),
        _ => panic!("Day {} not available", day)
    }
}
