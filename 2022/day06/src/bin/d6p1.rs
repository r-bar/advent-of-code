use std::collections::HashSet;

fn find_start_of_packet(data: &str) -> Option<usize> {
    let mut ptr = 0;
    while ptr + 4 < data.len() {
        let packet = &data[ptr..ptr + 4];
        let uniq: HashSet<char> = packet.chars().collect();
        if packet.len() == uniq.len() {
            return Some(ptr);
        }
        ptr += 1;
    }
    None
}

fn main() -> anyhow::Result<()> {
    let filename = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "input.txt".to_string());
    let data = String::from_utf8(std::fs::read(&filename)?)?;
    if let Some(start) = find_start_of_packet(&data) {
        println!("{}", start + 4);
    }
    Ok(())
}
