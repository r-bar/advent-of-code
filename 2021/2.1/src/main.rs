use std::{fs, env};
use std::str::FromStr;
use std::fmt::Display;
use std::io::BufReader;
use std::io::prelude::*;
use anyhow::{Result, anyhow};

enum Command {
    Forward(isize),
    Down(isize),
    Up(isize),
}

struct Sub {
    x: isize,
    depth: isize,
}

impl Sub {
    fn new() -> Self {
        Sub { x: 0, depth: 0 }
    }

    fn run_commands(&mut self, commands: &[Command]) {
        for command in commands {
            match command {
                Command::Forward(d) => self.x += d,
                Command::Down(d) => self.depth += d,
                Command::Up(d) => self.depth -= d,
            }
        }
    }
}

impl FromStr for Command {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tokens = s.split_ascii_whitespace();
        let command = tokens.next();
        let distance = tokens.next().and_then(|t| t.parse::<isize>().ok());
        match (command, distance) {
            (Some("forward"), Some(d)) => Ok(Command::Forward(d)),
            (Some("up"), Some(d)) => Ok(Command::Up(d)),
            (Some("down"), Some(d)) => Ok(Command::Down(d)),
            _ => Err(anyhow!("invalid command string: {}", &s)),
        }
    }
}

impl Command {
    fn from_file(path: &str) -> Result<Vec<Self>> {
        let mut commands = Vec::new();
        let file = fs::File::open(path)?;
        let reader = BufReader::new(file);
        for line in reader.lines() {
            let line = line?;
            commands.push(Command::from_str(&line)?)
        }
        Ok(commands)
    }
}

fn main() {
    let path = env::args().nth(1).expect("Input file path required");
    let commands = Command::from_file(&path).expect("Unable to parse commands");
    let mut sub = Sub::new();
    sub.run_commands(&commands);
    println!("{} commands", commands.len());
    println!("Sub position:");
    println!("x: {}", &sub.x);
    println!("depth: {}", &sub.depth);
    println!("agg: {}", &sub.x * sub.depth);
}
