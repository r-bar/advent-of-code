use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> anyhow::Result<()> {
    let filename = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "input.txt".to_string());
    let f = File::open(&filename)?;
    let reader = BufReader::new(f);
    for line in reader.lines() {
        println!("{}", line?.trim());
    }
    Ok(())
}

#[cfg(test)]
mod test {
    #[cfg(test)]
    mod tests {
        use super::*;

        fn #[test]
        fn example_test() {
            let example = include_str!("../../example.txt");
        }
    }
}
