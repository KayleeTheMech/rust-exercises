fn constant_str() -> char {
    const HI: &str = "Hello, World!";
    HI.chars().next().unwrap()
}

fn value_str() -> char {
    let hi: &str = "Hello, World!";
    hi.chars().next().unwrap()
}

fn variable_str() -> char {
    let mut hi: &str = "Hello, World!";
    hi.chars().next().unwrap()
}

fn external_string(hi: &str) -> char {
    hi.chars().next().unwrap()
}

fn main() {
    const separator: &str = "-----------------";
    println!("{}", separator);
    println!("{}", constant_str());
    println!("{}", separator);
    println!("{}", value_str());
    println!("{}", separator);
    println!("{}", variable_str());
    println!("{}", separator);
    println!("{}", external_string("Hello, World!"));
    println!("{}", separator);
}
