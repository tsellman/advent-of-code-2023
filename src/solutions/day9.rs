use crate::solutions::Harness;

pub struct Day9 {}

impl Harness for Day9 {
    fn part_1(&self, input: &str, _visualise: bool) -> i64 {
        let mut sum = 0;
        for line in input.lines() {
            let seq = line.split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect::<Vec<i64>>();

            let (_, next) = solve(seq);
            sum += next;
        }
        sum
    }

    fn part_2(&self, input: &str, _visualise: bool) -> i64 {
        let mut sum = 0;
        for line in input.lines() {
            let seq = line.split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect::<Vec<i64>>();

            let (prev, _) = solve(seq);
            sum += prev;
        }
        sum
    }
}

// ----------------

fn solve(seq: Vec<i64>) -> (i64, i64) {
    // if all zero, then we've bottomed out
    if seq.iter().all(|i| *i == 0) { return (0, 0); }

    // otherwise, go deeper
    let diffs = seq.windows(2)
        .map(|w| w[1] - w[0])
        .collect::<Vec<_>>();

    let (prev, next) = solve(diffs);

    (seq[0] - prev, seq[seq.len() - 1] + next)
}