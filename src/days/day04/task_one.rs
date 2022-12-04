use std::collections::HashSet;


pub fn run(data: &String) -> i32 {
    
    // init variables
    let mut total: i32 = 0;
    let mut elf1: HashSet<i32>;
    let mut elf2: HashSet<i32>;
    
    // loop over the lines
    for line in data.lines() {
        // split line at comma or -
        let split: Vec<&str> = line.split(|c| c == ',' || c == '-').collect();

        // generate sets
        elf1 = generate_set(
            split[0].parse().unwrap(), 
            split[1].parse().unwrap(),
        );
        elf2 = generate_set(
            split[2].parse().unwrap(), 
            split[3].parse().unwrap(),
        );

        // check if one of the sets is a subset of the other
        if elf1.union(&elf2).eq(&elf1) || elf1.union(&elf2).eq(&elf2) {
            total += 1;
        }
    }
    total
}

fn generate_set(start: i32, end: i32) -> HashSet<i32> {
    let mut set: HashSet<i32> = HashSet::new();
    for i in start..=end { set.insert(i); }
    set
}


