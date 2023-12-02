use crate::solutions::Harness;

pub struct Day1 {}

impl Harness for Day1 {
    fn part_1(&self, input: &str, _visualise: bool) -> i64 {
        input.lines()
            .map(|line| extract_number(line, to_digit))
            .sum()
    }

    fn part_2(&self, input: &str, _visualise: bool) -> i64 {
        input.lines()
            .map(|line| extract_number(line, to_digit_including_text))
            .sum()
    }
}

// ----------------

fn extract_number<F>(line: &str, to_digit: F) -> i64
    where F: Fn(&str, usize) -> Option<char>
{
    let mut first: char = ' ';
    let mut last: char = ' ';

    for i in 0..line.len() {
        let x = to_digit(line, i);
        if x.is_some() {
            if first == ' ' {
                first = x.unwrap();
                last = first;
            } else {
                last = x.unwrap();
            }
        }
    }

    let mut result = "".to_owned();
    result.push(first);
    result.push(last);

    result.trim().parse::<i64>().unwrap_or(0)
}

fn to_digit(line: &str, idx: usize) -> Option<char> {
    let c = line.chars().nth(idx).unwrap();
    if c.is_digit(10) {
        return Some(c);
    }
    None
}

fn to_digit_including_text(line: &str, idx: usize) -> Option<char> {
    let c = line.chars().nth(idx).unwrap();
    if c.is_digit(10) {
        return Some(c);
    }

    let named = [("one", '1'), ("two", '2'), ("three", '3'), ("four", '4'),
        ("five", '5'), ("six", '6'), ("seven", '7'), ("eight", '8'), ("nine", '9')];

    let sub = &line[idx..];
    for n in named {
        if sub.starts_with(n.0) {
            return Some(n.1);
        }
    }

    None
}