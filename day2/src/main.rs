// You play several games and record the information from each game (your
// puzzle input). Each game is listed with its ID number (like the 11 in
// Game 11: ...) followed by a semicolon-separated list of subsets of
// cubes that were revealed from the bag (like 3 red, 5 green, 4 blue).

// For example, the record of a few games might look like this:

// Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
// Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
// Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
// Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
// Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green

// In game 1, three sets of cubes are revealed from the bag (and then put
// back again). The first set is 3 blue cubes and 4 red cubes; the second
// set is 1 red cube, 2 green cubes, and 6 blue cubes; the third set is
// only 2 green cubes.

// The Elf would first like to know which games would have been possible
// if the bag contained only 12 red cubes, 13 green cubes, and 14 blue
// cubes?

// In the example above, games 1, 2, and 5 would have been possible if
// the bag had been loaded with that configuration. However, game 3 would
// have been impossible because at one point the Elf showed you 20 red
// cubes at once; similarly, game 4 would also have been impossible
// because the Elf showed you 15 blue cubes at once. If you add up the
// IDs of the games that would have been possible, you get 8.

use std::str::FromStr;

const RED_CUBES: u32 = 12;
const BLUE_CUBES: u32 = 14;
const GREEN_CUBES: u32 = 13;

fn main() {
    println!("Hello, world!");
}

#[derive(Eq, PartialEq)]
enum Color {
    Red,
    Green,
    Blue,
}

impl FromStr for Color {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "red" => Ok(Color::Red),
            "blue" => Ok(Color::Blue),
            "green" => Ok(Color::Green),
            _ => Err(()),
        }
    }
}

struct Pull {
    pairs: Vec<(Color, u32)>,
}

impl Pull {
    fn is_valid(&self) -> bool {
        for pair in self.pairs.into_iter() {
            if (pair.0 == Color::Red && pair.1 > RED_CUBES)
                || (pair.0 == Color::Blue && pair.1 > BLUE_CUBES)
                || (pair.0 == Color::Green && pair.1 > GREEN_CUBES)
            {
                return false;
            };
        }
        true
    }
}

struct Game {
    number: u32,
    pulled: Vec<Pull>,
}

fn evaluate_game(games: Vec<&Game>) -> u32 {
    games.into_iter().fold(0, |acc, g| j)
    // let mut score = 0;
    // for game in games.into_iter() {
    //     let is_valid: bool = game.pulled.into_iter().fol
    // }
    // score
}

fn parse_line(line: &str) -> Option<Game> {
    let mut parts = line.strip_prefix("Game ")?.split(":");
    let number: u32 = parts.next()?.parse().unwrap();

    let rest = parts.next()?;
    let pulls = rest.split(";");

    let mut pulled = Vec::new();

    for pull in pulls {
        pulled.push(parse_pull(pull));
    }
    Some(Game { number, pulled })
}

fn parse_pull(pull: &str) -> Pull {
    let pairs = pull.split(",").map(|pair| parse_pair(pair)).collect();
    Pull { pairs }
}

fn parse_pair(pair: &str) -> (Color, u32) {
    let mut pr = pair.trim().split(" ");
    let color = pr
        .next()
        .ok_or(())
        .and_then(|c| Color::from_str(c))
        .unwrap();
    let count: u32 = pr
        .next()
        .ok_or_else(|| ())
        .and_then(|c| c.parse().map_err(|e| ()))
        .unwrap();

    (color, count)
}
