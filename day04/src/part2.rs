use std::{fs::File, io::{BufRead, BufReader}, ops::RangeInclusive};

fn main() {
    // Read the current assignments for the pairs
    let input = File::open("data.txt").unwrap();
    let reader = BufReader::new(input);
    let raw: Vec<(RangeInclusive<usize>, RangeInclusive<usize>)> = reader.lines()
        .map(|line| match line {
            Ok(l) => {
                let mut split = l.split(",");
                let left_str = split.next().unwrap();
                let right_str = split.next().unwrap();
                let mut left_split = left_str.split("-");
                let mut right_split = right_str.split("-");
                let left = RangeInclusive::new(
                    left_split.next().unwrap().parse::<usize>().unwrap(),
                    left_split.next().unwrap().parse::<usize>().unwrap()
                );
                let right = RangeInclusive::new(
                    right_split.next().unwrap().parse::<usize>().unwrap(),
                    right_split.next().unwrap().parse::<usize>().unwrap()
                );
                (left, right)
            },
            Err(e) => panic!("{e:?}")
        })
        .collect();
    // Look for any ranges that completely encapsulate another
    let total: usize = raw.iter()
        .filter(|pair| if pair.0.contains(pair.1.start()) || pair.0.contains(pair.1.end()) {
            true
        } else if pair.1.contains(pair.0.start()) || pair.1.contains(pair.0.end()) {
            true
        } else {
            false
        })
        .count();
    println!("A total of {total} complete overlaps discovered!");
}