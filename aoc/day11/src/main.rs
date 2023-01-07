#[macro_use]
extern crate nom;
use nom::bytes::complete::take_until;
use nom::character::is_alphabetic;
use nom::{bytes::complete::tag, character::complete::digit0, error::ParseError, IResult};
use std::num::ParseIntError;
use std::{fs, path::Path};

struct Monkey {
    id: u32,
    starting_items: Vec<u32>,
    pub foo: Box<dyn Fn(u32) -> u32>,
    test_modulo: u32,
    test_fail_throw_to_monkey_id: u32,
    test_success_throw_to_monkey_id: u32,
}

struct Operation{
    op1: char,
    op2_opt: Option<u32>,
    closure: Box<dyn Fn(u32) -> u32>,
}

impl Operation {
    // private
    fn construct_operation(&mut self) {
        match self.op1 {
            '+' => {
                self.closure = Box::new(|x: u32| -> u32 {
                    x + match self.op2_opt {
                        None => x,
                        Some(number) => number,
                    }
                })
            }
            '*' => self.closure = Box::new(|x: u32| -> u32 { x }),
            _ => panic!(),
        }
    }

    fn parse(input: String) -> Option<Self> {
        let operands: Vec<&str> = input.trim().split(" ").collect();

        let op1 = match operands.get(1)? {
            &"+" => '+',
            &"*" => '*',
            _ => return None,
        };
        let op2_opt = match &operands.get(2) {
            Some(&"old") => None,
            Some(&other) => Some(other.parse::<u32>().unwrap()),
            _ => return None,
        };

        let mut result = Self {
            op1,
            op2_opt,
            closure: Box::new(|x| -> u32 { x }),
        };
        result.construct_operation();
        Some(result)
    }

    fn run(&self, x: u32) -> u32 {
        (self.closure)(x)
    }
}

#[test]
fn test_operation() {
    let op = Operation::parse("old + 2".to_string()).unwrap();
    assert_eq!(op.run(2), 4);
}

impl Monkey {
    fn new(
        id: u32,
        starting_items: Vec<u32>,
        foo: Box<dyn Fn(u32) -> u32>,
        test_modulo: u32,
        test_fail_throw_to_monkey_id: u32,
        test_success_throw_to_monkey_id: u32,
    ) -> Self {
        Self {
            id,
            starting_items,
            foo,
            test_modulo,
            test_fail_throw_to_monkey_id,
            test_success_throw_to_monkey_id,
        }
    }
}

fn parse_comma_separated_values(input: &str) -> Vec<u8> {
    let mut result: Vec<u8> = vec![];
    for str_value in input.split(",") {
        result.push(str_value.trim().parse::<u8>().expect("Not a u8 number"))
    }
    result
}

fn parse_monkey(input: &str) -> IResult<&str, &str> {
    // read monkey index
    let (input, _) = tag("Monkey ")(input)?;
    let (input, monkey_index) = digit0(input)?;
    let (input, _) = tag(":\n")(input)?;
    // read items in possession
    let (input, _) = take_until(":")(input)?;
    let (input, items_string) = take_until("\n")(input)?;
    let (items_string, _) = tag(": ")(items_string)?;
    let items = parse_comma_separated_values(items_string);
    // read operation
    let (input, _) = tag("Operation: new = ")(input.trim())?;
    let (input, op_string) = take_until("\n")(input)?;
    let (input, _) = tag("\n")(input)?;
    Ok((input, monkey_index))
}
#[test]
fn test_parser() {
    let string = "Monkey 0:
  Starting items: 52, 60, 85, 69, 75, 75
  Operation: new = old * 17
  Test: divisible by 13
    If true: throw to monkey 6
    If false: throw to monkey 7";
    let (input, monkey_index) = parse_monkey(string).expect("should be parseable");
}
fn main() {
    let filepath = Path::new("./input.txt");
    let content = fs::read_to_string(filepath).expect("Couldn't read input.txt");
    for block in content.trim_end_matches("\n").split("\n\n") {
        print!("{}", block);
    }
}
