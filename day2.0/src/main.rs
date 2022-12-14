#[derive(Debug, PartialEq, Eq)]
pub enum Play {
    Rock,
    Paper,
    Scissors,
}

impl From<&Play> for u64 {
    fn from(input: &Play) -> Self {
        match input {
            Play::Rock => 1,
            Play::Paper => 2,
            Play::Scissors => 3,
        }
    }
}

impl From<&str> for Play {
    fn from(input: &str) -> Play {
        match input {
            "A" => Play::Rock,
            "B" => Play::Paper,
            "C" => Play::Scissors,
            "X" => Play::Rock,
            "Y" => Play::Paper,
            "Z" => Play::Scissors,
            _ => panic!(),
        }
    }
}

impl PartialOrd for Play {
    fn partial_cmp(&self, _other: &Self) -> Option<std::cmp::Ordering> {
        todo!()
    }

    fn gt(&self, other: &Self) -> bool {
        match &self {
            Play::Rock => matches!(other, Play::Scissors),
            Play::Paper => matches!(other, Play::Rock),
            Play::Scissors => matches!(other, Play::Paper),
        }
    }
}

impl Play {
    fn play(&self, other: &Self) -> u64 {
        if self == other {
            3
        } else if self > other {
            6
        } else {
            0
        }
    }
}

fn main() {
    // let strats = "A Y
    // B X
    // C Z";

    let strats = "C Y
    B Y
    C Y
    B X
    B X
    B X
    C Y
    B Z
    A Z
    A Z
    A Z
    A Y
    A X
    A Z
    A X
    A Z
    C Y
    A X
    A Z
    A Z
    A Z
    A Y
    C Y
    B Z
    B Z
    C Y
    C Y
    C Y
    C Y
    A X
    B Z
    C X
    B X
    A Y
    C X
    C Y
    C Y
    B X
    C X
    B Z
    C Y
    B X
    C Y
    B X
    B X
    C Y
    A X
    B X
    B Z
    B X
    A Z
    B Z
    B X
    B Y
    C Y
    A X
    C Y
    A Z
    C Y
    C Z
    B Z
    C Y
    C Y
    C Y
    B Y
    B X
    B X
    A X
    B Z
    B X
    B X
    B X
    B X
    B Y
    B X
    B X
    B X
    B X
    B Z
    B X
    B X
    B Y
    B X
    C Y
    C Y
    B Z
    A Z
    A X
    B X
    B X
    A X
    B X
    A X
    B X
    A Z
    B Y
    A X
    A Y
    C Y
    A X
    A X
    C Y
    A Z
    B X
    B Y
    C Z
    C Y
    B X
    B X
    A Z
    B X
    B X
    C Y
    A Z
    A Z
    B X
    C X
    B Z
    C Y
    B X
    A Y
    C Y
    B Z
    C Y
    B Z
    A X
    C Y
    C Z
    B X
    B X
    C X
    A X
    B X
    B Z
    B Z
    B Z
    A Y
    B Z
    B X
    C Y
    B Z
    A Z
    C Y
    C Y
    A Z
    C Y
    B X
    B X
    C Y
    B Z
    B Z
    C Y
    A X
    C Y
    C Y
    B X
    B Y
    C Y
    C Y
    A Z
    B Z
    B X
    B X
    C Z
    C Y
    A X
    B Z
    B Y
    C Y
    C Y
    B X
    B Z
    C X
    B X
    C Y
    B Z
    A X
    B X
    B Z
    A X
    C Y
    B X
    B X
    C Y
    A X
    B X
    B Y
    A X
    A Y
    C Y
    C Y
    B X
    A Z
    A X
    B Y
    A X
    A X
    C X
    C Y
    B Y
    A Z
    C X
    A X
    B Z
    B X
    B X
    C Y
    A X
    C X
    C Y
    A X
    C Y
    B X
    B X
    C Y
    C Y
    B X
    A X
    B X
    A X
    C Y
    B Y
    B Z
    B X
    B X
    C X
    C Y
    B Z
    C Y
    C Y
    B X
    A Z
    B Z
    A Z
    A Z
    C Y
    B X
    C Y
    C Y
    C X
    B X
    C X
    C Z
    C Y
    B Z
    C Z
    C Y
    A Z
    B X
    C X
    B Y
    B Z
    C Y
    B Z
    B Z
    B Z
    A Z
    C X
    A Y
    B Z
    A Z
    B Z
    B Y
    A Z
    C Y
    C Y
    B X
    A X
    B X
    C X
    B Y
    C Y
    B Z
    B X
    C Y
    A X
    B Z
    B Y
    C X
    C Y
    B X
    A Z
    A Z
    C Y
    B Z
    B Z
    B X
    B X
    C X
    B X
    B X
    B X
    A Z
    A X
    C Z
    B X
    A Z
    B X
    C X
    B X
    C X
    B Z
    B X
    B X
    A X
    B Z
    B Y
    A Z
    A Z
    C Y
    C Y
    A Z
    A Z
    B X
    A X
    B Z
    C X
    C Y
    C Y
    B Z
    A X
    B Y
    B Z
    A X
    C Y
    B Z
    A X
    B X
    B X
    C Y
    C Z
    A Y
    A X
    C Y
    A Z
    C X
    A Z
    A Z
    C X
    A X
    A Z
    B Z
    A Y
    A Z
    C Y
    B X
    B X
    B Z
    B X
    B X
    B Y
    B X
    B X
    C Y
    B X
    A Z
    B X
    C Y
    A Z
    B Z
    C Y
    C X
    C Y
    B X
    C Y
    B Y
    B X
    B Z
    C Y
    A X
    A X
    C X
    B X
    A X
    C X
    B Y
    B Y
    A Z
    C Z
    C Z
    A X
    C Y
    A Y
    A Z
    B Z
    A Z
    A X
    A Y
    B X
    B Y
    C Y
    A X
    B Y
    C X
    C Y
    B Z
    C Y
    A X
    B Z
    B Z
    A X
    C X
    B X
    A X
    B Z
    C Y
    C Y
    B Y
    A Y
    B X
    B Z
    B X
    C Y
    B X
    C Y
    C X
    A Z
    B Z
    C Y
    B X
    B Z
    B Z
    B Z
    C Y
    C X
    C X
    B X
    A X
    C Y
    C Y
    B X
    B Z
    C Y
    A X
    B X
    B Z
    B Z
    B Y
    A Y
    B X
    B Z
    B X
    A X
    A Z
    A Z
    C X
    B Y
    B Z
    C Z
    B X
    C Y
    B X
    C Y
    B Y
    B Z
    B Y
    B X
    A Z
    C X
    A X
    A Z
    C Y
    B X
    C Y
    C Y
    B X
    C Z
    B Z
    B Z
    C X
    C Y
    A Y
    C Y
    A Y
    B X
    B Y
    A Z
    B Z
    B X
    B X
    A X
    C Y
    B Z
    C Y
    B X
    A Z
    B Z
    C Y
    B Z
    A Z
    C Y
    B X
    B X
    C Y
    C Y
    C Y
    B X
    C Z
    B Z
    B X
    C Y
    C X
    B X
    C Y
    B Y
    B Z
    B Z
    B Y
    B Z
    B Y
    A Z
    B X
    B X
    A Z
    C Z
    A Z
    B X
    C Z
    A X
    B X
    C Y
    B Z
    A Z
    C Y
    C Y
    B Z
    A X
    C Y
    A Z
    B X
    B Z
    B Y
    B X
    B Z
    B X
    C Y
    A X
    B Z
    C Y
    B Z
    B Z
    C Y
    B X
    B X
    A Y
    C Y
    B Z
    B Z
    B X
    B Z
    A X
    A X
    C Y
    B X
    C Y
    A Z
    A X
    B X
    C Y
    B Z
    A Z
    B X
    A X
    B X
    B Y
    A X
    C Z
    A X
    B Y
    B X
    A Y
    C Y
    A Z
    B X
    C Y
    A X
    B X
    C Y
    A Z
    B Y
    A Z
    B Z
    B X
    A X
    B Y
    A X
    A Z
    B Z
    B X
    B Z
    C Y
    C Y
    C Y
    B X
    B Y
    A Z
    A Z
    B X
    B X
    B Y
    A X
    A Z
    C Y
    A Y
    B Z
    A X
    C X
    A Z
    C X
    C Y
    C Y
    B X
    B X
    B Z
    A X
    A Z
    B Y
    B Z
    B X
    B Z
    B X
    C Y
    B X
    C Y
    C Y
    B X
    B X
    C Z
    C Y
    A X
    A Z
    C Y
    C Y
    A X
    A Z
    A X
    C Y
    C Y
    B X
    A X
    C Y
    B X
    B X
    A Z
    B X
    A Z
    A Z
    C Y
    C Y
    B X
    C Y
    B X
    A X
    C Y
    B Z
    A X
    C Y
    B Z
    A X
    A Z
    C Y
    C Y
    B X
    A X
    B X
    B Z
    A Z
    C X
    B Z
    A X
    B Z
    C Y
    B X
    A Z
    C X
    B Z
    A Z
    A X
    C Y
    B X
    C Y
    B X
    A Z
    B Z
    B X
    C X
    B Z
    B Y
    C Y
    B Z
    B Z
    B Z
    B Z
    C Y
    C Y
    A Z
    C Y
    C Z
    C Z
    C Y
    C Y
    B Z
    A Y
    B X
    A Y
    B Z
    A Z
    C Z
    B X
    B X
    C X
    A Z
    C Y
    A Z
    B X
    B X
    A X
    B Z
    B X
    C X
    A Z
    B Y
    A X
    B X
    A Z
    A X
    B Z
    C Y
    B X
    B X
    C Y
    C Y
    B X
    A Y
    C X
    B X
    C X
    B Y
    B X
    C Y
    C Y
    B X
    A Z
    A Z
    A Y
    B Z
    B Z
    B Z
    C Y
    A X
    B X
    C Y
    C Y
    A X
    C Y
    A Z
    B X
    A X
    A Y
    B Z
    B Y
    B Y
    C X
    A Z
    A Z
    B Z
    A Z
    B Z
    B X
    A Z
    B Z
    B Y
    B X
    B X
    B X
    A Y
    A X
    B X
    B X
    A X
    C Y
    B Z
    C Y
    B Y
    A X
    B Z
    A Z
    C Y
    A X
    C Y
    B X
    B Z
    B Z
    C Y
    B Z
    C Y
    B X
    C Y
    A Z
    A X
    B X
    C Y
    A X
    A X
    C Y
    C X
    C Y
    A Z
    B Z
    A X
    C X
    C X
    C Y
    B X
    C X
    A X
    C Y
    B Y
    A X
    C Y
    B X
    A Z
    C Y
    C Y
    A X
    A X
    B X
    B Y
    B Y
    A Y
    A X
    C Y
    A X
    C X
    B X
    C Y
    C Y
    A X
    A Z
    B Z
    B Z
    C Y
    B Z
    B Y
    A X
    C X
    C X
    A X
    C Y
    B Z
    A Z
    B X
    C Y
    C Y
    C X
    C Y
    C X
    A X
    C Y
    B X
    C X
    B X
    A Z
    B Z
    B Z
    B Z
    B X
    B Z
    B X
    B Y
    C Y
    C X
    A Y
    B Z
    B Y
    C Y
    B Z
    A X
    A X
    A Z
    C Y
    A X
    C X
    C Y
    B Y
    B X
    B X
    A X
    B Y
    C Y
    B X
    C X
    C X
    A Z
    A Z
    C Y
    B X
    B Z
    C Y
    B Z
    B Z
    C Y
    C X
    A X
    B Z
    C Y
    C Y
    C Y
    B Z
    B X
    C Y
    B Z
    A X
    B X
    C Y
    A Z
    C Y
    A Z
    A Z
    B Z
    C Y
    A Z
    C Y
    B X
    A X
    A X
    B Z
    C Y
    B Y
    B X
    A Z
    B X
    C Y
    A Z
    C Y
    A X
    A Z
    B X
    B X
    B Y
    B X
    A Z
    C Y
    A X
    C Y
    C Y
    C Y
    C Z
    C Y
    A Z
    A Y
    B Z
    C Y
    B X
    C Y
    C Y
    A Z
    C Z
    C Y
    B X
    C Y
    A Z
    B Z
    B Y
    B Z
    A Z
    B X
    B X
    C X
    B X
    B Z
    A X
    C Y
    C Y
    C Y
    A Z
    B X
    B Y
    A X
    A Z
    C Y
    A Z
    A Z
    B X
    C Y
    B Z
    B Z
    B X
    C X
    A Z
    C X
    B Z
    C Y
    A Z
    B X
    C Y
    B X
    A X
    C Z
    C X
    C Y
    A Z
    C Y
    B X
    B X
    B X
    B X
    B X
    A Y
    C Y
    B X
    A X
    A Z
    A Z
    A X
    C Z
    B Z
    C Y
    B X
    B Z
    A X
    B X
    B X
    C Y
    B X
    C Y
    A Z
    A Z
    C X
    B Z
    A X
    C Y
    B X
    C Y
    C Y
    C Y
    B X
    B Y
    A Z
    C Y
    C Y
    A X
    A X
    C Y
    B X
    B Z
    C Y
    A X
    C Z
    B Z
    B X
    C Y
    C Y
    B Z
    A X
    A Z
    B X
    C Y
    A Z
    A X
    A Y
    C Y
    B X
    B Z
    A X
    B Z
    B Z
    C Y
    C Y
    C X
    B Z
    A X
    C Y
    B Y
    A Z
    B X
    B Y
    B X
    C Y
    A Z
    B X
    A Z
    C Y
    B X
    B X
    C Y
    B Y
    C Y
    C Y
    C Y
    B X
    A X
    C Y
    C Y
    C Y
    A Z
    C Y
    B X
    B X
    B X
    B X
    A X
    B Y
    C X
    B X
    C X
    A X
    C X
    C Y
    A X
    A Z
    C Y
    B Z
    B Y
    A X
    B Y
    C Y
    C Y
    B X
    A X
    A X
    C X
    B Z
    C Y
    C Y
    A Z
    B Z
    B Z
    C X
    C Y
    B X
    B X
    B Z
    A Z
    C Y
    C Y
    A Z
    A Z
    B X
    B X
    B Y
    B Z
    B Z
    A Y
    B Z
    A Z
    B Z
    B X
    B Z
    C Z
    B Z
    A Z
    B Z
    B X
    B X
    B X
    A X
    C Y
    A Z
    C Y
    C Y
    B X
    A Z
    C X
    B Z
    B Y
    C Z
    B X
    C Y
    A X
    C Y
    A Y
    B Z
    C Y
    C Y
    B X
    B X
    A Z
    C X
    A X
    A Z
    B Y
    C Y
    C Y
    C Y
    B X
    C Y
    C Y
    A Z
    B X
    A Z
    C X
    C Y
    B Z
    B Z
    B Z
    A Z
    C Y
    C Z
    B Z
    C Y
    C Y
    C Y
    B X
    B X
    C Y
    C Y
    B X
    A X
    A Z
    B X
    C Y
    B X
    A Z
    C Y
    C Y
    B Y
    A X
    A X
    A X
    A X
    B X
    B Y
    C Y
    A Z
    B Y
    B Z
    A X
    B X
    C Y
    A Y
    C Y
    A Z
    B X
    B Z
    B Z
    A Y
    B X
    B X
    C Y
    A Z
    A Z
    A X
    A X
    B Z
    C Y
    B Z
    C X
    A Z
    C Y
    B Z
    C Y
    C Y
    B Z
    A Z
    C Y
    C X
    A Z
    C Y
    B X
    A Y
    A X
    B Z
    A Y
    A Z
    B X
    C X
    B Z
    C Y
    C Y
    B Y
    B Z
    A X
    B X
    A Z
    C Y
    C Y
    A Z
    A Z
    B X
    C Y
    B Z
    A X
    A X
    B X
    B Y
    C Y
    B X
    C X
    C Y
    B Z
    A Z
    A Y
    B X
    C Y
    C Y
    C X
    A Z
    C Z
    B Z
    B X
    A Z
    A X
    A X
    B Z
    B X
    A Z
    B Z
    C Y
    A X
    B Z
    B Z
    C Y
    C Y
    A X
    B X
    C Y
    A X
    B X
    C Y
    B X
    A Z
    A X
    B Z
    A Z
    B Z
    B Y
    A X
    B Z
    A Z
    A Z
    A Z
    A X
    A X
    B X
    B X
    A Z
    B Y
    A Y
    C Y
    B X
    C Y
    B Y
    C Y
    A Z
    A Z
    B X
    B X
    C Y
    B Z
    C Y
    B Z
    B X
    B X
    B X
    B X
    A X
    B Y
    B X
    C X
    A Z
    A Z
    B X
    C Y
    C Y
    C Y
    A X
    C Y
    C Y
    C Y
    A X
    C Y
    C X
    B X
    A Y
    C Z
    C Y
    A Z
    B X
    A Z
    A X
    B X
    A X
    A X
    C Y
    A Y
    A X
    B X
    A Z
    B Z
    B Y
    C Z
    B X
    A Z
    C Y
    B Z
    B X
    C Y
    C Y
    C Y
    B X
    A X
    A Z
    B X
    A X
    C Y
    B Z
    A X
    C Y
    C Y
    A X
    A X
    A X
    A Y
    C Y
    C Y
    C Y
    C Z
    C Y
    A X
    B X
    C Y
    C Y
    C Y
    C Y
    A X
    B Z
    A X
    B X
    C Y
    B Z
    A Z
    C Y
    A Z
    C Y
    B X
    B X
    C Y
    C Y
    B Y
    A X
    C Y
    C X
    C Y
    C X
    A Z
    A Z
    B Y
    A X
    A X
    B Z
    A Z
    A X
    A Z
    C Y
    A X
    B Z
    C X
    A X
    A X
    B X
    A X
    B Y
    A X
    B X
    B X
    C Y
    C X
    B X
    B Y
    C X
    A Z
    B Z
    B X
    C Y
    B Y
    B Z
    C Y
    A X
    C Y
    B X
    B X
    C Y
    C Z
    B X
    B X
    C X
    B Y
    B Y
    B Z
    C X
    B X
    C Y
    B X
    C Y
    B X
    A X
    A Y
    B X
    B X
    B X
    B Y
    C Y
    B X
    B X
    B X
    C Y
    A Z
    B Z
    B X
    A Z
    B X
    B Y
    C X
    A Z
    C Y
    C Y
    B Z
    B X
    C Y
    C Y
    C Y
    C Y
    B Z
    A X
    C Y
    C Y
    A Z
    B X
    B Z
    A Y
    C Y
    B X
    C Z
    A Z
    B Y
    A Y
    A Z
    A Z
    B X
    B X
    C X
    A Z
    B Z
    A X
    B Z
    A Z
    B X
    B X
    C Y
    B Z
    C Y
    A Z
    B Z
    B X
    A Z
    C Y
    B X
    B Y
    B Z
    C Y
    B X
    C Y
    B X
    A X
    A X
    C Y
    B X
    B X
    A Y
    A Z
    A Z
    B Z
    A Z
    C Y
    C Y
    A Z
    A X
    A Z
    B X
    C Y
    C X
    A Z
    A X
    B Z
    B Z
    A X
    B X
    C X
    C Y
    C Y
    B Z
    B X
    C Y
    C Z
    B Y
    C Y
    A Z
    B Z
    B Z
    B X
    B X
    C Y
    B X
    B X
    C X
    A X
    B Z
    C Y
    B Y
    B X
    A Z
    B Z
    B X
    A Z
    B Z
    B X
    A Z
    B X
    C Y
    C X
    B Y
    B Y
    B Z
    C Y
    A X
    B Z
    B Y
    C Y
    A X
    B Z
    B Z
    C X
    B X
    C Y
    C Y
    C Y
    C Y
    A Z
    A Z
    B X
    B X
    C Z
    B X
    B Z
    B X
    B Z
    A Z
    B X
    C Y
    B X
    C Y
    C Y
    A X
    B X
    A X
    C X
    B X
    B Y
    A X
    B Z
    A Z
    B Y
    B Z
    C Y
    B Z
    C Y
    B X
    B Z
    B Y
    B X
    B X
    A Z
    B Z
    B X
    A Z
    C Y
    C Y
    A Z
    A X
    C Z
    A X
    B Z
    B Z
    C Y
    B X
    B X
    A Y
    B Z
    C X
    C Y
    C Y
    C Y
    A Y
    A X
    B X
    B Z
    A Z
    B X
    C X
    C X
    C Y
    C Y
    C Y
    B X
    B X
    A Z
    B Z
    B X
    A X
    A Z
    C Z
    B Z
    B Z
    B X
    A X
    B X
    C Y
    B Z
    A X
    B Y
    B Z
    B Y
    B X
    A X
    B Z
    C Y
    C X
    C Y
    A Z
    B Z
    B X
    B Z
    B X
    A Y
    B X
    A Z
    B X
    C X
    A X
    C Y
    B X
    C Y
    C Y
    A Y
    C Z
    A Z
    C X
    B X
    C Y
    A Z
    B X
    B Z
    B Z
    C Y
    A X
    A Z
    B X
    B X
    B Z
    B X
    C Y
    B X
    A X
    A X
    C Y
    A Z
    C Y
    B Y
    B Z
    B Z
    C Y
    B Z
    C Y
    B X
    B X
    B Y
    C Y
    A X
    A Z
    A Z
    A X
    C Y
    B X
    B Y
    B Y
    B X
    A X
    A Y
    B Y
    B Y
    C Y
    A X
    B Z
    C Y
    A Z
    C Y
    B Y
    A X
    B X
    A X
    B X
    A Z
    C Y
    C Y
    A X
    B Y
    B Y
    B X
    A Z
    A Z
    A Z
    A Y
    B Z
    C X
    B X
    C Y
    C Z
    B Y
    C Y
    C X
    B X
    B Z
    B Z
    B Z
    B Z
    B Y
    B Z
    A Z
    C X
    B X
    C Y
    C Y
    B Z
    C Y
    C Y
    A X
    C X
    A X
    B Z
    C Y
    B X
    C Y
    A X
    A Z
    C Z
    A X
    B Z
    C Y
    C Y
    C Y
    A X
    C Y
    C Y
    B X
    B X
    B Y
    C Y
    B Z
    B X
    C Y
    A Z
    C X
    A Z
    C Y
    A Z
    A X
    B Z
    B X
    C Z
    B X
    C Y
    A Z
    C Y
    B X
    B X
    B X
    B Z
    C Y
    B X
    A Z
    A Z
    C Y
    B Y
    C Y
    B Z
    A Z
    A X
    B Z
    B X
    B X
    A Y
    B X
    B X
    C Y
    B X
    B X
    B Z
    C Y
    B X
    A X
    A Z
    A Z
    C X
    A X
    A X
    B X
    B X
    B Z
    A X
    C Y
    B X
    C Y
    C Y
    A X
    B X
    A X
    A Z
    A Z
    C Y
    B X
    C Y
    B Y
    B X
    B X
    C X
    B X
    A X
    C X
    B Z
    C Y
    B Z
    B X
    C Y
    C Y
    A Z
    C Y
    B Z
    B X
    A Z
    C Y
    C Y
    B X
    B Z
    A X
    B X
    C Y
    C Y
    C Y
    A X
    A X
    A X
    B Z
    B Z
    A X
    A X
    B X
    B Z
    C Y
    C Y
    B Z
    A Z
    C Y
    C Y
    A Z
    B Z
    C Y
    B X
    B X
    B X
    C Y
    C Y
    B Z
    C Y
    B Z
    A X
    A X
    B X
    C Y
    A X
    A X
    C X
    A Y
    C X
    B X
    B X
    B Z
    B Z
    B Z
    B X
    B Z
    C Y
    C Y
    C X
    A Z
    B X
    A X
    B Z
    B X
    A Z
    C Y
    B X
    B Y
    B Z
    A Z
    B X
    A Y
    C X
    B X
    C Z
    C Y
    B Z
    A X
    B Z
    B Z
    A X
    B Z
    B X
    C Y
    B Z
    C X
    C Y
    A Z
    C Y
    A Y
    B Z
    B X
    B Y
    A Z
    A Y
    A X
    B Y
    A Z
    B Z
    C Y
    C Y
    C X
    C Y
    C Y
    B X
    B Z
    A Z
    B Z
    B Z
    A Z
    C Y
    A Z
    C X
    C Y
    C Y
    C Y
    A X
    C Y
    B Z
    A X
    C Y
    C Y
    C Y
    B X
    B Z
    B Y
    C Y
    C Y
    C Y
    C Y
    B Y
    A Y
    B Z
    B X
    A X
    A Y
    C Y
    C X
    B X
    B Z
    A Z
    B Z
    B X
    A X
    C Z
    C X
    B Z
    C Y
    A X
    C X
    B Z
    B X
    B X
    A Z
    B X
    B Z
    B Y
    C Y
    B X
    A X
    B X
    B Z
    A X
    B Z
    A Y
    C Y
    A X
    C X
    B X
    B Y
    B Z
    B X
    B X
    C Y
    B X
    B X
    C Y
    B Y
    B Z
    B Y
    B Z
    C Y
    A Z
    C Y
    A Z
    B X
    B X
    A Z
    B X
    C Y
    C X
    C Y
    B Y
    C Y
    C Y
    C Z
    C Y
    A Y
    C Y
    B X
    B Z
    A Y
    B Z
    A Z
    B Z
    B Z
    B X
    B Y
    C Y
    B X
    C Y
    C Z
    C Y
    A X
    B X
    B X
    C Z
    A Z
    B X
    A Z
    C Y
    B Z
    B Z
    B Y
    A Z
    B Z
    C Y
    C Y
    A Z
    C Y
    B Y
    C Y
    B Z
    C X
    B Z
    B X
    A Z
    B Y
    B Y
    B Z
    A Z
    B Z
    C X
    C X
    B Z
    B X
    C Y
    B X
    A X
    B X
    B X
    B Z
    C Y
    A Z
    A Y
    B Z
    C Y
    A X
    A X
    A Y
    B X
    B Z
    A X
    B X
    C X
    B X
    B Y
    B X
    A Z
    C Y
    A X
    C Y
    A X
    C Y
    C Y
    B Y
    C Y
    B X
    C Y
    C Y
    B X
    B Z
    A X
    C Y
    C X
    C X
    C Y
    B Y
    A Z
    C Y
    A X
    A X
    B X
    B X
    A X
    B X
    C X
    B Z
    B X
    A X
    A Z
    B X
    C Y
    C Y
    A X
    C X
    B Z
    B Z
    B X
    C Y
    C Y
    A Z
    C X
    B Z
    C Y
    B Y
    A Z
    B Z
    A Z
    B X
    C X
    C Z
    C Y
    A Z
    B Z
    B X
    C Y
    C Y
    C X
    C Y
    C Y
    C Y
    A X
    A Z
    C Y
    B X
    B X
    B Z
    A Z
    C Y
    B X
    A Z
    C Y
    C Y
    B Z
    A X
    B Z
    B Z
    B X
    B Z
    B X
    B X
    B Z
    B Z
    C Y
    A X
    A X
    A Z
    C Y
    C Y
    B X
    B Z
    C Z
    B X
    C Z
    C Y
    C X
    C Y
    B Y
    A X
    B Z
    A Z
    C X
    B X
    C X
    A Z
    A Y
    B Z
    A X
    A X
    A Z
    A Z
    B Y
    B Z
    C Y
    B X
    C Y
    B Z
    B X
    C Y
    C Y
    A X
    A Z
    B X
    C Y
    B X
    B X
    A X
    A Z
    B X
    B X
    B Z
    A X
    A X
    B Z
    B X
    B X
    B X
    A Z
    C X
    B X
    B X
    B X
    B Z
    B X
    C X
    C Y
    A Z
    C X
    A Z
    C Y
    A X
    C Y
    A X
    B X
    A Z
    B X
    C Y
    C Y
    A Z
    A Z
    B Z
    B Z
    B X
    C X
    A X
    B Y
    B Z
    C Y
    B Z
    A X
    A X
    B X
    A X
    A X
    A X
    A X
    B Z
    B Y
    C Y
    A Z
    C Z
    C Y
    A X
    B Y
    C X
    A Z
    B X
    B Z
    A X
    A X
    C Y
    B X
    B X
    A X
    A Z
    C Y
    C Y
    A Z
    C Y
    B X
    B X
    A Z
    A Y
    C Y
    A Z
    A X
    C Y
    B Z
    B Z
    C Y
    B X
    C Y
    C Y
    C Y
    C Z
    B X
    C Y
    C X
    B Z
    A Z
    C Y
    A Z
    B Z
    A Z
    B X
    A X
    A Z
    B Z
    B Z
    A X
    B Z
    A X
    B X
    A Z
    B X
    B X
    A Z
    C Y
    B X
    B Z
    C Y
    B Z
    B Z
    B X
    B Z
    B X
    B X
    C Y
    B X
    B Y
    B X
    B X
    A Z
    B X
    C Y
    B X
    B X
    B X
    B Z
    A Z
    B Y
    B X
    B Z
    C X
    C Y
    B X
    B Y
    C Y
    B X
    C X
    B X
    C X
    B Z
    A Y
    A Z
    B Z
    B Z
    A X
    A Z
    B Z
    B X
    A Z
    B Z
    C Y
    C Z
    C Y
    A Z
    C Z
    A Z
    C Z
    C Y
    A Z
    A Y
    C Y";

    let mut score: u64 = 0;
    let mut play_result: u64;
    let mut item_score: u64;
    for strat in strats.lines() {
        let mut split = strat.trim().split(' ');
        let left: Play = split.next().unwrap().into();
        let right: Play = split.next().unwrap().into();
        play_result = right.play(&left);
        item_score = u64::from(&right);
        score += play_result + item_score;
        println!(
            "{} {score}\t{play_result} {item_score} - {left:?}\t{right:?}",
            strat.trim()
        );
    }
    if score == 15128 {
        eprintln!("failed, too high");
    }
    if score == 10094 {
        eprintln!("failed, too high");
    }
    if score == 9759 {
        eprintln!("Success!")
    }
    println!("Total score: {score}");
}
