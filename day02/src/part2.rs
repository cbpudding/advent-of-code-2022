use std::{fs::File, io::{BufRead, BufReader}};

enum RpsDecision {
    Rock,
    Paper,
    Scissors
}

enum RpsOutcome {
    Win,
    Loss,
    Draw
}

impl RpsDecision {
    fn strategize(&self, desired: &RpsOutcome) -> RpsDecision {
        match (self, desired) {
            (RpsDecision::Rock, RpsOutcome::Win) => RpsDecision::Paper,
            (RpsDecision::Rock, RpsOutcome::Loss) => RpsDecision::Scissors,
            (RpsDecision::Rock, RpsOutcome::Draw) => RpsDecision::Rock,
            (RpsDecision::Paper, RpsOutcome::Win) => RpsDecision::Scissors,
            (RpsDecision::Paper, RpsOutcome::Loss) => RpsDecision::Rock,
            (RpsDecision::Paper, RpsOutcome::Draw) => RpsDecision::Paper,
            (RpsDecision::Scissors, RpsOutcome::Win) => RpsDecision::Rock,
            (RpsDecision::Scissors, RpsOutcome::Loss) => RpsDecision::Paper,
            (RpsDecision::Scissors, RpsOutcome::Draw) => RpsDecision::Scissors
        }
    }

    fn value(&self) -> usize {
        match self {
            RpsDecision::Rock => 1,
            RpsDecision::Paper => 2,
            RpsDecision::Scissors => 3
        }
    }
}

impl From<char> for RpsDecision {
    fn from(c: char) -> Self {
        match c {
            'A' => Self::Rock,
            'B' => Self::Paper,
            'C' => Self::Scissors,
            _ => panic!("Unexpected character parsed by RpsDecision::from!")
        }
    }
}

impl RpsOutcome {
    fn value(&self) -> usize {
        match self {
            RpsOutcome::Win => 6,
            RpsOutcome::Loss => 0,
            RpsOutcome::Draw => 3
        }
    }
}

impl From<char> for RpsOutcome {
    fn from(c: char) -> Self {
        match c {
            'X' => Self::Loss,
            'Y' => Self::Draw,
            'Z' => Self::Win,
            _ => panic!("Unexpected character parsed by RpsOutcome::from!")
        }
    }
}

fn main() {
    // Read the strategy guide
    let input = File::open("data.txt").unwrap();
    let reader = BufReader::new(input);
    let raw: Vec<(RpsDecision, RpsOutcome)> = reader.lines()
        .map(|line| match line {
            Ok(l) => {
                let raw: Vec<char> = l.chars().collect();
                (RpsDecision::from(raw[0]), RpsOutcome::from(raw[2]))
            },
            Err(e) => panic!("{e:?}")
        })
        .collect();
    // Calculate the total score if everything goes according to plan
    let total: usize = raw.iter()
        .map(|round| round.1.value() + round.0.strategize(&round.1).value())
        .sum();
    println!("If the strategy guide works, I'll have a score of {total}.");
}