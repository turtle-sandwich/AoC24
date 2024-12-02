pub mod day01;
pub mod day02;

/// Common return type of puzzle solutions
pub type PuzzleResult = Result<String, Box<dyn std::error::Error>>;

pub fn solve(day: usize, part: usize, input: String) -> PuzzleResult {
    match (day, part) {
        (1, 1) => day01::part1(input),
        (1, 2) => day01::part2(input),
        (2, 1) => day02::part1(input),
        (day, part) => Err(format!("no solution for day {day} part {part}"))?,
    }
}
