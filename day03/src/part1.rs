use std::{fs::File, io::{BufRead, BufReader}};

fn get_priority(kind: char) -> usize {
    match kind {
        'a'..='z' => kind as usize - 96,
        'A'..='Z' => kind as usize - 38,
        _ => 0
    }
}

fn main() {
    // Read the rucksack manifest and find misplaced items
    let input = File::open("data.txt").unwrap();
    let reader = BufReader::new(input);
    let total: usize = reader.lines()
        .map(|line| match line {
            Ok(l) => {
                let mut priority = 0;
                let half = l.len() / 2;
                let (first, second) = (&l[0..half], &l[half..]);
                'main: for x in first.chars() {
                    for y in second.chars() {
                        if x == y {
                            priority += get_priority(x);
                            break 'main; // We can only match one duplicate for now
                        }
                    }
                }
                priority
            },
            Err(e) => panic!("{e:?}")
        })
        .sum();
    println!("The sum of the priorities found is {total}"); // Is this even grammatically correct?
}