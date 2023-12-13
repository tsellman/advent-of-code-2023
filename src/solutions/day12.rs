use std::collections::HashMap;
use std::fmt::{Debug, Formatter};

use Spring::*;

use crate::solutions::Harness;

pub struct Day12 {}

impl Harness for Day12 {
    fn part_1(&self, input: &str, _visualise: bool) -> i64 {
        let mut memo = HashMap::new();

        input.lines()
            .map(|line| {
                let (springs, rules) = parse_record(&line);
                // brute_force_count(&springs, &rules) // slow!
                count(&mut memo, &springs, &rules)
            })
            .sum()
    }

    fn part_2(&self, input: &str, _visualise: bool) -> i64 {
        let mut memo = HashMap::new();

        input.lines()
            .map(|line| {
                let (springs, rules) = parse_record(&line);
                let (springs, rules) = uncoil(springs, rules);
                count(&mut memo, &springs, &rules)
            })
            .sum()
    }
}

// ----------------

fn uncoil(springs: Vec<Spring>, rules: Vec<usize>) -> (Vec<Spring>, Vec<usize>) {
    let mut expanded_springs = Vec::new();
    let mut expanded_rules = Vec::new();

    for i in 0..5 {
        expanded_rules.extend(&rules);
        expanded_springs.extend(&springs);
        if i != 4 { expanded_springs.push(Unknown); }
    }

    (expanded_springs, expanded_rules)
}

// -------------------------------------------------------------------------------------------------
// brute-force solution

fn _brute_force_count(springs: &[Spring], rules: &[usize]) -> i64 {
    if rules.is_empty() { return 0; }

    // identify all the positions we can permute
    let places = springs.iter().enumerate()
        .filter(|(_, &s)| s == Unknown)
        .map(|(i, _)| i)
        .collect::<Vec<_>>();

    let mut permutable = springs.iter().cloned().collect::<Vec<_>>();

    let mut solutions = 0;
    for pattern in 0..=2_u64.pow(places.len() as u32) - 1 {
        _apply_pattern(&mut permutable, &places, pattern);
        if _is_solution(&permutable, rules) { solutions += 1; }
    }
    solutions
}

fn _is_solution(springs: &[Spring], rules: &[usize]) -> bool {
    // it's a solution if the groups of bad springs match the given rules
    let groups = springs.split(|s| s == &Good)
        .map(|g| g.len())
        .filter(|&l| l > 0)
        .collect::<Vec<_>>();

    &groups == rules
}

fn _apply_pattern(springs: &mut [Spring], places: &[usize], pattern: u64) {
    for i in 0..places.len() {
        let spring = if pattern & 2_u64.pow(i as u32) > 0 { Bad } else { Good };
        springs[places[i]] = spring;
    }
}

// -------------------------------------------------------------------------------------------------
// dynamic programming solution

type Key = (Vec<Spring>, Vec<usize>);
type Memo = HashMap<Key, i64>;

/// Recursive count implementation, with memoization
fn count(memo: &mut Memo, springs: &[Spring], rules: &[usize]) -> i64 {
    let key: Key = (springs.iter().cloned().collect(), rules.iter().cloned().collect());

    if let Some(&result) = memo.get(&key) {
        result
    } else {
        let result = do_count(memo, springs, rules);
        memo.insert(key, result);
        result
    }
}

fn do_count(memo: &mut Memo, springs: &[Spring], rules: &[usize]) -> i64 {
    // no springs and no rules -> valid
    if springs.is_empty() && rules.is_empty() { return 1; }
    // no springs -> invalid
    if springs.is_empty() { return 0; }
    // no rules -> valid if no more bad springs
    if rules.is_empty() {
        return if count_springs(springs, Bad) > 0 { 0 } else { 1 };
    }

    match springs[0] {
        Good => count_if_good(memo, springs, rules),
        Bad => count_if_bad(memo, springs, rules),
        Unknown => count_if_bad(memo, springs, rules) + count_if_good(memo, springs, rules)
    }
}

fn count_if_good(memo: &mut Memo, springs: &[Spring], rules: &[usize]) -> i64 {
    // skip over good springs
    count(memo, &springs[1..], rules)
}

fn count_if_bad(memo: &mut Memo, springs: &[Spring], rules: &[usize]) -> i64 {
    // how big is the current group?
    let rule = rules[0];
    // not enough springs left -> no valid permutations
    if springs.len() < rule { return 0; }

    // see if any of the next springs aren't Bad or Unknown -> no valid permutations
    let group = &springs[0..rule];
    if count_springs(group, Good) > 0 { return 0; }

    // chop the processed springs & groups
    let remaining_springs = &springs[rule..];
    let remaining_rules = &rules[1..];

    if remaining_springs.is_empty() {
        // no more springs -> valid if no more rules
        return if remaining_rules.is_empty() { 1 } else { 0 };
    }
    // dangling spring which should've been part of this group -> invalid
    if remaining_springs[0] == Bad { return 0; }
    // drop the next spring, since we know we need to treat it as Good (even if it's Unknown)
    let remaining_springs = &remaining_springs[1..];

    count(memo, &remaining_springs, &remaining_rules)
}

fn count_springs(springs: &[Spring], kind: Spring) -> usize {
    springs.iter().filter(|&&s| s == kind).count()
}

// -------------------------------------------------------------------------------------------------
// model

#[derive(Hash, Eq, PartialEq, Copy, Clone)]
enum Spring {
    Good,
    Bad,
    Unknown,
}

impl Debug for Spring {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Good => '.',
            Bad => '#',
            Unknown => '?'
        })
    }
}

// -------------------------------------------------------------------------------------------------
// parsing

fn parse_record(line: &str) -> (Vec<Spring>, Vec<usize>) {
    let (springs, hints) = line.split_once(" ").unwrap();

    let springs = springs.chars().map(|c| match c {
        '.' => Good,
        '#' => Bad,
        '?' => Unknown,
        _ => panic!("Unknown symbol: {}", c)
    }).collect();

    let hints = hints.split(",").map(|n| n.parse().unwrap()).collect();

    (springs, hints)
}

// ---------------------------------------------------------------------------------------------------------------------
// tests

#[cfg(test)]
mod tests {
    use super::*;

    fn _check(record: &str, expect: i64) {
        let mut m = HashMap::new();
        let (s, r) = parse_record(record);
        assert_eq!(count(&mut m, &s, &r), expect, "`{}` should be {}", record, expect);
    }

    fn _check_raw(s: &[Spring], r: &[usize], expect: i64) {
        let mut m = HashMap::new();
        assert_eq!(count(&mut m, &s, &r), expect, "`{:?} {:?}` should be {}", s, r, expect);
    }

    fn _check_expanded(record: &str, expect: i64) {
        let mut m = HashMap::new();
        let (s, r) = parse_record(record);
        let (s, r) = uncoil(s, r);
        assert_eq!(count(&mut m, &s, &r), expect, "`{}` should be {}", record, expect);
    }

    #[test]
    fn no_springs() {
        _check_raw(&[], &[], 1);
        _check_raw(&[], &[1], 0);
    }

    #[test]
    fn no_rules() {
        _check_raw(&[Good], &[], 1);
        _check_raw(&[Bad], &[], 0);
        _check_raw(&[Unknown], &[], 1);
        _check_raw(&[Good, Good], &[], 1);
        _check_raw(&[Unknown, Unknown], &[], 1);
    }

    #[test]
    fn single_spring_single_rule() {
        _check(". 1", 0);
        _check(". 2", 0);

        _check("# 1", 1);
        _check("# 2", 0);

        _check("? 1", 1);
        _check("? 2", 0);
    }

    #[test]
    fn single_rule() {
        _check(".. 1", 0);
        _check(".. 2", 0);

        _check("## 1", 0);
        _check("## 2", 1);

        _check("#? 1", 1);
        _check("?# 1", 1);
        _check("?# 2", 1);
        _check(".? 2", 0);
        _check("?? 2", 1);
        _check("?? 1", 2);

        _check("### 3", 1);
        _check("??? 3", 1);
        _check(".?? 3", 0);
        _check("#?? 3", 1);
    }

    #[test]
    fn two_rules() {
        _check("... 1,1", 0);
        _check(".#. 1,1", 0);
        _check("#.# 1,1", 1);
        _check("#.? 1,1", 1);
        _check("#?# 1,1", 1);
        _check("?## 1,1", 0);
        _check("##? 1,1", 0);
        _check("#?? 1,1", 1);
    }

    #[test]
    fn part_1_test() {
        _check("???.### 1,1,3", 1);
        _check(".??..??...?##. 1,1,3", 4);
        _check("?#?#?#?#?#?#?#? 1,3,1,6", 1);
        _check("????.#...#... 4,1,1", 1);
        _check("????.######..#####. 1,6,5", 4);
        _check("?###???????? 3,2,1", 10);
    }

    #[test]
    fn part_2_test() {
        _check_expanded("???.### 1,1,3", 1);
        _check_expanded(".??..??...?##. 1,1,3", 16384);
        _check_expanded("?#?#?#?#?#?#?#? 1,3,1,6", 1);
        _check_expanded("????.#...#... 4,1,1", 16);
        _check_expanded("????.######..#####. 1,6,5", 2500);
        _check_expanded("?###???????? 3,2,1", 506250);
    }
}
