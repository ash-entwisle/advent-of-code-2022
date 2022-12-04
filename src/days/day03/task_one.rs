
pub fn run(data: &String) -> u32 {
    data.lines()
        .map(|line| line.split_at(line.len()/2))
        .map(|(l, r)| l.chars()
            .filter(|c| r.contains(*c))
            .map(|c| get_priority(c))
            .next().unwrap_or(0))
        .sum::<u32>()
}

fn get_priority(item: char) -> u32 {
    if item.is_uppercase() {
        return item as u32 - 38;
    } else {
        return item as u32 - 96;
    }
}
