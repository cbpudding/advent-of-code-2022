use nom::{
    bytes::complete::{tag, take, take_till},
    character::{
        complete::{anychar, digit1, newline},
        is_digit,
    },
    combinator::peek,
    multi::{many0, separated_list0},
    sequence::delimited,
    IResult,
};
use std::{collections::VecDeque, fs::File, io::Read};

#[derive(Debug)]
struct CraneGame {
    crates: Vec<VecDeque<char>>,
    instructions: Vec<Instruction>,
}

impl CraneGame {
    fn solve(&mut self) -> String {
        for instruction in &self.instructions {
            // I tried to do this the smart way...
            let mut stack = VecDeque::new();
            // O(n) go brrrrrrrrrrrrrrrr
            for _ in 0..instruction.quantity {
                stack.push_front(self.crates[instruction.from - 1].pop_back().unwrap());
            }
            for c in &stack {
                self.crates[instruction.to - 1].push_back(*c);
            }
        }
        self.crates
            .iter()
            .map(|stack| stack.iter().last())
            .fold(String::new(), |mut s, c| {
                s.push(*c.unwrap());
                s
            })
    }
}

#[derive(Debug, PartialEq)]
struct Instruction {
    from: usize,
    quantity: usize,
    to: usize,
}

fn parse_crate(input: &str) -> IResult<&str, Option<char>> {
    let (input, next) = peek(anychar)(input)?;
    if next != '[' {
        let (input, _) = take(3usize)(input)?;
        Ok((input, None))
    } else {
        let (input, id) = delimited(tag("["), anychar, tag("]"))(input)?;
        Ok((input, Some(id)))
    }
}

fn parse_crate_row(input: &str) -> IResult<&str, Vec<Option<char>>> {
    let (input, row) = take_till(|c| c == '\n')(input)?;
    let (_, row) = separated_list0(tag(" "), parse_crate)(row)?;
    let (input, _) = newline(input)?;
    Ok((input, row))
}

fn parse_crates(input: &str) -> IResult<&str, Vec<VecDeque<char>>> {
    let mut rows = Vec::new();
    let (mut input, mut next) = peek(take(3usize))(input)?;
    while !is_digit(next.chars().nth(1).unwrap() as u8) {
        let (inner, row) = parse_crate_row(input)?;
        rows.push(row);
        (input, next) = peek(take(3usize))(inner)?;
    }
    let (input, _) = take_till(|c| c == '\n')(input)?;
    let (input, _) = newline(input)?;
    let mut crates = Vec::new();
    for x in 0..rows[0].len() {
        let mut column = VecDeque::new();
        for y in (0..rows.len()).rev() {
            // Floating containers aren't possible right?
            if let Some(v) = rows[y][x] {
                column.push_back(v);
            }
        }
        crates.push(column);
    }
    Ok((input, crates))
}

// My first parser written with nom :D
fn parse_instruction(input: &str) -> IResult<&str, Instruction> {
    let (input, _) = tag("move ")(input)?;
    let (input, quantity) = digit1(input)?;
    let (input, _) = tag(" from ")(input)?;
    let (input, from) = digit1(input)?;
    let (input, _) = tag(" to ")(input)?;
    let (input, to) = digit1(input)?;
    let (input, _) = newline(input)?;
    Ok((
        input,
        Instruction {
            from: usize::from_str_radix(from, 10).unwrap(),
            quantity: usize::from_str_radix(quantity, 10).unwrap(),
            to: usize::from_str_radix(to, 10).unwrap(),
        },
    ))
}

fn parse_instructions(input: &str) -> IResult<&str, Vec<Instruction>> {
    many0(parse_instruction)(input)
}

fn parse_input(input: &str) -> IResult<&str, CraneGame> {
    let (input, crates) = parse_crates(input)?;
    let (input, _) = newline(input)?;
    let (input, instructions) = parse_instructions(input)?;
    Ok((
        input,
        CraneGame {
            crates,
            instructions,
        },
    ))
}

fn main() {
    // Read the current crate order and instructions
    let mut input = File::open("data.txt").unwrap();
    let mut buffer = String::new();
    input.read_to_string(&mut buffer).unwrap();
    let (_, mut raw) = parse_input(&buffer).unwrap();
    // Move some crates around and see what's on top!
    let solution = raw.solve();
    println!("The crates on the very top of each stack spell out \"{solution}\".")
}

#[cfg(test)]
mod test {
    #[test]
    fn parse_crate() {
        assert_eq!(super::parse_crate("[H]").unwrap().1, Some('H'));
        assert_eq!(super::parse_crate("   ").unwrap().1, None);
    }

    #[test]
    fn parse_crate_row() {
        assert_eq!(
            super::parse_crate_row("[T] [E] [S] [T]").unwrap().1,
            vec![Some('T'), Some('E'), Some('S'), Some('T')]
        );
        assert_eq!(
            super::parse_crate_row("[O] [O]     [P] [S]").unwrap().1,
            vec![Some('O'), Some('O'), None, Some('P'), Some('S')]
        )
    }

    #[test]
    fn parse_instruction() {
        assert_eq!(
            super::parse_instruction("move 1 from 3 to 9\n").unwrap().1,
            super::Instruction {
                from: 3,
                quantity: 1,
                to: 9
            }
        );
    }

    #[test]
    fn parse_instructions() {
        assert_eq!(
            super::parse_instructions(concat!(
                "move 2 from 2 to 1\n",
                "move 3 from 5 to 4\n",
                "move 1 from 1 to 8\n"
            ))
            .unwrap()
            .1,
            vec![
                super::Instruction {
                    from: 2,
                    quantity: 2,
                    to: 1
                },
                super::Instruction {
                    from: 5,
                    quantity: 3,
                    to: 4
                },
                super::Instruction {
                    from: 1,
                    quantity: 1,
                    to: 8
                }
            ]
        )
    }
}
