use std::collections::{BTreeSet, HashMap};

use crate::solutions::Harness;

pub struct Day4 {}

impl Harness for Day4 {
    fn part_1(&self, input: &str, _visualise: bool) -> i64 {
        input.lines()
            .map(|line| parse_line(line))
            .map(|card| calculate_matches(&card))
            .map(|matches| if matches > 0 { 2_u32.pow(matches - 1) } else { 0 })
            .sum::<u32>() as i64
    }

    fn part_2(&self, input: &str, _visualise: bool) -> i64 {
        let mut matches_per_card = HashMap::new();
        let mut num_copies: HashMap<u32, u32> = HashMap::new();

        input.lines()
            .map(|line| parse_line(line))
            .for_each(|card| {
                let num_matches = calculate_matches(&card);
                matches_per_card.insert(card.id, num_matches);
                num_copies.insert(card.id, 1); // insert 'original' card
            });

        // calculate how many copies of each card to magic up
        for card_id in 1..matches_per_card.len() + 1 {
            update_copies(card_id as u32, &mut num_copies, &matches_per_card);
        }

        num_copies.values().sum::<u32>() as i64
    }
}

// ----------------

fn calculate_matches(card: &Card) -> u32 {
    card.numbers.iter()
        .filter(|&n| card.winning.contains(n))
        .count() as u32
}

fn update_copies(card_id: u32, copies: &mut HashMap<u32, u32>, matches_per_card: &HashMap<u32, u32>) {
    let num_matches = matches_per_card.get(&card_id).unwrap();

    let range = card_id + 1..=card_id + *num_matches;
    for next in range {
        let n = *copies.get(&card_id).unwrap();
        if let Some(v) = copies.get_mut(&next) {
            *v += n;
        }
    }
}

// -------------------------------------------------------------------------------------------------
// model

struct Card {
    id: u32,
    numbers: Vec<u32>,
    winning: BTreeSet<u32>,
}

// -------------------------------------------------------------------------------------------------
// parsing

fn parse_line(line: &str) -> Card {
    // grab the id
    let id = line.split(":").collect::<Vec<_>>()[0][5..]
        .trim().parse::<u32>().unwrap();

    // separate numbers from winning
    let parts: Vec<&str> = line[line.find(":").unwrap_or(0) + 1..]
        .split("|").collect();

    let numbers = parts[0].split_whitespace().map(|n| n.parse().unwrap()).collect();
    let winning = parts[1].split_whitespace().map(|n| n.parse().unwrap()).collect();

    Card { id, numbers, winning }
}