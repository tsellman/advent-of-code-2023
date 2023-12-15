use std::fmt::{Debug, Formatter};

use crate::solutions::Harness;

pub struct Day15 {}

impl Harness for Day15 {
    fn part_1(&self, input: &str, _visualise: bool) -> i64 {
        input.split(",")
            .map(|s| hash(s))
            .sum()
    }

    fn part_2(&self, input: &str, visualise: bool) -> i64 {
        let steps: Vec<Lens> = parse_steps(input);

        let mut boxes = vec![vec![]; 256];
        for step in steps {
            let hash = step.hash() as usize;
            if visualise { println!("\n{:?}", step); }
            apply_step(step, &mut boxes[hash]);
            if visualise { _visualise(&boxes) };
        }

        boxes.iter().enumerate()
            .map(|(i, b)| calc_box(i as i64, b))
            .sum()
    }
}

// ----------------

fn hash(value: &str) -> i64 {
    let mut hash = 0;
    for c in value.chars() {
        // 1. Determine the ASCII code for the current character of the string.
        // 2. Increase the current value by the ASCII code you just determined.
        hash += c as i64;
        // 3. Set the current value to itself multiplied by 17.
        hash *= 17;
        // 4. Set the current value to the remainder of dividing itself by 256.
        hash %= 256;
    }
    hash
}

fn calc_box(num: i64, boxx: &[Lens]) -> i64 {
    boxx.iter().enumerate()
        .map(|(i, lens)| (1 + num) * (i as i64 + 1) * lens.focus)
        .sum()
}

fn parse_steps(input: &str) -> Vec<Lens> {
    input.split(",")
        .map(|s| if s.contains("=") {
            let (l, f) = s.split_once("=").unwrap();
            Lens::new(l, f.parse().unwrap())
        } else {
            Lens::new(&s[..s.len() - 1], 0)
        })
        .collect()
}

fn apply_step<'a>(step: Lens<'a>, boxx: &mut Vec<Lens<'a>>) {
    if step.focus == 0 { // 0 implies '-'
        boxx.retain(|l| l.label != step.label);
    } else {
        match boxx.iter_mut().find(|l| l.label == step.label) {
            Some(e) => *e = step,
            None => boxx.push(step)
        };
    }
}

// -------------------------------------------------------------------------------------------------
// model

#[derive(Clone)]
struct Lens<'a> {
    label: &'a str,
    focus: i64,
}

impl Lens<'_> {
    fn new(label: &str, focus: i64) -> Lens {
        Lens { label, focus }
    }

    fn hash(&self) -> i64 {
        hash(self.label)
    }
}

impl Debug for Lens<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.label, self.focus)
    }
}

// -------------------------------------------------------------------------------------------------
// visualisation

fn _visualise(boxes: &[Vec<Lens>]) {
    for (i, b) in boxes.iter().enumerate() {
        if !b.is_empty() {
            println!("Box {}: {:?}", i, b);
        }
    }
}


// -------------------------------------------------------------------------------------------------
// tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash() {
        assert_eq!(hash("HASH"), 52);
    }
}