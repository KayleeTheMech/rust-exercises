// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let mut map: HashMap<&str, u16> = HashMap::new();
    for &word in magazine.iter() {
        map.entry(word).and_modify(|count| *count += 1).or_insert(1);
    }

    for &word in note {
        match map.get(word) {
            None => return false,
            Some(count) if *count == 0 => return false,
            Some(count) => {
                map.entry(word).and_modify(|count| *count -= 1);
            }
        }
    }
    true
}
