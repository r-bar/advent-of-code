use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn quadratic(time: f64, distance: f64) -> f64 {
    let s1 = time + ((time * time) - (4.0 * distance)).sqrt() / 2.0;
    let s2 = time - ((time * time) - (4.0 * distance)).sqrt() / 2.0;
    s1.floor() - s2.floor()
}

fn main() -> anyhow::Result<()> {
    let filename = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "input.txt".to_string());
    let f = File::open(filename)?;
    let reader = BufReader::new(f);
    let mut time = 0.0;
    let mut distance_record = 0.0;
    for line in reader.lines() {
        let line = line?;
        let parts: Vec<_> = line.split_whitespace().collect();
        match parts[0] {
            "Time:" => {
                let s: String = parts[1..].iter().flat_map(|s| s.chars()).collect();
                time = s.parse::<f64>()?;
            }
            "Distance:" => {
                let s: String = parts[1..].iter().flat_map(|s| s.chars()).collect();
                distance_record = s.parse::<f64>()?;
            }
            _ => continue,
        }
    }
    let margin = quadratic(time, distance_record);
    println!("{margin}");
    Ok(())
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn example_test() {
        let time = 71530.0;
        let distance = 940200.0;
        assert_eq!(quadratic(time, distance), 71503.0);
    }
}
