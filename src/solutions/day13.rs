use std::usize;

use crate::solutions::Harness;

pub struct Day13 {}

impl Harness for Day13 {
    fn part_1(&self, input: &str, _visualise: bool) -> i64 {
        parse_file(input).iter()
            .map(|pattern| {
                let symmetry = find_symmetry(&pattern);
                score(&symmetry)
            })
            .sum()
    }

    fn part_2(&self, input: &str, _visualise: bool) -> i64 {
        parse_file(input).iter()
            .map(|pattern| {
                let original = find_symmetry(&pattern);
                let symmetry = find_smudged_symmetry(&pattern, &original);
                score(&symmetry)
            })
            .sum()
    }
}

// ----------------

fn score(symmetry: &Symmetry) -> i64 {
    if let Some(x) = symmetry.0 {
        x as i64 + 1
    } else if let Some(y) = symmetry.1 {
        100 * (y as i64 + 1)
    } else { 0 }
}

fn find_symmetry<T: AsRef<Row>>(pattern: &[T]) -> Symmetry {
    find_alternate_symmetry(pattern, &(None, None))
}

fn find_alternate_symmetry<T: AsRef<Row>>(pattern: &[T], skip: &Symmetry) -> Symmetry {
    if let Some(y) = find_y_symmetry(&pattern, &skip.1) {
        return (None, Some(y));
    }
    if let Some(x) = find_x_symmetry(&pattern, &skip.0) {
        return (Some(x), None);
    }
    (None, None)
}

fn find_y_symmetry<T: AsRef<Row>>(pattern: &[T], skip: &Option<usize>) -> Option<usize> {
    let skip_y = skip.unwrap_or(usize::MAX);
    for y in 0..pattern.len() - 1 {
        if y == skip_y { continue; }
        if is_symmetric(y, pattern) { return Some(y); }
    }
    None
}

fn find_x_symmetry<T: AsRef<Row>>(pattern: &[T], skip: &Option<usize>) -> Option<usize> {
    let len = pattern[0].as_ref().len();
    let mut transposed = vec![String::new(); len];

    for row in pattern {
        for (i, c) in row.as_ref().chars().enumerate() {
            transposed.get_mut(i).unwrap().push(c);
        }
    }

    find_y_symmetry(&transposed, skip)
}

fn is_symmetric<T: AsRef<Row>>(y: usize, pattern: &[T]) -> bool {
    let mut i = y;
    let mut j = y + 1;

    loop {
        match (pattern.get(i), pattern.get(j)) {
            (Some(r_i), Some(r_j)) if r_i.as_ref() == r_j.as_ref() => {}
            (Some(_), Some(_)) => return false,
            _ => return true
        }

        if i < 1 { return true; }
        i -= 1;
        j += 1;
    }
}

fn find_smudged_symmetry(pattern: &[&Row], original: &Symmetry) -> Symmetry {
    let width = pattern[0].len();
    let height = pattern.len();

    for y in 0..height {
        for x in 0..width {
            let smudged = smudge(x, y, &pattern);
            let (vert, horz) = find_alternate_symmetry(&smudged, original);
            if vert.is_some() || horz.is_some() {
                return (vert, horz);
            }
        }
    }

    (None, None)
}

fn smudge<T: AsRef<Row>>(x: usize, y: usize, pattern: &[T]) -> Vec<String> {
    let mut copy: Vec<String> = pattern.iter().map(|s| s.as_ref().to_owned()).collect();

    let row = copy.get_mut(y).unwrap();
    let c = row.chars().nth(x).unwrap();
    let c_prime = if c == '.' { "#" } else { "." };
    row.replace_range(x..=x, c_prime);

    copy
}

// -------------------------------------------------------------------------------------------------
// model

type Row = str;
type Pattern<'a> = Vec<&'a Row>;
type Symmetry = (Option<usize>, Option<usize>);

// -------------------------------------------------------------------------------------------------
// parsing

fn parse_file(file: &str) -> Vec<Pattern> {
    let mut result = Vec::new();

    let mut pattern = Pattern::new();
    for line in file.lines() {
        if line.is_empty() {
            result.push(pattern);
            pattern = Pattern::new();
        } else {
            pattern.push(line);
        }
    }
    if pattern.len() > 0 { result.push(pattern); }

    result
}

// -------------------------------------------------------------------------------------------------
// visualisation

fn _visualise<R: AsRef<Row>>(pattern: &[R]) {
    for row in pattern {
        println!("{}", row.as_ref());
    }
}