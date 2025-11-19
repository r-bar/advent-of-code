use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

const DIGIT_NAMES: [&str; 9] = [
    "one",
    "two",
    "three",
    "four",
    "five",
    "six",
    "seven",
    "eight",
    "nine",
];

fn main() -> anyhow::Result<()> {
    let filename = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "input.txt".to_string());
    let f = File::open(filename)?;
    let reader = BufReader::new(f);
    let mut sum: usize = 0;
    for (i, line) in reader.lines().enumerate() {
        let line = line?;
        let (first_char_pos, first_char) = line
            .chars()
            .enumerate()
            .find(|(_, c)| c.is_ascii_digit())
            .unwrap_or_else(|| panic!("No digit found in line: {i}"));
        let last_char = line[first_char_pos..]
            .chars()
            .rev()
            .find(|c| c.is_ascii_digit())
            .unwrap_or_else(|| panic!("No digit found in line: {i}"));
        let first_digit = first_char.to_digit(10).unwrap();
        let last_digit = last_char.to_digit(10).unwrap();
        let num = first_digit * 10 + last_digit;
        println!("{num}: {line}");
        sum += num as usize;
    }
    println!("Sum: {sum}");
    Ok(())
}

#[cfg(test)]
mod test {
    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn example_test() {
            let _example = include_str!("../../example.txt");
        }
    }
}
