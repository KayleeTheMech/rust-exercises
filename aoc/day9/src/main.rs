use std::collections::HashMap;
use std::{fmt, fs, path::Path};

#[derive(Clone, Copy, PartialEq, Debug)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

impl Direction {
    fn parse(c: char) -> Option<Direction> {
        match c {
            'U' => Some(Direction::UP),
            'D' => Some(Direction::DOWN),
            'L' => Some(Direction::LEFT),
            'R' => Some(Direction::RIGHT),
            _ => None,
        }
    }
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let c = match self {
            Direction::UP => 'U',
            Direction::DOWN => 'D',
            Direction::LEFT => 'L',
            Direction::RIGHT => 'R',
        };
        write!(f, "{}", c)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Instruction {
    direction: Direction,
    steps: u16,
}

impl Instruction {
    fn new(direction: Direction, steps: u16) -> Self {
        Self { direction, steps }
    }

    fn parse(input: &str) -> Option<Instruction> {
        let mut inp_iter = input.split(" ").filter(|s| !s.is_empty());
        let direction = Direction::parse(inp_iter.next()?.chars().next()?)?;
        let steps = match inp_iter.next()?.parse::<u16>() {
            Ok(val) => val,
            Err(err) => return None,
        };
        Some(Self::new(direction, steps))
    }
}

#[derive(Clone, Copy)]
struct Position {
    x: i16,
    y: i16,
}

impl Position {
    fn diff(&self, other: Position) -> Position {
        Position {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }

    fn abs(&self) -> i16 {
        if self.x == 0 && self.y == 0 {
            0
        } else if self.x.abs() <= 1 && self.y.abs() <= 1 {
            1
        } else {
            self.x.abs() + self.y.abs()
        }
    }
    fn to_tuple(&self) -> (i16, i16) {
        (self.x, self.y)
    }
}

#[test]
fn test_abs_position() {
    assert_eq!(Position { x: 0, y: 0 }.abs(), 0);
    assert_eq!(Position { x: 1, y: 1 }.abs(), 1);
    assert_eq!(Position { x: 3, y: 1 }.abs(), 4);
    assert_eq!(Position { x: 1, y: 0 }.abs(), 1);
    assert_eq!(Position { x: -1, y: 1 }.abs(), 1);
    assert_eq!(Position { x: -2, y: 1 }.abs(), 3);
}

#[derive(Clone, Copy)]
struct RopeSegment {
    head: Position,
    tail: Position,
}

impl RopeSegment {
    fn init() -> Self {
        Self {
            head: Position { x: 0, y: 0 },
            tail: Position { x: 0, y: 0 },
        }
    }

    fn move_head(&mut self, direction: Direction) {
        match direction {
            Direction::UP => self.head.y += 1,
            Direction::DOWN => self.head.y -= 1,
            Direction::LEFT => self.head.x -= 1,
            Direction::RIGHT => self.head.x += 1,
        }
    }

    fn physics_step(&mut self) {
        // vector pointing from tail to head
        let diff = self.head.diff(self.tail);
        let abstest = diff.abs();
        if diff.abs() >= 2 {
            if diff.x != 0 {
                self.tail.x += diff.x / diff.x.abs();
            }
            if diff.y != 0 {
                self.tail.y += diff.y / diff.y.abs();
            }
        }
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Direction({}, {})", self.direction, self.steps)
    }
}

#[test]
fn test_get_instructions() {
    assert_eq!(
        Instruction::parse("D 4523").expect("Expect this to parse correctly"),
        Instruction::new(Direction::DOWN, 4523)
    );
}

enum KnotType {
    KnotType(Box<Knot>),
    Nil,
}

struct Knot {
    rope: RopeSegment,
    next: KnotType,
}

impl Knot {
    fn new(rope: RopeSegment) -> Self {
        Self {
            rope: rope,
            next: KnotType::Nil,
        }
    }

    fn tie_next_rope_to_end(&mut self, rope: RopeSegment) {
        match self.next {
            KnotType::KnotType(ref mut next_address) => next_address.tie_next_rope_to_end(rope),
            KnotType::Nil => {
                self.next = KnotType::KnotType(Box::new(Knot {
                    rope: rope,
                    next: KnotType::Nil,
                }))
            }
        }
    }

    fn move_head(&mut self, direction: Direction) {
        self.rope.move_head(direction);
        self.physics_step();
    }

    fn physics_step(&mut self) {
        self.rope.physics_step();
        match self.next {
            KnotType::KnotType(ref mut next_knot) => {
                next_knot.rope.head = self.rope.tail;
                next_knot.physics_step();
            }
            KnotType::Nil => {}
        }
    }

    fn process_instruction(
        &mut self,
        instruction: Instruction,
        map: &mut HashMap<(i16, i16), u16>,
    ) {
        for _ in 0..instruction.steps {
            self.move_head(instruction.direction);

            map.entry(self.get_tail().tail.to_tuple())
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }
    }

    fn get_tail(&self) -> RopeSegment {
        match self.next {
            KnotType::KnotType(ref next_knot) => next_knot.get_tail(),
            KnotType::Nil => self.rope,
        }
    }
}

#[test]
fn test_rope() {
    let mut knot = Knot::new(RopeSegment::init());
    let mut map: HashMap<(i16, i16), u16> = HashMap::new();
    knot.process_instruction(Instruction::parse("L 2").unwrap(), &mut map);
    assert_eq!(knot.rope.head.to_tuple(), (-2, 0));
    assert_eq!(knot.rope.tail.to_tuple(), (-1, 0));
    knot.process_instruction(Instruction::parse("U 1").unwrap(), &mut map);
    assert_eq!(knot.rope.head.to_tuple(), (-2, 1));
    assert_eq!(knot.rope.tail.to_tuple(), (-1, 0));
    knot.process_instruction(Instruction::parse("U 1").unwrap(), &mut map);
    assert_eq!(knot.rope.head.to_tuple(), (-2, 2));
    assert_eq!(knot.rope.tail.to_tuple(), (-2, 1));
    knot.process_instruction(Instruction::parse("R 4").unwrap(), &mut map);
    assert_eq!(knot.rope.head.to_tuple(), (2, 2));
    assert_eq!(knot.rope.tail.to_tuple(), (1, 2));

    let mut knot = Knot::new(RopeSegment::init());
    for i in 1..9 {
        knot.tie_next_rope_to_end(RopeSegment::init());
    }

    knot.process_instruction(Instruction::parse("R 5").unwrap(), &mut map);
    knot.process_instruction(Instruction::parse("U 8").unwrap(), &mut map);
    assert_eq!(knot.get_tail().tail.to_tuple(), (0, 0));
    knot.process_instruction(Instruction::parse("L 8").unwrap(), &mut map);
    assert_eq!(knot.get_tail().tail.to_tuple(), (1, 3));
}

fn main() {
    let filepath = Path::new("./input.txt");
    let content = fs::read_to_string(filepath).expect("Couldn't read input.txt");

    let mut map: HashMap<(i16, i16), u16> = HashMap::new();
    let mut knot = Knot::new(RopeSegment::init());

    for i in 1..9 {
        println!("Tying knot {}", i);
        knot.tie_next_rope_to_end(RopeSegment::init())
    }
    map.entry(knot.get_tail().tail.to_tuple())
        .and_modify(|count| *count += 1)
        .or_insert(1);

    for input in content.trim_end_matches("\n").split("\n") {
        let instruction = Instruction::parse(input).expect("Should be parseable!");
        knot.process_instruction(instruction, &mut map)
    }
    println!("Number of positions visited: {}", map.keys().count());
}
