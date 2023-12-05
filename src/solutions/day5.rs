use std::cmp::Ordering;
use std::collections::VecDeque;
use std::fmt::{Debug, Formatter};

use crate::solutions::Harness;

pub struct Day5 {}

impl Harness for Day5 {
    fn part_1(&self, input: &str, _visualise: bool) -> i64 {
        let (seeds, mappings) = parse_input(input);

        let apply_mapping = |seed| mappings.iter()
            .fold(seed, |v, m| apply_mapping(v, m));

        // calculate the location for each input seed, by applying each
        // mapping in turn, and find the smallest value
        seeds.iter()
            .map(|&seed| apply_mapping(seed))
            .min().unwrap_or(0)
    }

    fn part_2(&self, input: &str, visualise: bool) -> i64 {
        let (seeds, mut mappings) = parse_input(input);

        // treat the seeds as ranges rather than individual items, and order them
        let mut seed_ranges = to_ranges(&seeds);
        seed_ranges.sort();
        if visualise { println!("Seed ranges: {:?}\n", seed_ranges) };

        // make sure the mappings are all sorted
        mappings.iter_mut().for_each(|m| m.sort());

        // apply the mappings for each range in bulk
        let locations = apply_mappings_to_ranges(seed_ranges, &mappings, visualise);

        // find the smallest value in the location ranges
        locations.iter()
            .map(|r| r.min)
            .min().unwrap_or(0)
    }
}

// ----------------

/// Apply the given mapping to the given input value
fn apply_mapping(value: i64, mapping: &Mapping) -> i64 {
    mapping.iter()
        .filter(|m| m.range.contains(value))
        .map(|m| value + m.offset)
        .next()
        .unwrap_or(value)
}

/// Convert a list of numbers into a list of ranges
fn to_ranges(values: &[i64]) -> Vec<Range> {
    values.chunks(2)
        .map(|v| {
            let start = v.get(0).unwrap();
            let length = v.get(1).unwrap();

            Range { min: *start, max: *start + length }
        })
        .collect()
}


fn apply_mappings_to_ranges(ranges: Vec<Range>, mappings: &[Mapping], visualise: bool) -> Vec<Range> {
    let mut result = Vec::from(ranges);
    for mapping in mappings {
        result = result.into_iter()
            .flat_map(|range| apply_mapping_to_range(range, mapping))
            .collect();

        if visualise {
            println!("Applying mapping:\n\t {:?}", mapping);
            println!("Result:\n\t {:?}\n", result);
        }
    }
    result
}

/// Apply the given mapping to the input ranges
fn apply_mapping_to_range(range: Range, mapping: &Mapping) -> Vec<Range> {
    let mut result = Vec::new();

    let mut to_process = VecDeque::new();
    to_process.push_back(range);

    for transform in mapping {
        if let Some(values) = to_process.pop_front() {
            let (processed, unprocessed) = transform_range(values, transform);
            result.extend(processed);
            to_process.extend(unprocessed);
        }
    }

    result.extend(to_process);
    result
}

/// Apply the given transformation to the given range of values.
/// Returns a tuple: (processed-ranges, unprocessed-range)
fn transform_range(values: Range, transform: &Transform) -> (Vec<Range>, Option<Range>) {
    let t_range = &transform.range;

    let mut processed = Vec::new();
    let mut unprocessed = None;

    if values.max < t_range.min {
        // values below transform range -> effectively processed
        processed.push(values);
    } else if values.min > t_range.max {
        // values above transform range -> leave for a later transform
        unprocessed = Some(values);
    } else if t_range.contains(values.min) && t_range.contains(values.max) {
        // values range is a subset of transform range -> simple shift
        processed.push(values.apply_offset(transform.offset));
    } else if values.contains(t_range.min) && values.contains(t_range.max) {
        // transform range is a subset of values range -> shift overlapping portion
        processed.push(values.with_max(t_range.min - 1));
        processed.push(t_range.apply_offset(transform.offset));
        unprocessed = Some(values.with_min(t_range.max + 1));
    } else if values.contains(t_range.max) {
        // transform range overlaps lower values
        processed.push(values.with_max(t_range.max).apply_offset(transform.offset));
        unprocessed = Some(values.with_min(t_range.max + 1));
    } else {
        // transform range overlaps upper values
        processed.push(values.with_max(t_range.min - 1));
        processed.push(values.with_min(t_range.min).apply_offset(transform.offset));
    }

    (processed, unprocessed)
}

// -------------------------------------------------------------------------------------------------
// model

// ----------------
// range

/// Inclusive range
#[derive(Eq, PartialEq, Ord, Clone)]
struct Range {
    min: i64,
    max: i64,
}

impl Range {
    fn new(min: i64, max: i64) -> Range {
        Range { min, max }
    }

    fn contains(&self, value: i64) -> bool {
        self.min <= value && self.max >= value
    }

    fn apply_offset(&self, offset: i64) -> Range {
        Range::new(self.min + offset, self.max + offset)
    }

    fn with_min(&self, from: i64) -> Range {
        Range::new(from, self.max)
    }

    fn with_max(&self, to: i64) -> Range {
        Range::new(self.min, to)
    }
}

impl PartialOrd for Range {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.min.cmp(&other.min))
    }
}

impl Debug for Range {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({} -> {})", self.min, self.max)
    }
}

// ----------------
// transform

/// Transformation instruction
#[derive(Eq, PartialEq, Ord)]
struct Transform {
    /// what range of values does this apply to?
    range: Range,
    /// what offset should be applied?
    offset: i64,
}

impl PartialOrd for Transform {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.range.cmp(&other.range))
    }
}

impl Debug for Transform {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}: {}", self.range, self.offset)
    }
}

// ----------------
// mapping

/// A mapping (eg seed-to-soil) is a collection of Transforms
type Mapping = Vec<Transform>;


// -------------------------------------------------------------------------------------------------
// parsing

fn parse_input(input: &str) -> (Vec<i64>, Vec<Mapping>) {
    let mut seeds = Vec::new();
    let mut mappings = Vec::new();

    let mut mapping_in_progress = Vec::new();
    for line in input.lines() {
        if line.starts_with("seeds") {
            // list of seeds, all on one line
            seeds = line[6..].split_whitespace()
                .map(|s| s.parse().unwrap()).collect();
        } else if line.is_empty() && !mapping_in_progress.is_empty() {
            // end of mapping
            mappings.push(mapping_in_progress);
            mapping_in_progress = Vec::new();
        } else if !line.is_empty() && !line.contains("map") {
            // mapping item
            mapping_in_progress.push(to_mapping(line));
        }
    }

    if !mapping_in_progress.is_empty() {
        mappings.push(mapping_in_progress);
    }

    (seeds, mappings)
}

fn to_mapping(line: &str) -> Transform {
    let values: Vec<i64> = line.split_whitespace().map(|s| s.parse().unwrap()).collect();
    let src = values[1];
    let dst = values[0];
    let len = values[2];

    Transform { range: Range::new(src, src + len - 1), offset: dst - src }
}

// -------------------------------------------------------------------------------------------------
// tests

#[cfg(test)]
mod tests {
    use super::*;

    /// Shorthand to create a range
    fn r(min: i64, max: i64) -> Range { Range::new(min, max) }

    /// Shorthand to create a transform
    fn t(start: i64, end: i64, offset: i64) -> Transform { Transform { range: r(start, end), offset } }

    #[test]
    fn range_transformation() {
        fn t(start: i64, end: i64) -> Transform { self::t(start, end, 3) }

        // no overlap
        assert_eq!(
            (vec![r(0, 5)], None),
            transform_range(r(0, 5), &t(10, 12))
        );

        // all values transformed
        assert_eq!(
            (vec![r(5, 8)], None),
            transform_range(r(2, 5), &t(0, 10))
        );

        // middle values transformed
        assert_eq!(
            (vec![r(1, 2), r(6, 8)], Some(r(6, 7))),
            transform_range(r(1, 7), &t(3, 5))
        );

        // lower values transformed
        assert_eq!(
            (vec![r(6, 8)], Some(r(6, 7))),
            transform_range(r(3, 7), &t(1, 5))
        );

        // upper values transformed
        assert_eq!(
            (vec![r(1, 2), r(6, 8)], None),
            transform_range(r(1, 5), &t(3, 7))
        );
    }

    #[test]
    fn range_transformations() {
        assert_eq!(
            vec![r(74, 87), r(95, 95)],
            apply_mapping_to_range(
                r(81, 95),
                &vec![t(18, 24, 70), t(25, 94, -7)],
            )
        );
    }
}