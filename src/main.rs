use std::fs::read_to_string;
use std::io;

use clap::Parser;

mod solutions;

#[derive(Parser, Debug)]
struct Args {
    day: u8,

    #[arg(short, long, required = false)]
    input: Option<String>,

    #[arg(short, long, required = false)]
    visualise: bool,
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    // load input data
    println!("\nRunning day: {}", args.day);
    let input = args.input
        .map(|filename| read_to_string(&filename))
        .unwrap_or_else(|| load_input(args.day))?;

    // execute part 1
    let solution = solutions::get_solution(args.day);
    let ans_1 = solution.part_1(&input, args.visualise);
    println!("\nPart 1: {}", ans_1);

    // execute part 2
    if args.visualise { println!(); }
    let ans_2 = solution.part_2(&input, args.visualise);
    println!("\nPart 2: {}", ans_2);

    Ok(())
}

fn load_input(day_num: u8) -> io::Result<String> {
    read_to_string(format!("inputs/day{}", day_num))
}

// -------------------------------------------------------------------------------------------------
// tests

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use crate::load_input;
    use crate::solutions::get_solution;

    /// Run solution to part 1 and check against expected answer
    fn expect_part_1(day_num: u8, answer: i64) {
        let input = load_input(day_num).unwrap();
        let solution = get_solution(day_num);
        assert_eq!(solution.part_1(&input, false), answer, "Day {}, Part 1 should be: {}", day_num, answer);
    }

    /// Run solution to part 1 and check against expected answer
    fn expect_part_2(day_num: u8, answer: i64) {
        let input = load_input(day_num).unwrap();
        let solution = get_solution(day_num);
        assert_eq!(solution.part_2(&input, false), answer, "Day {}, Part 2 should be: {}", day_num, answer);
    }

    fn load_answers(day_num: u8) -> (Option<i64>, Option<i64>) {
        // Answers, if they exist, should be in files under answers/day1 etc
        read_to_string(format!("answers/day{}", day_num)).ok()
            .map(|answers| {
                let answers = answers.lines()
                    .map(|a| a.parse().unwrap())
                    .collect::<Vec<_>>();

                match answers.len() {
                    0 => (None, None),
                    1 => (Some(answers[0]), None),
                    _ => (Some(answers[0]), Some(answers[1]))
                }
            })
            .unwrap_or((None, None))
    }

    #[test]
    fn check_answers() {
        // test the solutions match expected answers for any days which have answer files
        for day in 1..=25 {
            let (answer_1, answer_2) = load_answers(day);

            if let Some(answer) = answer_1 {
                expect_part_1(day, answer);
            }
            if let Some(answer) = answer_2 {
                expect_part_2(day, answer);
            }
        }
    }
}