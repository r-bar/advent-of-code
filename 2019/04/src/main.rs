use std::cmp::Ordering;
use std::convert::TryFrom;
use std::env;

//type BoxResult<T> = Result<T, Box<dyn std::error::Error>>;
#[derive(Debug, PartialEq, PartialOrd, Clone)]
struct Passcode {
    digits: [u8; 6],
}

impl std::convert::TryFrom<&str> for Passcode {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value
            .parse::<u32>()
            .map_err(|_| "invalid number")
            .and_then(Passcode::try_from)
    }
}

impl std::convert::TryFrom<u32> for Passcode {
    type Error = &'static str;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        if value < 100_000 || 1_000_000 < value {
            return Err("must be a 6 digit number");
        }

        let mut n: i32 = value as i32;
        let mut digits: [u8; 6] = [0; 6];
        for i in 0..6 as i32 {
            let power: i32 = (10 as i32).pow(5 - i as u32);
            let digit_val: u8 = (n / power) as u8;
            digits[i as usize] = digit_val;
            n -= (digit_val as i32) * power;
        }

        Ok(Passcode { digits })
    }
}

impl Passcode {
    fn validate_digits_cannot_decrease(&self) -> Result<(), &'static str> {
        let mut prev = &self.digits[0];
        for i in &self.digits[1..] {
            if i < prev {
                return Err("passcode digits cannot decrease");
            }
            prev = i;
        }
        Ok(())
    }

    fn validate_groups_of(
        &self,
        comparison: &[Ordering],
        value: usize,
    ) -> Result<(), &'static str> {
        let mut groups: Vec<Vec<u8>> = Vec::new();
        for i in &self.digits {
            let maybe_last_group = groups.pop();
            let maybe_last_val = maybe_last_group.clone().and_then(|g| g.last().map(|i| *i));
            match (maybe_last_val, maybe_last_group) {
                (None, None) => groups.push(vec![*i]),
                (Some(last_val), Some(ref mut last_group)) if last_val == *i => {
                    last_group.push(*i);
                    groups.push(last_group.to_vec());
                }
                (_, Some(last_group)) => {
                    groups.push(last_group);
                    groups.push(vec![*i]);
                }
                _ => unreachable!(),
            }
        }

        match groups
            .iter()
            .find(|grp| comparison.contains(&grp.len().cmp(&value)))
        {
            None => return Err("needs at least 1 group of exactly 2 identical digits"),
            Some(_) => Ok(()),
        }
    }
}

impl std::convert::Into<u32> for Passcode {
    fn into(self) -> u32 {
        let mut output: u32 = 0;
        for (i, digit) in self.digits.iter().enumerate() {
            output += *digit as u32 * (10 ^ (5 - i)) as u32;
        }
        output
    }
}

fn main() {
    let range_min: u32 = env::args()
        .nth(1)
        .expect("2 arguments are required")
        .parse()
        .expect("invalid minimum range");
    let range_max: u32 = env::args()
        .nth(2)
        .expect("2 arguments are required")
        .parse()
        .expect("invalid maximum range");

    println!("part 1");
    let passcodes: Vec<Passcode> = (range_min..=range_max)
        .filter_map(|i| Passcode::try_from(i).ok())
        .collect();
    let part1_passcodes: Vec<Passcode> = passcodes
        .iter()
        .filter(|p| p.validate_digits_cannot_decrease().is_ok())
        .filter(|p| {
            p.validate_groups_of(&[Ordering::Greater, Ordering::Equal], 2)
                .is_ok()
        })
        .map(|p| p.clone())
        .collect();
    println!("number of passcodes {}", part1_passcodes.len());
    println!("sample passcodes {:?}", &part1_passcodes[0..10]);

    println!("\npart 2");
    let part2_passcodes: Vec<Passcode> = passcodes
        .iter()
        .filter(|p| p.validate_digits_cannot_decrease().is_ok())
        .filter(|p| p.validate_groups_of(&[Ordering::Equal], 2).is_ok())
        .map(|p| p.clone())
        .collect();
    println!("number of passcodes {}", part2_passcodes.len());
    println!("sample passcodes {:?}", &part2_passcodes[0..10]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_passcode() {
        assert_eq!(
            Passcode::try_from(111111),
            //Err("needs at least 1 group of exactly 2 identical digits"),
            Ok(Passcode {
                digits: [1, 1, 1, 1, 1, 1]
            }),
        );
        assert_eq!(
            Passcode::try_from(223450),
            //Err("passcode digits cannot decrease"),
            Ok(Passcode {
                digits: [2, 2, 3, 4, 5, 0]
            }),
        );
        assert_eq!(
            Passcode::try_from(123789),
            //Err("needs at least 1 group of exactly 2 identical digits"),
            Ok(Passcode {
                digits: [1, 2, 3, 7, 8, 9]
            }),
        );
    }
}
