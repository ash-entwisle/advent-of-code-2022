pub fn run(data: &String) -> i32 {
    data.lines()
        .map(|line| (line.chars().next(), line.chars().last()))
        .map(|(a, b)| (get_choice(a), get_choice(b)))
        .map(|(a, b)| b + match a-b { -1 | 2 => 6, 0 => 3, _ => 0, })
        .sum()
}

fn get_choice(character: Option<char>) -> i32 {
    match character.unwrap() {
        'X'|'A' => return 1,
        'Y'|'B' => return 2,
        'Z'|'C' => return 3,
        _ => return 0,
    }
}
