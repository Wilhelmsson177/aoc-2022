use std::str::FromStr;

use itertools::Itertools;

#[derive(Debug, Clone, Copy)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug, Clone, Copy)]
struct Round {
    theirs: Move,
    ours: Move,
}

#[derive(Debug, Clone, Copy)]
enum Outcome {
    Win,
    Draw,
    Loss,
}

impl TryFrom<char> for Outcome {
    type Error = color_eyre::Report;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'X' => Ok(Outcome::Loss),
            'Y' => Ok(Outcome::Draw),
            'Z' => Ok(Outcome::Win),
            _ => Err(color_eyre::eyre::eyre!("not a valid outcome: {c:?}")),
        }
    }
}

impl Outcome {
    fn matching_move(self, theirs: Move) -> Move {
        match self {
            Outcome::Win => theirs.winning_move(),
            Outcome::Draw => theirs.drawing_move(),
            Outcome::Loss => theirs.loosing_move(),
        }
    }
}

impl TryFrom<char> for Move {
    type Error = String;
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'A' => Ok(Move::Rock),
            'B' => Ok(Move::Paper),
            'C' => Ok(Move::Scissors),
            _ => Err("this is not a vlaid move: {c:?}".to_string()),
        }
    }
}

impl std::str::FromStr for Round {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();
        let (Some(theirs), Some(' '), Some(outcome), None) =
            (chars.next(), chars.next(), chars.next(), chars.next())
        else {
            return Err("expected <theirs>SP<ours>EOF, got {s:?}".to_string());
        };
        let theirs = Move::try_from(theirs)?;
        let outcome = Outcome::try_from(outcome).unwrap();
        let ours = outcome.matching_move(theirs);
        Ok(Self {
            theirs,
            ours,
        })
    }
}

impl Outcome {
    fn inherit_points(self) -> usize {
        match self {
            Outcome::Win => 6,
            Outcome::Draw => 3,
            Outcome::Loss => 0,
        }
    }
}

impl Move {
    fn inherit_points(self) -> usize {
        match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }

    fn outcome(self, theirs: Move) -> Outcome {
        if self.beats(theirs) {
            Outcome::Win
        } else if theirs.beats(self) {
            Outcome::Loss
        } else {
            Outcome::Draw
        }
    }

    const ALL_MOVES: [Move; 3] = [Move::Rock, Move::Paper, Move::Scissors];

    fn winning_move(self) -> Self {
        Self::ALL_MOVES
            .iter()
            .copied()
            .find(|m| m.beats(self))
            .expect("at least one move beats us")
    }

    fn loosing_move(self) -> Self {
        Self::ALL_MOVES
            .iter()
            .copied()
            .find(|&m| self.beats(m))
            .expect("we beat ar leas one move")
    }

    fn drawing_move(self) -> Self {
        self
    }

    fn beats(self, other: Move) -> bool {
        matches!(
            (self, other),
            (Self::Rock, Self::Scissors)
                | (Self::Paper, Self::Scissors)
                | (Self::Scissors, Self::Paper)
        )
    }
}

impl Round {
    fn outcome(self) -> Outcome {
        self.ours.outcome(self.theirs)
    }

    fn our_score(self) -> usize {
        self.ours.inherit_points() + self.outcome().inherit_points()
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut sum = 0;
    for round in input.lines().map(|line| line.parse::<Round>()) {
        let round = round.unwrap();
        println!(
            "{round:?}: outcome={outcome:?}, our score={our_score}",
            outcome = round.outcome(),
            our_score = round.our_score()
        );
        sum += round.our_score() as u32;
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut sum = 0;
    for round in input.lines().map(|line| line.parse::<Round>()) {
        let round = round.unwrap();
        println!(
            "{round:?}: outcome={outcome:?}, our score={our_score}",
            outcome = round.outcome(),
            our_score = round.our_score()
        );
        sum += round.our_score() as u32;
    }
    Some(sum)
}

advent_of_code::main!(2);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", 2));
        assert_eq!(result, Some(15));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", 2));
        assert_eq!(result, Some(12));
    }
}
