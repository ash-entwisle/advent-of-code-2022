use std::collections::HashSet;

pub fn run(data: &String) -> i32 {
    // init variables
    let mut total: i32 = 0;
    let mut sets: Vec<HashSet<char>> = vec![
        HashSet::new(),
        HashSet::new(),
        HashSet::new(),
    ];

    // loop over the lines
    for (index, line) in data.lines().enumerate() {

        // set the coresponding set to a set of the current line
        match index % 3 {
            0 => { sets[index % 3] = line.chars().collect(); },
            1 => { sets[index % 3] = line.chars().collect(); },
            2 => { sets[index % 3] = line.chars().collect(); },
            _ => {}
        }

        // if the index is 3, check the sets
        if index % 3 == 2 {
            for character in sets[0].intersection(&sets[1]).cloned() {
                if sets[2] .contains(&character) {
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
