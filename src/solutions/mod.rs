mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;

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
        _ => panic!("Day {} not available", day)
    }
}
