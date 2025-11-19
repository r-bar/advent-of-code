use std::{fs, env};
use std::io::prelude::*;
use std::io::BufReader;
//use rayon::prelude::*;

const INIT_SPAWN_DAYS: usize = 9;
const SPAWN_DAYS: usize = 7;

/// Instead of holding each individual fish as an element this vec holds the population counts for
/// each spawn day.
struct School {
    fish: Vec<usize>,
}

impl School {
    fn from_file(path: &str) -> anyhow::Result<Self> {
        let mut file = fs::File::open(path)?;
        let mut data = String::new();
        file.read_to_string(&mut data)?;
        data = data.trim().to_string();
        //let fish = data.split(',').map(|s| Fish { days_until_spawn: i8::from_str_radix(&s, 10).unwrap() }).collect();
        let fish: Vec<usize> = data.split(',')
            .map(|s| usize::from_str_radix(&s, 10).unwrap())
            .fold(vec![0; INIT_SPAWN_DAYS as usize], |mut a, i| { a[i] += 1; a });
        Ok(School { fish })
    }

    fn tick(&mut self, days: usize) {
        for day in 0..days {
            println!("day {}", day);
            let spawning = self.fish[0];
            self.fish[0] = 0;
            for group in 1..self.fish.len() {
                self.fish[group - 1] += self.fish[group];
                self.fish[group] -= self.fish[group];
            }
            self.fish[SPAWN_DAYS - 1] += spawning;
            self.fish[INIT_SPAWN_DAYS - 1] += spawning;
        }
    }
}

fn main() {
    let path = env::args().nth(1).expect("first argument must be the input file");
    let days_arg = env::args().nth(2).expect("second argument must be the number of days to simulate");
    let days = usize::from_str_radix(&days_arg, 10).expect("not a valid number of days");
    let mut school = School::from_file(&path).expect("could not load fish population data");
    school.tick(days);
    let total: usize = school.fish.iter().sum();
    println!("{} fish", total);
}
