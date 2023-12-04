use crate::solutions::Harness;

pub struct Day2 {}

impl Harness for Day2 {
    fn part_1(&self, input: &str, _visualise: bool) -> i64 {
        const RULES: Rgb = (12, 13, 14);

        input.lines()
            .map(|line| parse_game(line))
            .filter(|g| is_game_possible(g, &RULES))
            .map(|g| g.id as i64)
            .sum()
    }

    fn part_2(&self, input: &str, _visualise: bool) -> i64 {
        input.lines()
            .map(|line| parse_game(line))
            .map(|g| find_min_cubes(&g.rounds))
            .map(|min| (min.0 * min.1 * min.2) as i64)
            .sum()
    }
}

// ----------------

/// Determine if the game is possible under the given rules
fn is_game_possible(game: &Game, rules: &Rgb) -> bool {
    let is_possible = |round: &Rgb| {
        round.0 <= rules.0 && round.1 <= rules.1 && round.2 <= rules.2
    };

    let rounds = &game.rounds;
    rounds.iter()
        .filter(|r| is_possible(r))
        .count() == rounds.len()
}

/// Calculate the minimum number of each colour required for the given game
fn find_min_cubes(rounds: &Vec<Rgb>) -> Rgb {
    let mut red = 0;
    let mut green = 0;
    let mut blue = 0;

    for round in rounds {
        if round.0 > red { red = round.0; }
        if round.1 > green { green = round.1; }
        if round.2 > blue { blue = round.2; }
    }

    (red, green, blue)
}

// -------------------------------------------------------------------------------------------------
// model

type Rgb = (u32, u32, u32);

struct Game {
    id: u32,
    rounds: Vec<Rgb>,
}

// -------------------------------------------------------------------------------------------------
// parsing

fn parse_game(game: &str) -> Game {
    let (id, rounds) = game.split_once(":").unwrap();
    let id = id[5..].parse::<u32>().unwrap();

    let rounds: Vec<Rgb> = rounds.split(";")
        .map(|p| parse_round(p))
        .collect();

    Game { id, rounds }
}

fn parse_round(round: &str) -> Rgb {
    let mut red = 0;
    let mut green = 0;
    let mut blue = 0;
    for cubes in round.split(",") {
        if cubes.ends_with("red") {
            red = parse_cube(cubes, "red");
        } else if cubes.ends_with("green") {
            green = parse_cube(cubes, "green");
        } else if cubes.ends_with("blue") {
            blue = parse_cube(cubes, "blue");
        }
    }

    (red, green, blue)
}

fn parse_cube(cubes: &str, colour: &'static str) -> u32 {
    let len: usize = cubes.len() - colour.len();
    cubes[..len].trim().parse().unwrap()
}