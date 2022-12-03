use std::{fs::File, io::{BufRead, BufReader}};

fn main() {
    // Read our input from the file
    let input = File::open("data.txt").unwrap();
    let reader = BufReader::new(input);
    let raw: Vec<Vec<usize>> = reader.lines()
        .fold(vec![vec![]], |mut data, cal| match cal {
            Ok(c) => if c.is_empty() {
                data.push(vec![]);
                data
            } else {
                let last = data.len() - 1;
                data[last].push(c.parse().unwrap());
                data
            },
            Err(e) => panic!("{e:?}")
        });
    // Locate three elves with the most calories and figure out how many calories they have
    let mut summed: Vec<usize> = raw.iter()
        .map(|elf| elf.iter().sum())
        .collect();
    summed.sort_by(|a, b| b.cmp(a));
    let (first, second, third) = (summed[0], summed[1], summed[2]);
    let total: usize = summed[0..=2].iter().sum();
    println!("The top three elves have {first} cal, {second} cal, and {third} cal giving a total of {total} cal.");
}