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
    // Locate the elf with the most calories and figure out how many calories they have
    let highest = raw.iter()
        .map(|elf| elf.iter().sum())
        .reduce(|a: usize, b| if a > b {
            a
        } else {
            b
        })
        .unwrap();
    println!("The elf with the highest number of calories has: {highest} cal.");
}