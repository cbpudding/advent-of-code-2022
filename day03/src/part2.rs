#![feature(iter_array_chunks)]

use std::{fs::File, io::{BufRead, BufReader}};

fn get_priority(kind: char) -> usize {
    match kind {
        'a'..='z' => kind as usize - 96,
        'A'..='Z' => kind as usize - 38,
        _ => 0
    }
}

fn main() {
    // Read the rucksack manifest
    let input = File::open("data.txt").unwrap();
    let reader = BufReader::new(input);
    let lines = reader.lines();
    // Locate the badge for each group of three
    let mut badges = Vec::new();
    for [first, second, third] in lines.array_chunks() {
        match (first, second, third) {
            (Ok(first), Ok(second), Ok(third)) => {
                'main: {
                    for x in first.chars() {
                        for y in second.chars() {
                            if x == y {
                                for z in third.chars() {
                                    if y == z {
                                        badges.push(z);
                                        break 'main;
                                    }
                                }
                            }
                        }
                    }
                    panic!("Failed to find badge for group!\n");
                }
            }
            e => panic!("{e:?}")
        }
    }
    let total: usize = badges.iter().map(|b| get_priority(*b)).sum();
    println!("The sum of the priorities found is {total}"); // Is this even grammatically correct?
}