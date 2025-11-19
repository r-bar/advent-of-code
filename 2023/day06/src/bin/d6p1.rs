use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(Debug, Clone, Copy)]
struct Race {
    time: u32,
    distance_record: u32,
}

impl Race {
    fn strategies(&self) -> Vec<(u32, u32)> {
        let mut result = Vec::with_capacity(self.time as usize);
        for hold in 0..self.time {
            let distance = race(self.time, hold);
            result.push((hold, distance));
        }
        result
    }
}

/// Returns the distance traveled after `t` seconds, given a `hold` time.
fn race(t: u32, hold: u32) -> u32 {
    if hold >= t {
        return 0;
    }
    let speed = hold;
    let runtime = t - hold;
    runtime * speed
}

fn main() -> anyhow::Result<()> {
    let filename = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "input.txt".to_string());
    let f = File::open(filename)?;
    let reader = BufReader::new(f);
    let mut times: Option<Vec<u32>> = None;
    let mut distances: Option<Vec<u32>> = None;
    for line in reader.lines() {
        let line = line?;
        let parts: Vec<_> = line.split_whitespace().collect();
        match parts[0] {
            "Time:" => {
                times = parts[1..].iter().map(|s| s.parse::<u32>().ok()).collect();
            }
            "Distance:" => {
                distances = parts[1..].iter().map(|s| s.parse::<u32>().ok()).collect();
            }
            _ => continue,
        }
    }
    let times = times.ok_or_else(|| anyhow::anyhow!("No times found"))?;
    let distances = distances.ok_or_else(|| anyhow::anyhow!("No distances found"))?;
    let races: Vec<_> = times
        .into_iter()
        .zip(distances)
        .map(|(time, distance)| Race {
            time,
            distance_record: distance,
        })
        .collect();
    let margin: usize = races
        .iter()
        .enumerate()
        .map(|(i, r)| {
            let winning = r.strategies()
                .into_iter()
                //.inspect(|(hold, distance)| { if i == 2 { dbg!(&i, hold, distance); } })
                .filter(|(_hold, d)| d > &r.distance_record)
                .count();
            //dbg!(&i, winning);
            winning
        })
        .product();
    dbg!(&margin);
    Ok(())
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn example_test() {
        let _example = include_str!("../../example.txt");
    }
}
