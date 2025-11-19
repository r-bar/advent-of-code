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
            self.valid_byr()
            && self.valid_ecl()
            && self.valid_eyr()
            && self.valid_hcl()
            && self.valid_hgt()
            && self.valid_iyr()
            && self.valid_pid()
    }

    fn valid_byr(&self) -> bool {
        self.values.get("byr")
            .and_then(|byr| byr.parse::<usize>().ok())
            .map(|byr| (1920..=2002).contains(&byr))
            .unwrap_or(false)
    }

    fn valid_iyr(&self) -> bool {
        self.values.get("iyr")
            .and_then(|iyr| iyr.parse::<usize>().ok())
            .map(|iyr| (2010..=2020).contains(&iyr))
            .unwrap_or(false)
    }

    fn valid_eyr(&self) -> bool {
        self.values.get("eyr")
            .and_then(|eyr| eyr.parse::<usize>().ok())
            .map(|eyr| 2020 <= eyr && eyr <= 2030)
            .unwrap_or(false)
    }

    fn hgt<'a>(&'a self) -> Option<(usize, &'a str)> {
        self.values.get("hgt")
            .and_then(|hgt| {
                let suffix_offset = hgt.len() - 2;
                let scalar = hgt.get(..suffix_offset).and_then(|s| s.parse::<usize>().ok());
                let unit = hgt.get(suffix_offset..);
                match (scalar, unit) {
                    (Some(s), Some(u)) => Some((s, u)),
                    _ => None
                }
            })
    }

    fn valid_hgt(&self) -> bool {
        match self.hgt() {
            Some((scalar, "in")) => (59..=76).contains(&scalar),
            Some((scalar, "cm")) => (150..=193).contains(&scalar),
            _ => false,
        }
    }

    fn hcl<'a>(&'a self) -> Option<&'a str> {
        self.values.get("hcl").map(|v| v.as_str())
    }

    fn valid_hcl(&self) -> bool {
        lazy_static! {
            static ref HEX_RE: Regex = Regex::new("^#[0-9a-f]{6}$").unwrap();
        }
        self.hcl().map(|v| HEX_RE.is_match(v)).unwrap_or(false)
    }

    fn ecl<'a>(&'a self) -> Option<&'a str> {
        self.values.get("ecl").map(|v| v.as_str())
    }

    fn valid_ecl(&self) -> bool {
        match self.ecl() {
            Some("amb") => true,
            Some("blu") => true,
            Some("brn") => true,
            Some("gry") => true,
            Some("grn") => true,
            Some("hzl") => true,
            Some("oth") => true,
            _ => false,
        }
    }

    fn pid<'a>(&'a self) -> Option<&'a str> {
        self.values.get("pid").map(|v| v.as_str())
    }

    fn valid_pid(&self) -> bool {
        lazy_static! {
            static ref PID_RE: Regex = Regex::new("^[0-9]{9}$").unwrap();
        }
        self.pid().map(|pid| PID_RE.is_match(pid)).unwrap_or(false)
    }
}

fn main() {
    let path = env::args().nth(1).expect("Please provide an input file argument.");
    let passports = Passport::from_file(&path).expect("Could not read map file");
    let valid_count = passports.iter().filter(|p| p.valid()).count();
    println!("{} valid pasports", valid_count);
}
