use std::collections::HashSet;

pub fn run(data: &String) -> i32 {
    // init variables
    let mut total: i32 = 0;
    let mut set1: HashSet<char> = HashSet::new();
    let mut set2: HashSet<char> = HashSet::new();
    let mut set3: HashSet<char> = HashSet::new();


    for (index, line) in data.lines().enumerate() {
        match index % 3 {
            0 => { set1 = line.chars().collect(); },
            1 => { set2 = line.chars().collect(); },
            2 => { set3 = line.chars().collect(); },
            _ => {}
        }

        if index % 3 == 2 {
            for character in set1.intersection(&set2).cloned() {
                if set3.contains(&character) {
                    total += get_priority(character);
                }
            }
        }
    }

    total
}

fn get_priority(item: char) -> i32 {
    if item.is_uppercase() {
        return item as i32 - 38;
    } else {
        return item as i32 - 96;
    }
}
