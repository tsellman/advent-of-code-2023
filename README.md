# Advent of Code 2023

[Advent of Code 2023](https://adventofcode.com/2023) solutions in Rust.

## Input data

Problem inputs are not included (see [AoC FAQ](https://adventofcode.com/about#faq_copying)).

Input files can either be placed into a directory called `inputs` (eg. `inputs/day1`), and then
run with just the day number:
```shell
cargo run --release -- 1 
```

or provided with the `--input` flag:
```shell
cargo run --release -- 1 --input $FILE
```

## Checking answers

To verify solutions, answer files can be placed into a directory called `answers` (eg. `answers/day1`). 

Any day for which an answer file exists will automatically be checked when the tests are run:
```shell
cargo test --release 
```