use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(Copy, Debug, Clone, PartialEq)]
enum Choice {
    Rock,
    Paper,
    Scissors,
}

// Each entry beats the previous
const WINNERS: [Choice; 3] = [Choice::Rock, Choice::Paper, Choice::Scissors];

impl Choice {
    fn winners_index(self) -> usize {
        let (index, _) = WINNERS
            .into_iter()
            .enumerate()
            .find(|(_i, c)| self == *c)
            .unwrap();
        index
    }

    fn would_beat(self) -> Choice {
        WINNERS[(self.winners_index() + 2) % 3]
    }

    fn would_lose_to(self) -> Choice {
        WINNERS[(self.winners_index() + 1) % 3]
    }

    fn play(self, other: Self) -> Outcome {
        if self == other {
            return Outcome::Tie;
        } else if self.would_lose_to() == other {
            return Outcome::Lose;
        } else if self.would_beat() == other {
            return Outcome::Win;
        }
        unreachable!()
    }

    fn value(&self) -> usize {
        match self {
            Choice::Rock => 1,
            Choice::Paper => 2,
            Choice::Scissors => 3,
        }
    }
}

type Game = (Choice, Choice);

#[derive(Copy, Debug, Clone, PartialEq)]
enum Outcome {
    Win,
    Lose,
    Tie,
}

fn score(game: &Game) -> (usize, usize) {
    let left = game.0.value();
    let right = game.1.value();
    match game.0.play(game.1) {
        Outcome::Tie => (left + 3, right + 3),
        Outcome::Win => (left + 6, right),
        Outcome::Lose => (left, right + 6),
    }
}

fn parse_line(line: &str) -> anyhow::Result<Game> {
    let choices: Vec<_> = line.trim().split(' ').collect();
    let left = match choices[0] {
        "A" => Choice::Rock,
        "B" => Choice::Paper,
        "C" => Choice::Scissors,
        s => anyhow::bail!("'{}' is not a valid left value", s),
    };
    let right = match choices[1] {
        "X" => left.would_beat(),
        "Y" => left,
        "Z" => left.would_lose_to(),
        s => anyhow::bail!("'{}' is not a valid right value", s),
    };
    Ok((left, right))
}

fn main() -> anyhow::Result<()> {
    let filename = env::args()
        .nth(1)
        .unwrap_or_else(|| String::from("input.txt"));
    let f = File::open(&filename)?;
    let reader = BufReader::new(f);
    let mut games: Vec<Game> = Vec::with_capacity(2500);
    for line in reader.lines() {
        let line = line?;
        let game = parse_line(&line)?;
        println!("{} -> {:?} -> {:?}", &line, &game, score(&game));
        games.push(game)
    }
    let myscore: usize = games.iter().map(|&game| score(&game).1).sum();
    println!("{}", myscore);
    Ok(())
}
