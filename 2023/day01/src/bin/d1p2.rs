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

/// Add two values together, clamp the result to the max value
fn add_up_to<T>(a: T, b: T, max: T) -> T
where
    T: std::ops::Add<Output = T> + std::cmp::Ord,
{
    let sum = a + b;
    if sum > max {
        max
    } else {
        sum
    }
}

fn find_first_digit(line: &str) -> Option<(usize, usize)> {
    for (i, c) in line.chars().enumerate() {
        if c.is_ascii_digit() {
            let num = c.to_digit(10).unwrap();
            return Some((i, num as usize));
        }
        for (name, num) in DIGIT_NAMES.iter().zip(1..) {
            if &line[i..add_up_to(i, name.len(), line.len())] == *name {
                return Some((i, num));
            }
        }
    }
    None
}

fn find_last_digit(line: &str) -> Option<(usize, usize)> {
    for i in (0..line.chars().count()).rev() {
        let c = line.chars().nth(i).unwrap();
        if c.is_ascii_digit() {
            let num = c.to_digit(10).unwrap();
            return Some((i, num as usize));
        }
        for (name, num) in DIGIT_NAMES.iter().zip(1..) {
            //dbg!(i);
            //dbg!(name);
            //dbg!(add_up_to(i, name.len(), name.len()));
            if &line[i..add_up_to(i, name.len(), line.len())] == *name {
                return Some((i, num));
            }
        }
    }
    None
}

fn main() -> anyhow::Result<()> {
    let filename = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "input.txt".to_string());
    let f = File::open(filename)?;
    let reader = BufReader::new(f);
    let mut sum: usize = 0;
    for (i, line) in reader.lines().enumerate() {
        let line = line?;
        let (first_char_pos, first_digit) = find_first_digit(&line)
            .unwrap_or_else(|| panic!("No digit found in line: {i}"));
        let (_, second_digit) = find_last_digit(&line[first_char_pos..])
            .unwrap_or_else(|| panic!("No second digit found in line: {i}"));
        let num: usize = first_digit * 10 + second_digit;
        println!("{num}: {line}");
        sum += num;
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
