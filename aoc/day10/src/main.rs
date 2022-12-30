use std::collections::VecDeque;
use std::{fs, path::Path};

#[derive(PartialEq)]
enum Mmemomic {
    Noop,
    Addx,
}
struct Instruction {
    m: Mmemomic,
    op1: i16,
}

impl Instruction {
    fn parse(input: &str) -> Instruction {
        let mut command_iter = input.split(" ").filter(|s| !s.is_empty());
        let m = match command_iter.next() {
            Some("addx") => Mmemomic::Addx,
            Some("noop") => Mmemomic::Noop,
            _ => panic!("Unexpected instruction, program halt."),
        };
        let op1: i16 = match m {
            Mmemomic::Noop => 0,
            Mmemomic::Addx => command_iter
                .next()
                .expect("Expect an operand")
                .parse::<i16>()
                .ok()
                .expect("Expect operand to be parseable as i16"),
        };

        Self { m, op1 }
    }
}

struct RadioCPU {
    program: VecDeque<Instruction>,
    counter: u32,
    op: Option<Instruction>,
    op_count: u16,
    x: i16,
}

impl RadioCPU {
    fn new(program: VecDeque<Instruction>) -> Self {
        Self {
            counter: 0,
            x: 1,
            program,
            op_count: 0,
            op: None,
        }
    }

    fn tick(&mut self) {
        self.counter += 1;
        self.op_count += 1;
    }

    fn load(&mut self) {
        self.op = self.program.pop_front();
    }

    fn op_status(&self) -> bool {
        let op = match &self.op {
            Some(op) => op,
            _ => return true,
        };
        let finished = match op.m {
            Mmemomic::Noop => self.op_count == 1,
            Mmemomic::Addx => self.op_count == 2,
        };
        finished
    }

    fn finish_op(&mut self) {
        let op = match &self.op {
            Some(op) => op,
            _ => return,
        };
        match op.m {
            Mmemomic::Noop => {}
            Mmemomic::Addx => self.x += op.op1,
        }
    }

    fn halt(&self) -> bool {
        match self.program.front() {
            Some(_) => false,
            None => true,
        }
    }

    fn reset_op_count(&mut self) {
        self.op_count = 0;
    }
}

#[test]
fn test_program() {
    let snapshots: Vec<u32> = vec![3];
    let program = load_program("noop\naddx 3\naddx -5".to_string());
    assert_eq!(3, run_with_linebreaks_at(program, &snapshots));

    let program = load_program("addx 1\nnoop\naddx 2\naddx 5\naddx 2\nnoop".to_string());
    let snapshots: Vec<u32> = vec![2, 5];
    assert_eq!(12, run_with_linebreaks_at(program, &snapshots));

    let snapshots: Vec<u32> = vec![3, 6];
    let program = load_program("addx 1\nnoop\naddx 2\naddx 5\naddx 2\nnoop".to_string());
    assert_eq!(30, run_with_linebreaks_at(program, &snapshots));
}

fn load_program(content: String) -> VecDeque<Instruction> {
    let mut program: VecDeque<Instruction> = VecDeque::new();
    for line in content.trim_end_matches("\n").split("\n") {
        program.push_back(Instruction::parse(line))
    }
    program
}

fn sprite_in_range(pixel: i16, register_val: i16) -> bool {
    pixel == register_val || pixel == register_val + 1 || pixel == register_val + 2
}

fn run_with_linebreaks_at(program: VecDeque<Instruction>, linebreaks: &Vec<u32>) -> i32 {
    let mut cpu = RadioCPU::new(program);
    let mut sum: i32 = 0;
    loop {
        // for first cycle get first instruction
        match cpu.op {
            Some(_) => {}
            None => cpu.load(),
        }
        // start cycle
        cpu.tick();
        if sprite_in_range((cpu.counter % 40) as i16, cpu.x) {
            print!("#");
        } else {
            print!(".");
        }

        if linebreaks.contains(&cpu.counter) {
            sum += cpu.counter as i32 * cpu.x as i32; // collect snapshots for aoc-10
            println!("");
        }

        if cpu.op_status() {
            cpu.finish_op(); // operation is done, execute on register
        }

        // end cycle
        if cpu.halt() {
            break;
        }
        if cpu.op_status() {
            cpu.load(); // loads next instruction
            cpu.reset_op_count();
        }
    }
    sum
}

fn main() {
    let filepath = Path::new("./input.txt");
    let content = fs::read_to_string(filepath).expect("Couldn't read input.txt");

    let snapshots: Vec<u32> = vec![40, 80, 120, 160, 200, 240];
    println!(
        "Result: {}",
        run_with_linebreaks_at(load_program(content), &snapshots)
    )
}
