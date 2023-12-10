// The newly-improved calibration document consists of lines of text;
// each line originally contained a specific calibration value that the
// Elves now need to recover. On each line, the calibration value can be
// found by combining the first digit and the last digit (in that order)
// to form a single two-digit number.

// For example:

// 1abc2
// pqr3stu8vwx
// a1b2c3d4e5f
// treb7uchet

// In this example, the calibration values of these four lines are 12,
// 38, 15, and 77. Adding these together produces 142.

// Consider your entire calibration document. What is the sum of all of
// the calibration values?

use std::env;
use std::error::Error;
use std::fs;

const RADIX: u32 = 10;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Not enough arguments")
    }
    let path = args[1].clone();
    let result = read_lines(&path);

    result.map(|r| println!("{r}"))
}

fn read_lines(path: &str) -> Result<u32, Box<dyn Error>> {
    let contents = fs::read_to_string(path)?;

    let mut result = 0;

    for line in contents.lines() {
        // parse_line(&line)
        println!("{}", line);
        result += parse_line(&line);
    }

    Ok(result)
}

fn parse_line(line: &str) -> u32 {
    let mut parsed = line
        .chars()
        .into_iter()
        .map(|x| x.to_digit(RADIX))
        .flatten();
    let fst = parsed.next();
    let lst = parsed.last();

    match (fst, lst) {
        (Some(f), Some(l)) => (10 * f) + l,
        (Some(f), None) => 11 * f,
        _ => 0,
    }
}
