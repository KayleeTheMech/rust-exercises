use std::{fs, path::Path};

#[derive(Clone, Copy)]
struct Range {
    start: u8,
    end: u8,
}

impl Range {
    fn parse(range_string: &str) -> Option<Self> {
        let mut ranges: Vec<&str> = range_string.split("-").collect();
        let end = ranges.pop()?.parse::<u8>().unwrap();
        let start = ranges.pop()?.parse::<u8>().unwrap();
        Some(Self { start, end })
    }

    fn contains(self, other: &Range) -> bool {
        self.start <= other.start && self.end >= other.end
    }

    fn overlaps(self, other: &Range) -> bool {
        let condition1 = self.contains(other);
        let condition2 = other.contains(&self);
        let condition3 = self.start <= other.start && self.end >= other.start;
        let condition4 = self.start >= other.start && self.start <= other.end;
        condition1 || condition2 || condition3 || condition4
    }
}

#[test]
fn test_contains() {
    let range1 = Range::parse("1-3").unwrap();
    let range2 = Range::parse("2-3").unwrap();
    assert!(range1.contains(&range2));
    let range1 = Range::parse("1-3").unwrap();
    let range2 = Range::parse("4-5").unwrap();
    assert!(!range1.contains(&range2));
    let range1 = Range::parse("4-6").unwrap();
    let range2 = Range::parse("2-3").unwrap();
    assert!(!range1.contains(&range2));
    let range1 = Range::parse("1-3").unwrap();
    let range2 = Range::parse("2-4").unwrap();
    assert!(!range1.contains(&range2));
    let range1 = Range::parse("2-4").unwrap();
    let range2 = Range::parse("1-3").unwrap();
    assert!(!range1.contains(&range2));
}

#[test]
fn test_overlaps() {
    let range1 = Range::parse("1-3").unwrap();
    let range2 = Range::parse("2-3").unwrap();
    assert!(range1.overlaps(&range2));
    let range1 = Range::parse("1-3").unwrap();
    let range2 = Range::parse("4-5").unwrap();
    assert!(!range1.overlaps(&range2));
    let range1 = Range::parse("4-6").unwrap();
    let range2 = Range::parse("2-3").unwrap();
    assert!(!range1.overlaps(&range2));
    let range1 = Range::parse("1-3").unwrap();
    let range2 = Range::parse("2-4").unwrap();
    assert!(range1.overlaps(&range2));
    let range1 = Range::parse("2-4").unwrap();
    let range2 = Range::parse("1-3").unwrap();
    assert!(range1.overlaps(&range2));
}

fn main() {
    let filepath = Path::new("./input.txt");
    let content = fs::read_to_string(filepath).expect("Couldn't read input.txt");
    let mut contained_assignments: u32 = 0;
    let mut total_overlaps: u32 = 0;
    let mut lines: u32 = 0;
    for line in content.trim_end_matches("\n").split("\n") {
        let mut ranges: Vec<&str> = line.split(",").collect();
        lines += 1;
        let second = Range::parse(ranges.pop().unwrap()).unwrap();
        let first = Range::parse(ranges.pop().unwrap()).unwrap();
        if first.contains(&second) || second.contains(&first) {
            contained_assignments += 1;
        }

        if first.overlaps(&second) {
            total_overlaps += 1;
        }
    }
    println!("Contained assignments: {}", contained_assignments);
    println!("Total overlaps: {}", total_overlaps);
}
