use std::{fs, path::Path, vec};
#[derive(Clone, Copy)]
struct Instruction {
    amount: u8,
    from: u8,
    to: u8,
}

impl Instruction {
    fn parse(input: &str) -> Option<Self> {
        let mut input = input.split(" ");
        let mut amount: Option<u8> = None;
        let mut from: Option<u8> = None;
        let mut to: Option<u8> = None;
        while let Some(command) = input.next() {
            match command {
                "move" => amount = Some(input.next()?.parse::<u8>().unwrap()),
                "from" => from = Some(input.next()?.parse::<u8>().unwrap() - 1),
                "to" => to = Some(input.next()?.parse::<u8>().unwrap() - 1),
                _ => return None,
            }
        }
        match (amount, from, to) {
            (Some(amount), Some(from), Some(to)) => Some(Self { amount, from, to }),
            _ => None,
        }
    }
}

#[test]
fn test_case() {
    let instruction = Instruction::parse("move 12 from 5 to 4").unwrap();
    assert_eq!(instruction.amount, 12 as u8);
    assert_eq!(instruction.from, 5 as u8);
    assert_eq!(instruction.to, 4 as u8);
}

// first LINES_FOR_STACK_STATE describe stack at the beginning of operation
const LINES_FOR_STACK_STATE: usize = 8;
const NUMBER_OF_STACKS: u8 = 8;

fn get_fresh_stacks(content: &str) -> Vec<Vec<char>> {
    let mut stacks: Vec<Vec<char>> = vec![];
    for _ in 0..(NUMBER_OF_STACKS + 1) as u8 {
        let stack: Vec<char> = vec![];
        stacks.push(stack)
    }
    fill_stacks_from_content(&mut stacks, &content);
    stacks
}

fn fill_stacks_from_content(stacks: &mut Vec<Vec<char>>, content: &str) {
    let mut lines = content.trim_end_matches("\n").split("\n");
    let mut pre_stack: Vec<&str> = vec![];
    for i in 0..LINES_FOR_STACK_STATE {
        pre_stack.push(lines.next().unwrap())
    }
    for i in 0..LINES_FOR_STACK_STATE {
        let line: Vec<char> = pre_stack.pop().unwrap().chars().collect();
        for j in 0..NUMBER_OF_STACKS + 1 {
            let item = line.get(1 + 4 * j as usize).unwrap();
            match item {
                ' ' => {}
                _ => {
                    let mut stack = stacks.get_mut(j as usize).unwrap();
                    stack.push(*item);
                }
            }
        }
    }
    println!("Read {} lines from file for state.", LINES_FOR_STACK_STATE);
}

fn execute_instruction_on_cratemover9000(stacks: &mut Vec<Vec<char>>, instruction: Instruction) {
    for _ in 0..instruction.amount {
        let stack = stacks
            .get_mut(instruction.from as usize)
            .expect("Instruction wrong, from stack doesn't exist!");
        let item = stack
            .pop()
            .expect("Expect something on the stack when instruction says so!");
        let stack = stacks
            .get_mut(instruction.to as usize)
            .expect("Instruction wrong, to stack doesn't exist!");
        stack.push(item);
    }
}

fn execute_instruction_on_cratemover9001(stacks: &mut Vec<Vec<char>>, instruction: Instruction) {
    let stack = stacks
        .get_mut(instruction.from as usize)
        .expect("Instruction wrong, from stack doesn't exist!");
    let mut items: Vec<char> = stack
        .split_off(stack.len() - instruction.amount as usize);
    let stack = stacks
        .get_mut(instruction.to as usize)
        .expect("Instruction wrong, to stack doesn't exist!");
    stack.append(&mut items);
}

fn print_secret_message(stacks: &Vec<Vec<char>>) {
    for stack in stacks {
        print!("{}", stack.last().expect("Expect one item on each"))
    }
}

fn main() {
    let filepath = Path::new("./input.txt");
    let content = fs::read_to_string(filepath).expect("Couldn't read input.txt");
    let mut lines: u32 = 0;
    let mut stacks: Vec<Vec<char>> = get_fresh_stacks(&content);
    // Execute instructions with CrateMover 9000
    for line in content
        .trim_end_matches("\n")
        .split("\n")
        .skip(LINES_FOR_STACK_STATE + 2)
    {
        let instruction = Instruction::parse(line).expect("Expect Instruction to be readable.");
        execute_instruction_on_cratemover9000(&mut stacks, instruction);
        lines += 1;
    }
    print!("Result on CrateMover 9000: ");
    print_secret_message(&stacks);
    print!("\n");

    let mut stacks: Vec<Vec<char>> = get_fresh_stacks(&content);
    // Execute instructions with CrateMover 9001
    for line in content
        .trim_end_matches("\n")
        .split("\n")
        .skip(LINES_FOR_STACK_STATE + 2)
    {
        let instruction = Instruction::parse(line).expect("Expect Instruction to be readable.");
        execute_instruction_on_cratemover9001(&mut stacks, instruction);
        lines += 1;
    }
    print!("Result on CrateMover 9001: ");
    print_secret_message(&stacks);
    print!("\n");
    println!("Total instructions processed: {}", lines);
}
