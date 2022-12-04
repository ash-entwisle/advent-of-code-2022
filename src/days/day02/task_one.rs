pub fn run(data: &String) -> i32 {
    let mut diff: i32 = 0;
    data.lines().map(
        |line| {
            diff = get_choice(line.chars().next().unwrap()) - get_choice(line.chars().last().unwrap());
            match diff {
                -1 | 2 => 6,
                0 => 3,
                _ => 0,
            }
        }
    ).sum()
}

fn get_choice(character: char) -> i32 {
    match character.to_uppercase().next().unwrap() {
        'X'|'A' => return 1,
        'Y'|'B' => return 2,
        'Z'|'C' => return 3,
        _ => return 0,
    }
}