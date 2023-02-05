use either::Either;
use nom::{
    bytes::complete::{tag, take_till},
    character::complete::{anychar, digit1, newline},
    combinator::peek,
    multi::many0,
    IResult,
};
use std::{collections::HashMap, fs};

#[derive(Debug)]
enum Command {
    ChangeDirectory(String),
    List,
}

#[derive(Debug)]
enum Entry {
    Directory {
        name: String,
        children: HashMap<String, Entry>,
    },
    File {
        name: String,
        size: usize,
    },
}

impl Entry {
    pub fn directory<S: Into<String>>(name: S) -> Self {
        Self::Directory {
            name: name.into(),
            children: HashMap::new(),
        }
    }

    pub fn file<S: Into<String>>(name: S, size: usize) -> Self {
        Self::File {
            name: name.into(),
            size,
        }
    }

    pub fn name(&self) -> String {
        match self {
            Self::Directory { name, .. } => name.clone(),
            Self::File { name, .. } => name.clone(),
        }
    }

    pub fn size(&self) -> usize {
        match self {
            Entry::Directory { children, .. } => children.iter().map(|(_, e)| e.size()).sum(),
            Entry::File { size, .. } => *size,
        }
    }
}

fn navigate<'a>(root: &'a mut Entry, path: &Vec<String>) -> &'a mut Entry {
    let mut current = root;
    for dir in path {
        if let Entry::Directory { children, .. } = current {
            current = children.get_mut(dir).unwrap();
        }
    }
    current
}

fn parse_command(input: &str) -> IResult<&str, Command> {
    let (input, _) = tag("$ ")(input)?;
    let (_, next) = peek(anychar)(input)?;
    match next {
        'c' => {
            let (input, _) = tag("cd ")(input)?;
            let (input, target) = take_till(|c| c == '\n')(input)?;
            Ok((input, Command::ChangeDirectory(target.into())))
        }
        'l' => {
            let (input, _) = tag("ls")(input)?;
            Ok((input, Command::List))
        }
        _ => panic!("Unknown command"),
    }
}

fn parse_directory(input: &str) -> IResult<&str, Entry> {
    let (input, _) = tag("dir ")(input)?;
    let (input, name) = take_till(|c| c == '\n')(input)?;
    Ok((input, Entry::directory(name)))
}

fn parse_file(input: &str) -> IResult<&str, Entry> {
    let (input, size) = digit1(input)?;
    let (input, _) = tag(" ")(input)?;
    let (input, name) = take_till(|c| c == '\n')(input)?;
    Ok((
        input,
        Entry::file(name, usize::from_str_radix(size, 10).unwrap()),
    ))
}

fn parse_line(input: &str) -> IResult<&str, Either<Command, Entry>> {
    let (_, next) = peek(anychar)(input)?;
    let (input, output) = match next {
        '$' => {
            let (input, command) = parse_command(input)?;
            (input, Either::Left(command))
        }
        '0'..='9' => {
            let (input, entry) = parse_file(input)?;
            (input, Either::Right(entry))
        }
        'd' => {
            let (input, entry) = parse_directory(input)?;
            (input, Either::Right(entry))
        }
        _ => panic!("Unexpected character '{next}'"),
    };
    let (input, _) = newline(input)?;
    Ok((input, output))
}

fn parse_log(input: &str) -> IResult<&str, Entry> {
    let (input, raw) = many0(parse_line)(input)?;
    let mut root = Entry::directory("");
    let mut path = Vec::new();
    let mut current = &mut root;
    for line in raw {
        println!("{line:?}");
        match line {
            Either::Left(c) => match c {
                Command::ChangeDirectory(d) => match d.as_str() {
                    "/" => {
                        path.clear();
                        current = &mut root;
                    }
                    ".." => {
                        path.pop().unwrap();
                        current = navigate(&mut root, &path);
                    }
                    _ => {
                        path.push(d);
                        current = navigate(&mut root, &path);
                    }
                },
                Command::List => {
                    // I'm just going to ignore lists in this case. A proper
                    // parser would wait for the command before attempting to
                    // get filesystem data.
                }
            },
            Either::Right(e) => {
                if let Entry::Directory { children, .. } = current {
                    children.insert(e.name(), e);
                } else {
                    panic!("Attempted to open file as a directory")
                }
            }
        }
    }
    Ok((input, root))
}

fn part1_solution(entry: &Entry) -> usize {
    // ...
}

fn main() {
    // Read and parse the input data into a usable data structure
    let input = fs::read_to_string("data.log").unwrap();
    let (_, root) = parse_log(&input).unwrap();
    // Calculate the total bytes of the directories that meet the criteria
    let total = part1_solution(&root);
    println!("The total size of all directories that meet the criteria is {total} bytes");
}
