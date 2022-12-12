use std::collections::HashMap;
use std::collections::HashSet;
use std::{fs, path::Path};

//generate map of priorities and return it
fn build_hash_map() -> HashMap<char, u32> {
    let mut map: HashMap<char, u32> = HashMap::new();
    let mut priority: u32 = 1;
    let order = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    for char in order.chars() {
        map.insert(char, priority);
        priority += 1;
    }
    return map;
}

// splits the string in two halves, in our AOC3 example this represents opening both compartments of the backpack
fn open_backpacks(backback_string: &str) -> Option<(&str, &str)> {
    if backback_string.len() % 2 != 0 {
        None
    } else {
        Some((
            &backback_string[0..backback_string.len() / 2],
            &backback_string[backback_string.len() / 2..backback_string.len()],
        ))
    }
}

// catalogizes items in compartment of backpack
fn catalogize_backpack(backback_string: &str) -> HashSet<char> {
    let mut catalogue: HashSet<char> = HashSet::new();
    backback_string.chars().for_each(|item| {
        catalogue.insert(item);
    });
    catalogue
}

// picks first item of string that appears in catalogue
fn first_in_catalogue(catalogue: HashSet<char>, items: &str) -> Option<char> {
    for item in items.chars() {
        if catalogue.contains(&item) {
            return Some(item);
        }
    }
    None
}

fn rummage_thru_backpacks(content: &String) {
    let mut sum_priorities: u32 = 0;
    let map = build_hash_map();
    for backpack in content.trim_end_matches("\n").split("\n") {
        let (front, back) = open_backpacks(backpack).unwrap();
        match first_in_catalogue(catalogize_backpack(front), back) {
            Some(item_in_both) => {
                sum_priorities += map.get(&item_in_both).unwrap();
            }
            _ => {}
        }
    }
    println!(
        "Sum of all priorities of all doppelgaengers: {}",
        sum_priorities
    )
}

// figure out what all the elfs in the group have in common
fn inspect_elf_group(group: &[&str]) -> Option<char> {
    let mut sets: Vec<HashSet<char>> = vec![];
    for elf in group {
        let mut set: HashSet<char> = HashSet::new();
        elf.chars().for_each(|item| {
            set.insert(item);
        });

        sets.append(&mut vec![set]);
    }
    let mut merged_set = sets.pop()?;
    while let Some(next_set) = sets.pop() {
        merged_set = merged_set.intersection(&next_set).map(|s| *s).collect();
    }
    merged_set.iter().next().copied()
}

fn main() {
    let filepath = Path::new("./input.txt");
    let content = fs::read_to_string(filepath).expect("Couldn't read input.txt");
    // first part of puzzle find all the duplicates and sum their priorities
    rummage_thru_backpacks(&content);

    // second part: figure out what item each group has in common (group badge), a group consists of three lines
    let mut elves_with_backpacks: Vec<&str> = vec![];
    content
        .trim_end_matches("\n")
        .split("\n")
        .for_each(|elf_with_pack| elves_with_backpacks.append(&mut vec![elf_with_pack]));

    let priorities = build_hash_map();
    let mut badge_priorities: u32 = 0;
    let groups: Vec<&[&str]> = elves_with_backpacks.chunks(3).collect();
    for group in groups {
        let shared_badge = inspect_elf_group(group);
        println!("Designated Group {}", shared_badge.unwrap());
        badge_priorities += priorities.get(&shared_badge.unwrap()).unwrap();
    }
    println!("Sum of Badge Priorities: {}", badge_priorities)
}
