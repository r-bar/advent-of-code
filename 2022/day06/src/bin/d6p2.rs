use std::collections::HashSet;

fn find_unique_window(size: usize, data: &[impl PartialEq]) -> Option<usize> {
    let mut ptr = 0;
    for (i, c) in data.iter().enumerate() {
        if let Some(last_index) = data[ptr..i].iter().position(|w| w == c) {
            ptr += last_index + 1;
        } else if i - ptr + 1 == size {
            return Some(ptr);
        }
    }
    None
}

fn main() -> anyhow::Result<()> {
    let filename = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "input.txt".to_string());
    let data: Vec<char> = String::from_utf8(std::fs::read(&filename)?)?
        .chars()
        .collect();
    match find_unique_window(4, &data) {
        Some(packet_start) => println!("Start of packet: {}", packet_start),
        None => println!("Could not find packet start"),
    }
    match find_unique_window(14, &data) {
        Some(message_start) => println!("Start of message: {}", message_start),
        None => println!("Could not find message start"),
    }
    Ok(())
}
