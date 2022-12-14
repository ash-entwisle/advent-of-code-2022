# Advent of Code, Day 1 Task 1

## Brief

```
--- Day 3: Rucksack Reorganization ---

One Elf has the important job of loading all of the rucksacks with supplies for the jungle journey. Unfortunately, that Elf didn't quite follow the packing instructions, and so a few items now need to be rearranged.

Each rucksack has two large compartments. All items of a given type are meant to go into exactly one of the two compartments. The Elf that did the packing failed to follow this rule for exactly one item type per rucksack.

The Elves have made a list of all of the items currently in each rucksack (your puzzle input), but they need your help finding the errors. Every item type is identified by a single lowercase or uppercase letter (that is, a and A refer to different types of items).

The list of items for each rucksack is given as characters all on a single line. A given rucksack always has the same number of items in each of its two compartments, so the first half of the characters represent items in the first compartment, while the second half of the characters represent items in the second compartment.

For example, suppose you have the following list of contents from six rucksacks:

vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw

    The first rucksack contains the items vJrwpWtwJgWrhcsFMMfFFhFp, which means its first compartment contains the items vJrwpWtwJgWr, while the second compartment contains the items hcsFMMfFFhFp. The only item type that appears in both compartments is lowercase p.
    The second rucksack's compartments contain jqHRNqRjqzjGDLGL and rsFMfFZSrLrFZsSL. The only item type that appears in both compartments is uppercase L.
    The third rucksack's compartments contain PmmdzqPrV and vPwwTWBwg; the only common item type is uppercase P.
    The fourth rucksack's compartments only share item type v.
    The fifth rucksack's compartments only share item type t.
    The sixth rucksack's compartments only share item type s.

To help prioritize item rearrangement, every item type can be converted to a priority:

    Lowercase item types a through z have priorities 1 through 26.
    Uppercase item types A through Z have priorities 27 through 52.

In the above example, the priority of the item type that appears in both compartments of each rucksack is 16 (p), 38 (L), 42 (P), 22 (v), 20 (t), and 19 (s); the sum of these is 157.

Find the item type that appears in both compartments of each rucksack. What is the sum of the priorities of those item types?

--- Part Two ---

As you finish identifying the misplaced items, the Elves come to you with another issue.

For safety, the Elves are divided into groups of three. Every Elf carries a badge that identifies their group. For efficiency, within each group of three Elves, the badge is the only item type carried by all three Elves. That is, if a group's badge is item type B, then all three Elves will have item type B somewhere in their rucksack, and at most two of the Elves will be carrying any other item type.

The problem is that someone forgot to put this year's updated authenticity sticker on the badges. All of the badges need to be pulled out of the rucksacks so the new authenticity stickers can be attached.

Additionally, nobody wrote down which item type corresponds to each group's badges. The only way to tell which item type is the right one is by finding the one item type that is common between all three Elves in each group.

Every set of three lines in your list corresponds to a single group, but each group can have a different badge item type. So, in the above example, the first group's rucksacks are the first three lines:

vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg

And the second group's rucksacks are the next three lines:

wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw

In the first group, the only item type that appears in all three rucksacks is lowercase r; this must be their badges. In the second group, their badge item type must be Z.

Priorities for these items must still be found to organize the sticker attachment efforts: here, they are 18 (r) for the first group and 52 (Z) for the second group. The sum of these is 70.

Find the item type that corresponds to the badges of each three-Elf group. What is the sum of the priorities of those item types?

```

## Task 1

Task 1 seems pretty simple. We have a list of strings and we need to see if there are any characters that appear in both halves of the string. If there are, we need to add the priority of that character to a total. All we need to do is split the string in half, loop through all the characters in the left half and see if they appear in the right half. If they do, we add the priority of that character to the total.

### Process

1. loop through each line of the input file
2. split the line in half
3. loop through each character in the left half and see if it appears in the right half
4. if it does, add the priority of that character to the total

### Psuedocode

```py
for line in data:
    left, right = line.split(line.length / 2)
    for char in left:
        if char in right:
            total += priority(char)

# priority(char) returns the priority of the character
```

### Code

First we initialise total which will hold the total priority of all the characters that appear in both halves of the string. We then loop through each line of the input file. We split the line in half and then loop through each character in the left half. If the character appears in the right half, we add the priority of that character to the total. The `get_priority` function returns the priority of the character based on its ASCII value. If the character is lowercase, we subtract 96 from its ASCII value to get the priority. If the character is uppercase, we subtract 64 from its ASCII value to get the priority.

```rs
pub fn run(data: &String) -> i32 {
    // initialize total
    let mut total: i32 = 0;
    // loop through lines
    for line in data.lines() {
        // split line in half
        let (left, right) = line.split_at(line.len() / 2);
        // if there is a match, increase score by priority
        for character in left.chars() {
            if right.contains(character) {
                total += get_priority(character);
                break;
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
```

## Part 2



### Process

1. 

### Psuedocode

```py
# TODO
```

### Code

// TODO

```rs
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
```
