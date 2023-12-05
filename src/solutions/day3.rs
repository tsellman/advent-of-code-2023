use std::collections::HashMap;

use crate::solutions::Harness;

pub struct Day3 {}

impl Harness for Day3 {
    fn part_1(&self, input: &str, _visualise: bool) -> i64 {
        // keep track of things which might become part numbers
        let mut pending_numbers = Vec::new();
        let mut pending_symbols = Vec::new();

        let mut result = 0;
        // scan line-by-line
        for (idx, line) in input.lines().enumerate() {
            // get the numbers and symbols from the current line
            let (numbers, symbols) = parse_line(idx, line);

            pending_numbers.extend(numbers);
            pending_symbols.extend(symbols);

            // look for any valid part numbers
            let part_numbers = find_part_numbers(&mut pending_numbers, &pending_symbols);
            result += part_numbers.iter().map(|p| p.number.value).sum::<i64>();

            // drop any pending values which can no longer be matched
            if idx > 1 {
                pending_numbers.retain(|n| n.line > idx - 1);
                pending_symbols.retain(|s| s.line > idx - 1);
            }
        }
        result
    }

    fn part_2(&self, input: &str, _visualise: bool) -> i64 {
        // keep track of things which might become part numbers, or gears
        let mut pending_numbers = Vec::new();
        let mut pending_symbols = Vec::new();
        let mut pending_gears: Vec<PartNumber> = Vec::new();

        let mut result: i64 = 0;
        // scan line-by-line
        for (idx, line) in input.lines().enumerate() {
            // get the numbers and symbols from the current line
            let (numbers, symbols) = parse_line(idx, line);

            pending_numbers.extend(numbers);
            pending_symbols.extend(symbols);

            // look for valid part numbers or gears
            let part_numbers = find_part_numbers(&mut pending_numbers, &pending_symbols);
            let mut partial_gears = find_gears(&part_numbers);
            let gears = match_gears(&mut pending_gears, &mut partial_gears);
            result += gears.iter().map(|g| g.product()).sum::<i64>();
            pending_gears.extend(partial_gears);

            // drop any pending values which can no longer be matched
            if idx > 1 {
                pending_numbers.retain(|n| n.line > idx - 1);
                pending_symbols.retain(|s| s.line > idx - 1);
                pending_gears.retain(|g| g.symbol.line > idx - 1);
            }
        }
        result
    }
}

// ----------------

fn find_part_numbers(numbers: &mut Vec<Number>, symbols: &Vec<Symbol>) -> Vec<PartNumber> {
    let mut part_numbers = Vec::new();

    let mut retain = Vec::new();
    for number in numbers.drain(..) {
        match find_adjacent_symbol(&number, &symbols) {
            Some(symbol) => part_numbers.push(PartNumber { number, symbol: symbol.clone() }),
            None => retain.push(number)
        }
    }
    numbers.extend(retain);

    part_numbers
}

fn find_gears(parts: &Vec<PartNumber>) -> Vec<PartNumber> {
    parts.iter()
        .filter(|p| p.symbol.value == '*')
        .map(|p| p.clone())
        .collect()
}

fn match_gears(partials: &mut Vec<PartNumber>, parts: &mut Vec<PartNumber>) -> Vec<Gear> {
    let mut matched = Vec::new();

    // index the partials by symbol instance
    let mut unmatched: HashMap<_, _> = partials.drain(..)
        .map(|p| (p.symbol.clone(), p))
        .collect();

    // check each new part for a match in the partials index
    for part in parts.drain(..) {
        match unmatched.remove(&part.symbol) {
            Some(p) => matched.push(Gear::new(p, part)),
            None => { unmatched.insert(part.symbol.clone(), part); }
        };
    }

    // insert any remaining unmatched gears into the partials list
    partials.extend(unmatched.values().cloned().collect::<Vec<_>>());

    matched
}

fn find_adjacent_symbol<'a>(num: &Number, symbols: &'a Vec<Symbol>) -> Option<&'a Symbol> {
    symbols.iter()
        .filter(|s| is_adjacent(num, s))
        .next()
}

fn is_adjacent(num: &Number, symbol: &Symbol) -> bool {
    let sub = |lhs: usize, rhs: usize| lhs.checked_sub(rhs).unwrap_or(0);

    let line_match = (sub(num.line, 1)..=num.line + 1).contains(&symbol.line);
    let idx_match = (sub(num.start, 1)..=num.end + 1).contains(&symbol.idx);
    line_match && idx_match
}

// -------------------------------------------------------------------------------------------------
// model

#[derive(Debug, Clone)]
struct Number {
    value: i64,
    line: usize,
    start: usize,
    end: usize,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Symbol {
    value: char,
    line: usize,
    idx: usize,
}

#[derive(Debug, Clone)]
struct PartNumber {
    number: Number,
    symbol: Symbol,
}

#[derive(Debug)]
struct Gear {
    part_a: PartNumber,
    part_b: PartNumber,
}

impl Gear {
    fn new<>(a: PartNumber, b: PartNumber) -> Gear {
        Gear { part_a: a, part_b: b }
    }

    fn product(&self) -> i64 {
        self.part_a.number.value * self.part_b.number.value
    }
}

// -------------------------------------------------------------------------------------------------
// parsing

fn parse_line(line_num: usize, line: &str) -> (Vec<Number>, Vec<Symbol>) {
    let mut symbols = Vec::new();
    let mut numbers = Vec::new();

    let mut cur_num = "".to_owned();
    for (idx, char) in line.chars().enumerate() {
        // if digit, construct numeric value
        if char.is_digit(10) {
            cur_num.push(char);
            continue;
        }

        // not digit, finish any numeric value
        if !cur_num.is_empty() {
            numbers.push(to_number(&cur_num, line_num, idx));
            cur_num.clear();
        }

        // dot is just padding
        if char == '.' { continue; }

        // symbols are hotspots for part numbers
        symbols.push(Symbol { value: char, line: line_num, idx });
    }

    if !cur_num.is_empty() {
        numbers.push(to_number(&cur_num, line_num, line.len()));
    }

    (numbers, symbols)
}

fn to_number(txt: &str, line: usize, idx: usize) -> Number {
    Number {
        value: txt.parse().unwrap(),
        line,
        start: idx - txt.len(),
        end: idx - 1,
    }
}
