use std::cmp::Reverse;
use std::fs;

fn main() -> anyhow::Result<()> {
    let content = String::from_utf8(fs::read("input.txt")?)?;
    let mut elves: Vec<usize> = Vec::new();
    let mut elf_cals = 0usize;
    for line in content.lines() {
        match line.trim() {
            "" => {
                elves.push(elf_cals);
                elf_cals = 0;
            }
            int_str => elf_cals += int_str.parse::<usize>()?,
        }
    }
    elves.sort_by_key(|&num| Reverse(num));
    let top3: usize = elves[..3].iter().sum();
    println!("Answer: {}", top3);
    Ok(())
}
