mod day1;
mod day2;

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
        _ => panic!("Day {} not available", day)
    }
}
