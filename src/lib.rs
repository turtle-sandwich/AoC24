pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;
pub mod day08;
pub mod day09;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;
pub mod day14;

/// Common return type of puzzle solutions
pub type PuzzleResult = Result<String, Box<dyn std::error::Error>>;

pub fn solve(day: usize, part: usize, input: String) -> PuzzleResult {
    match (day, part) {
        (1, 1) => day01::part1(input),
        (1, 2) => day01::part2(input),
        (2, 1) => day02::part1(input),
        (2, 2) => day02::part2(input),
        (3, 1) => day03::part1(input),
        (3, 2) => day03::part2(input),
        (4, 1) => day04::part1(input),
        (4, 2) => day04::part2(input),
        (5, 1) => day05::part1(input),
        (5, 2) => day05::part2(input),
        (6, 1) => day06::part1(input),
        (6, 2) => day06::part2(input),
        (7, 1) => day07::part1(input),
        (7, 2) => day07::part2(input),
        (8, 1) => day08::part1(input),
        (8, 2) => day08::part2(input),
        (9, 1) => day09::part1(input),
        (9, 2) => day09::part2(input),
        (10, 1) => day10::part1(input),
        (10, 2) => day10::part2(input),
        (11, 1) => day11::part1(input),
        (11, 2) => day11::part2(input),
        (12, 1) => day12::part1(input),
        (12, 2) => day12::part2(input),
        (13, 1) => day13::part1(input),
        (13, 2) => day13::part2(input),
        (14, 1) => day14::part1(input),
        (14, 2) => day14::part2(input),
        (day, part) => Err(format!("no solution for day {day} part {part}"))?,
    }
}
