use std::{fs, path::Path, vec};

/// Grows the buffer with the symbol until it reaches buffer_size,
/// after that it removes the first symbol in the buffer to keep the buffer at buffer_size
fn add_to_buffer(buffer: &mut Vec<char>, symbol: char, buffer_size: usize) {
    buffer.push(symbol);
    if buffer.len() > buffer_size {
        buffer.remove(0);
    }
}

/// Figures out if the current buffer content contains a marker
///
/// # Panics
///
/// Panics if buffer is longer than buffer_size.
fn is_marker(buffer: &Vec<char>, buffer_size: usize) -> bool {
    if buffer.len() < buffer_size {
        return false;
    } else if buffer.len() != buffer_size {
        panic!("Buffer needs to be buffer_size large!");
    }

    // loop over all combinations, return false if two characters are the some
    for i in 0..buffer_size {
        let outer_symbol = buffer.get(i).unwrap();
        let mut inner_buffer = buffer.clone();
        inner_buffer.remove(i);
        for inner_symbol in inner_buffer {
            if inner_symbol == *outer_symbol {
                return false;
            }
        }
    }
    // by this point all combinations are unique -> we found a marker in the buffer
    true
}

/// Loops over the content to find the position of the first marker in it
fn get_first_marker(content: &str, buffer_size: usize) -> Option<u32> {
    let mut count: u32 = 0;
    let mut buffer: Vec<char> = vec![];
    for symbol in content.trim_end_matches("\n").chars() {
        add_to_buffer(&mut buffer, symbol, buffer_size);
        count += 1;
        if is_marker(&buffer, buffer_size) {
            return Some(count);
        }
    }
    None
}

fn get_first_packet_marker(content: &str) -> Option<u32> {
    const BUFFER_SIZE: usize = 4;
    get_first_marker(content, BUFFER_SIZE)
}

fn get_first_message_marker(content: &str) -> Option<u32> {
    const BUFFER_SIZE: usize = 14;
    get_first_marker(content, BUFFER_SIZE)
}

fn main() {
    let filepath = Path::new("./input.txt");
    let content = fs::read_to_string(filepath).expect("Couldn't read input.txt");
    println!(
        "First Packet Marker at: {}",
        get_first_packet_marker(&content).expect("Expect a packet marker to find in this string")
    );
    println!(
        "First Message Marker at: {}",
        get_first_message_marker(&content).expect("Expect a message marker to find in this string")
    );
}
