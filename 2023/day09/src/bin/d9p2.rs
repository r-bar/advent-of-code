use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::str::FromStr;

struct Reading(Vec<i32>);

impl FromStr for Reading {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut v = Vec::new();
        for num in s.split_whitespace() {
            v.push(num.parse()?);
        }
        Ok(Reading(v))
    }
}

impl Reading {
    fn delta(history: &Vec<i32>) -> Vec<i32> {
        let mut delta = Vec::with_capacity(history.len() - 1);
        for i in 1..history.len() {
            delta.push(history[i] - history[i - 1]);
        }
        delta
    }

    fn extrapolate(&self) -> i32 {
        let mut deltas: Vec<Vec<i32>> = vec![self.0.clone()];
        for depth in 0.. {
            if depth > 100 { panic!("Too deep"); }
            let last = deltas.pop().unwrap();
            if last.iter().all(|&x| x == 0) {
                break;
            }
            let next = Reading::delta(&last);
            deltas.extend([last, next]);
        }
        let mut result = 0;
        for row in deltas.iter().rev() {
            result += row.last().unwrap()
        }
        result
    }

    fn extrapolate_backwards(&self) -> i32 {
        let mut deltas: Vec<Vec<i32>> = vec![self.0.clone()];
        for depth in 0.. {
            if depth > 100 { panic!("Too deep"); }
            let last = deltas.pop().unwrap();
            if last.iter().all(|&x| x == 0) {
                break;
            }
            let next = Reading::delta(&last);
            deltas.extend([last, next]);
        }
        let mut result = 0;
        for row in deltas.iter().rev() {
            result = row.first().unwrap() - result;
        }
        result
    }
}

fn main() -> anyhow::Result<()> {
    let filename = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "input.txt".to_string());
    let f = File::open(filename)?;
    let reader = BufReader::new(f);
    let mut total = 0;
    for line in reader.lines() {
        let line = line?;
        let reading: Reading = line.parse()?;
        total += reading.extrapolate_backwards();
    }
    println!("Total: {}", total);
    Ok(())
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;
    const EXAMPLE: &str = include_str!("../../example.txt");

    #[test]
    fn test_part2_example() {
        let mut total = 0;
        let expected = [-3, 0, 5];
        for (i, line) in EXAMPLE.lines().enumerate() {
            let reading: Reading = line.parse().unwrap();
            let extrapolated = reading.extrapolate_backwards();
            assert_eq!(extrapolated, expected[i]);
            total += extrapolated;
        }
        assert_eq!(total, 2);
    }
}
