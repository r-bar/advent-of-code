use std::{fs, env};
use std::io::prelude::*;
use std::io::BufReader;

const INIT_SPAWN_DAYS: i8 = 9;
const SPAWN_DAYS: i8 = 7;

struct Fish {
    days_until_spawn: i8,
}

impl Fish {
    fn new() -> Self {
        Fish { days_until_spawn: INIT_SPAWN_DAYS }
    }
    fn tick(&mut self) -> Option<Self> {
        self.days_until_spawn -= 1;
        if self.days_until_spawn < 0 {
            self.days_until_spawn += SPAWN_DAYS;
            return Some(Fish::new())
        }
        None
    }
}


struct School {
    fish: Vec<Fish>,
}

impl School {
    fn from_file(path: &str) -> anyhow::Result<Self> {
        let mut file = fs::File::open(path)?;
        let mut data = String::new();
        file.read_to_string(&mut data)?;
        data = data.trim().to_string();
        //let fish = data.split(',').map(|s| Fish { days_until_spawn: i8::from_str_radix(&s, 10).unwrap() }).collect();
        let mut fish = Vec::new();
        for str in data.split(',') {
            let days_until_spawn = i8::from_str_radix(&str, 10)?;
            fish.push(Fish { days_until_spawn });
        }
        Ok(School { fish })
    }

    fn tick(&mut self, days: usize) {
        for day in 0..days {
            println!("day {}", &day);
            let mut babies = Vec::new();
            for fish in self.fish.iter_mut() {
                if let Some(mut baby) = fish.tick() {
                    baby.tick();
                    babies.push(baby);
                }
            }
            self.fish.extend(babies);
        }
    }
}

fn main() {
    let path = env::args().nth(1).expect("first argument must be the input file");
    let days_arg = env::args().nth(2).expect("second argument must be the number of days to simulate");
    let days = usize::from_str_radix(&days_arg, 10).expect("not a valid number of days");
    let mut school = School::from_file(&path).expect("could not load fish population data");
    school.tick(days);
    println!("{} fish", school.fish.len());
}
