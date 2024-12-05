pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;

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
        (day, part) => Err(format!("no solution for day {day} part {part}"))?,
    }
}
