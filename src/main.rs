use aoc24::solve;

const HELP: &str = "Usage: aoc24 INPUT DAY PART";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = std::env::args();
    let path = args.nth(1).ok_or(HELP)?;
    let day: usize = args.next().ok_or(HELP)?.parse()?;
    let part: usize = args.next().ok_or(HELP)?.parse()?;
    let input = std::fs::read_to_string(&path).or(Err(format!("couldn't read {path}")))?;
    solve(day, part, input).and_then(|solution| Ok(println!("Solution: {solution}")))
}
