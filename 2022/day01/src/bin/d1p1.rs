use std::cmp::Reverse;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> anyhow::Result<()> {
    let f = File::open("input.txt")?;
    let reader = BufReader::new(f);
    let mut elves: Vec<usize> = Vec::new();
    let mut elf_cals = 0usize;
    for line in reader.lines() {
        match line?.trim() {
            "" => {
                elves.push(elf_cals);
                elf_cals = 0;
            }
            int_str => elf_cals += int_str.parse::<usize>()?,
        }
    }
    elves.sort_by_key(|&num| Reverse(num));
    println!("Answer: {}", elves[0]);
    Ok(())
}
