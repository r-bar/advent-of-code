use std::{env, fs};
use regex::Regex;
use lazy_static::lazy_static;
use std::collections::HashMap;
use anyhow::Result;
use std::io::BufReader;
use std::io::prelude::*;

#[derive(Debug)]
struct Passport {
    values: HashMap<String, String>,
}

impl Passport {

    fn from_file(path: &str) -> Result<Vec<Self>> {
        lazy_static!{
            static ref VALUE_RE: Regex = Regex::new(r"(\S+):(\S+)").unwrap();
        }
        let file = fs::File::open(path)?;
        let reader = BufReader::new(file);
        let mut passports = Vec::new();
        let mut values = HashMap::new();
        for line in reader.lines() {
            let line = line?.trim().to_string();
            // blank line denotes new passport
            if line == "" {
                passports.push(Passport { values });
                values = HashMap::new();
                continue
            }
            let matches = VALUE_RE.captures_iter(&line);
            for m in matches {
                let key = m[1].to_string();
                let val = m[2].to_string();
                values.insert(key, val);
            }
        }
        if values.len() > 0 {
            passports.push(Passport { values })
        }
        Ok(passports)
    }

    fn valid(&self) -> bool {
        lazy_static! {
            static ref REQUIRED_FIELDS: [String; 7] = [
                "byr".to_string(),
                "ecl".to_string(),
                "eyr".to_string(),
                "hcl".to_string(),
                "hgt".to_string(),
                "iyr".to_string(),
                "pid".to_string(),
            ];
        }
        REQUIRED_FIELDS.iter().all(|f| self.values.get(f.into()).is_some())
    }
}

fn main() {
    let path = env::args().nth(1).expect("Please provide an input file argument.");
    let passports = Passport::from_file(&path).expect("Could not read map file");
    let valid_count = passports.iter().filter(|p| p.valid()).count();
    println!("{} valid pasports", valid_count);
}
