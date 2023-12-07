use std::cmp::Ordering;
use std::collections::HashMap;
use std::iter::zip;

use crate::solutions::Harness;

pub struct Day7 {}

impl Harness for Day7 {
    fn part_1(&self, input: &str, _visualise: bool) -> i64 {
        let jokers_wild = false;
        solve(input, jokers_wild)
    }

    fn part_2(&self, input: &str, _visualise: bool) -> i64 {
        let jokers_wild = true;
        solve(input, jokers_wild)
    }
}

fn solve(input: &str, jokers_wild: bool) -> i64 {
    // parse hands
    let mut hands = input.lines()
        .map(|line| parse_hand(line, jokers_wild))
        .collect::<Vec<_>>();

    // order hands by rank
    hands.sort_by(|a, b| order(a, b, jokers_wild));

    // count winnings
    hands.iter().enumerate()
        .map(|(rank, hand)| (rank + 1) as i64 * hand.bid)
        .sum()
}

// -------------------------------------------------------------------------------------------------
// model

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug)]
enum Type {
    FiveOfAKind = 7,
    FourOfAKind = 6,
    FullHouse = 5,
    ThreeOfAKind = 4,
    TwoPair = 3,
    OnePair = 2,
    HighCard = 1,
}

#[derive(Debug)]
struct Hand {
    kind: Type,
    cards: String,
    bid: i64,
}

fn order(a: &Hand, b: &Hand, jokers_wild: bool) -> Ordering {
    a.kind.cmp(&b.kind)
        .then_with(|| compare(&a.cards, &b.cards, jokers_wild))
}

fn compare(a: &str, b: &str, jokers_wild: bool) -> Ordering {
    let card_value = |c| match c {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' if jokers_wild => 1,
        'J' => 11,
        'T' => 10,
        _ => c.to_digit(10).unwrap()
    };

    for (x, y) in zip(a.chars(), b.chars()) {
        let ord = card_value(x).cmp(&card_value(y));
        if ord != Ordering::Equal { return ord; }
    }
    Ordering::Equal
}

// -------------------------------------------------------------------------------------------------
// parsing

fn parse_hand(line: &str, jokers_wild: bool) -> Hand {
    let (card_str, bid_str) = line.split_once(" ").unwrap();

    let cards = card_str.to_owned();
    let kind = determine_hand_type(&cards, jokers_wild);
    let bid = bid_str.parse().unwrap();

    Hand { kind, cards, bid }
}

fn determine_hand_type(cards: &str, jokers_wild: bool) -> Type {
    // count cards of each type
    let mut counts = HashMap::new();
    for card in cards.chars() {
        let count = counts.entry(card).or_insert(0);
        *count += 1;
    }

    // remove jokers, if wild
    let num_jokers = if jokers_wild { counts.remove(&'J').unwrap_or(0) } else { 0 };

    // oder counts highest -> lowest
    let mut counts: Vec<i32> = counts.values().cloned().collect();
    counts.sort();
    counts.reverse();

    // add jokers to highest count
    if counts.len() == 0 {
        counts.push(num_jokers);
    } else {
        counts[0] += num_jokers;
    }

    match counts[0] {
        5 => Type::FiveOfAKind,
        4 => Type::FourOfAKind,
        3 if counts[1] == 2 => Type::FullHouse,
        3 => Type::ThreeOfAKind,
        2 if counts[1] == 2 => Type::TwoPair,
        2 => Type::OnePair,
        _ => Type::HighCard
    }
}