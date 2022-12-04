
pub fn run(data: &String) -> i32 {
    data.lines().collect::<Vec<&str>>().chunks(3)
        .map(|chunk| chunk[0].chars()
            .find(|c| chunk[1].contains(*c) && chunk[2].contains(*c))
            .unwrap())
        .map(|c| get_priority(c))
        .sum::<i32>()
}

fn get_priority(item: char) -> i32 {
    if item.is_uppercase() {
        return item as i32 - 38;
    } else {
        return item as i32 - 96;
    }
}
