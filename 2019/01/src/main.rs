use std::io::prelude::*;
use std::{env, error, fs, io};

type BoxResult<T> = Result<T, Box<dyn error::Error>>;

fn read_input(filepath: &str) -> BoxResult<Vec<i64>> {
    let file = fs::File::open(&filepath)?;
    let mut reader = io::BufReader::new(file);
    let mut output: Vec<i64> = Vec::new();
    for line in reader.lines() {
        if let Ok(mass) = line?.parse() {
            output.push(mass);
        }
    }
    Ok(output)
}

fn required_fuel(mass: i64) -> i64 {
    mass / 3 - 2
}

fn total_fuel(module_mass: i64) -> i64 {
    let mut total = required_fuel(module_mass);
    let mut additional = required_fuel(total);
    while additional > 0 {
        total += additional;
        additional = required_fuel(additional);
    }
    total
}

fn main() {
    let filepath = env::args().nth(1).expect("input file argument required");
    let masses = read_input(&filepath).expect("error reading input file");
    let required_fuel: i64 = masses.iter().map(|mass| required_fuel(*mass)).sum();
    let total_fuel: i64 = masses.iter().map(|mass| total_fuel(*mass)).sum();
    println!("required fuel (just modules): {}", required_fuel);
    println!("required fuel (just modules + fuel weight): {}", total_fuel);
}

mod test {
    use super::*;

    #[test]
    fn test_required_fuel() {
        assert_eq!(required_fuel(12), 2);
        assert_eq!(required_fuel(14), 2);
        assert_eq!(required_fuel(1969), 654);
        assert_eq!(required_fuel(100756), 33583);
    }

    #[test]
    fn test_total_fuel() {
        assert_eq!(total_fuel(12), 2);
        assert_eq!(total_fuel(14), 2);
        assert_eq!(total_fuel(1969), 966);
        assert_eq!(total_fuel(100756), 50346);
    }
}
