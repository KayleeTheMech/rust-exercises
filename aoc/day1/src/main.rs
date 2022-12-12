use std::{fs, path::Path};

fn main() {
    let filepath = Path::new("./input.txt");
    let content = fs::read_to_string(filepath).expect("Couldn't read input.txt");

    let mut elves: Vec<u64> = vec![];
    for elf_string in content.split("\n\n") {
        let mut elf: u64 = 0;
        elf_string
            .split("\n")
            .for_each(|calory_string| match calory_string.parse::<u64>() {
                Ok(calory) => elf = elf + calory,
                Err(err) => println!("Parse error: {}", err),
            });
        elves.append(&mut vec![elf])
    }
    elves.sort();
    elves.reverse();
    println!("{:?}", elves.first());
    let mut biggest_three: u64 = 0;
    for elf in &elves[0..3] {
        println!("Individual:{:?}",elf);
        biggest_three += elf;
    }
    println!("Sum: {:?}", biggest_three);
}
