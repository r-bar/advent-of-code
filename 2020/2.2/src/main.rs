use std::{env, fs};
use std::io::BufReader;
use std::io::prelude::*;
//use std::error::Error;
use regex::Regex;
use lazy_static::lazy_static;
use anyhow::{anyhow, Result};

#[derive(Debug)]
struct PasswordCheck {
    left: usize,
    right: usize,
    char: char,
    password: String,
}

impl PasswordCheck {
    fn valid(&self) -> bool {
        let left_char = self.password.chars().nth(self.left - 1).unwrap();
        let right_char = self.password.chars().nth(self.right - 1).unwrap();
        (left_char == self.char) ^ (right_char == self.char)
    }
}

fn read_input(path: &str) -> Result<Vec<PasswordCheck>> {
    lazy_static!{
        static ref INPUT_RE: Regex = Regex::new(r"^(\d+)-(\d+) (\w): (.*)$").unwrap();
    }

    let mut output = Vec::new();
    let file = fs::File::open(path)?;
    let reader = BufReader::new(file);
    for (num, line) in reader.lines().enumerate() {
        let line = line?;
        let caps = INPUT_RE.captures(&line).ok_or(anyhow!("invalid line {}", num + 1))?;
        output.push(PasswordCheck {
            left: caps.get(1).ok_or(anyhow!("left not found on {}", num + 1))?.as_str().parse()?,
            right: caps.get(2).ok_or(anyhow!("right not found on {}", num + 1))?.as_str().parse()?,
            char: caps.get(3).ok_or(anyhow!("char not found on {}", num + 1))?.as_str().parse()?,
            password: caps.get(4).ok_or(anyhow!("password not found on {}", num + 1))?.as_str().parse()?,
        })
    }
    Ok(output)
}


fn main() {
    let path = env::args().nth(1).expect("Please provide an input file argument.");
    let passwords = read_input(&path).expect("Could not read password file");
    //println!("Hello, world!");
    //println!("{:?}", passwords.iter().take(10).collect());
    let valid = passwords.iter().filter(|p| p.valid()).count();
    println!("{} valid passwords", valid);
}
