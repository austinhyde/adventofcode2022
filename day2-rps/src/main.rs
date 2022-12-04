use std::{str::FromStr};

fn main() {
    let input = include_str!("input.txt");
    println!("Part 1 Answer: {}", part1(input));
    println!("Part 2 Answer: {}", part2(input));
}

/** Part 1:
    The first column is what your opponent is going to play:
    A for Rock, B for Paper, and C for Scissors.

    The second column, you reason, must be what you should play in response:
    X for Rock, Y for Paper, and Z for Scissors.

    Your total score is the sum of your scores for each round.

    The score for a single round is the score for the shape you selected
    (1 for Rock, 2 for Paper, and 3 for Scissors) plus the score for the
    outcome of the round (0 if you lost, 3 if the round was a draw,
    and 6 if you won).

    What would your total score be if everything goes exactly
    according to your strategy guide?
*/
fn part1(input: &str) -> u32 {
    input.lines().map(|line| line.parse::<Play>().unwrap().score()).sum()
}

#[derive(Clone, Copy)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}
use Shape::*;

impl FromStr for Shape {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Rock),
            "B" | "Y" => Ok(Paper),
            "C" | "Z" => Ok(Scissors),
            other => Err(other.to_owned()),
        }
    }
}

impl Shape {
    fn fight(&self, other: &Self) -> Outcome {
        match (self, other) {
            (Rock, Scissors) | (Scissors, Paper) | (Paper, Rock) => Win,
            (Rock, Rock) | (Scissors, Scissors) | (Paper, Paper) => Draw,
            _ => Lose,
        }
    }

    fn score(&self) -> u32 {
        match self {
            Rock => 1,
            Paper => 2,
            Scissors => 3,
        }
    }
}

enum Outcome {
    Win,
    Lose,
    Draw,
}
use Outcome::*;

impl Outcome {
    fn score(&self) -> u32 {
        match self {
            Lose => 0,
            Draw => 3,
            Win => 6,
        }
    }
}

impl FromStr for Outcome {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Lose),
            "Y" => Ok(Draw),
            "Z" => Ok(Win),
            other => Err(other.to_owned()),
        }
    }
}

struct Play {
    them: Shape,
    me: Shape
}

impl FromStr for Play {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.split_once(" ").ok_or(s.to_owned())?;
        Ok(Self{them: s.0.parse()?, me: s.1.parse()?})
    }
}

impl Play {
    fn fight(&self) -> Outcome {
        self.me.fight(&self.them)
    }
    fn score(&self) -> u32 {
        self.fight().score() + self.me.score()
    }
}

/** Part 2:
    the second column says how the round needs to end: X means you need to lose,
    Y means you need to end the round in a draw, and Z means you need to win.

    Following the Elf's instructions for the second column, what would your total
    score be if everything goes exactly according to your strategy guide?
*/
fn part2(input: &str) -> u32 {
    input.lines().map(|line| line.parse::<Round>().unwrap().score()).sum()
}

struct Round {
    them: Shape,
    outcome: Outcome,
}

impl FromStr for Round {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.split_once(" ").ok_or(s.to_owned())?;
        Ok(Self{them: s.0.parse()?, outcome: s.1.parse()?})
    }
}

impl Round {
    fn what_to_play(&self) -> Shape {
        match (&self.them, &self.outcome) {
            (Rock, Lose) | (Paper, Win) => Scissors,
            (Rock, Win) | (Scissors, Lose) => Paper,
            (Paper, Lose) | (Scissors, Win) => Rock,
            (x, Draw) => *x
        }
    }

    fn score(&self) -> u32 {
        let me = self.what_to_play();
        me.score() + me.fight(&self.them).score()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example1() {
        let input = include_str!("example1.txt");
        let expected = 15;
        assert_eq!(expected, part1(input));
    }

    #[test]
    fn part2_example1() {
        let input = include_str!("example1.txt");
        let expected = 12;
        assert_eq!(expected, part2(input));
    }
}