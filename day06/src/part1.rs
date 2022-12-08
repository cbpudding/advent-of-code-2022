use std::{collections::VecDeque, fs};

fn is_unique(list: &VecDeque<char>) -> bool {
    for x in 0..(list.len() - 1) {
        for y in (x + 1)..list.len() {
            if list[x] == list[y] {
                return false;
            }
        }
    }
    true
}

fn main() {
    let mut buffer = VecDeque::new();
    // This probably isn't an optimization problem... right?
    let input = fs::read_to_string("data.txt").unwrap();
    for (i, c) in input.char_indices() {
        buffer.push_back(c);
        if buffer.len() > 3 {
            if is_unique(&buffer) {
                println!("Start of packet marker detected at character {}", i + 1);
                break;
            }
            buffer.pop_front();
        }
    }
}
