#[derive(Debug, Clone, Copy)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug, Clone, Copy)]
enum Outcome {
    Win,
    Lose,
    Draw
}

#[derive(Debug, Clone, Copy)]
struct Round {
    theirs: Move,
    ours: Move,
}

#[derive(Debug, Clone, Copy)]
struct Round2 {
    theirs: Move,
    outcome: Outcome,
}
struct Score2 {
    score: u8,
}

struct Score {
    score: u8,
}

impl From<Round> for Score {
    fn from(value: Round) -> Self {
        let s = match value {
            Round {
                theirs: Move::Paper,
                ours: Move::Paper,
            } => 5,
            Round {
                theirs: Move::Paper,
                ours: Move::Rock,
            } => 1,
            Round {
                theirs: Move::Paper,
                ours: Move::Scissors,
            } => 9,
            Round {
                theirs: Move::Rock,
                ours: Move::Scissors,
            } => 3,
            Round {
                theirs: Move::Rock,
                ours: Move::Paper,
            } => 8,
            Round {
                theirs: Move::Rock,
                ours: Move::Rock,
            } => 3,
            Round {
                theirs: Move::Scissors,
                ours: Move::Scissors,
            } => 6,
            Round {
                theirs: Move::Scissors,
                ours: Move::Paper,
            } => 2,
            Round {
                theirs: Move::Scissors,
                ours: Move::Rock,
            } => 7,
        };
        Score { score: s }
    }
}

impl From<Round2> for Score2 {
    fn from(value: Round2) -> Self {
        let s = match value {
            Round2 {
                theirs: Move::Paper,
                outcome: Outcome::Draw
            } => 5,
            Round2 {
                theirs: Move::Paper,
                outcome: Outcome::Win
            } => 1,
            Round2 {
                theirs: Move::Paper,
                outcome: Outcome::Lose
            } => 1,
            Round2 {
                theirs: Move::Rock,
                outcome: Outcome::Win
            } => 8,
            Round2 {
                theirs: Move::Rock,
                outcome: Outcome::Draw
            } => 4,
            Round2 {
                theirs: Move::Rock,
                outcome: Outcome::Lose
            } => 3,
            Round2 {
                theirs: Move::Scissors,
                outcome: Outcome::Draw
            } => 6,
            Round2 {
                theirs: Move::Scissors,
                outcome: Outcome::Win
            } => 7,
            Round2 {
                theirs: Move::Scissors,
                outcome: Outcome::Lose
            } => 2,
        };
        Score2 { score: s }
    }
}


impl TryFrom<char> for Move {
    type Error = String;
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'A' => Ok(Move::Rock),
            'B' => Ok(Move::Paper),
            'C' => Ok(Move::Scissors),
            'X' => Ok(Move::Rock),
            'Y' => Ok(Move::Paper),
            'Z' => Ok(Move::Scissors),
            _ => Err("this is not a valid move: {c:?}".to_string()),
        }
    }
}

impl TryFrom<char> for Outcome {
    type Error = String;
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'X' => Ok(Outcome::Lose),
            'Y' => Ok(Outcome::Draw),
            'Z' => Ok(Outcome::Win),
            _ => Err("this is not a valid outcome: {c:?}".to_string()),
        }
    }
}

impl std::str::FromStr for Round {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();
        let (Some(theirs), Some(' '), Some(ours), None) =
            (chars.next(), chars.next(), chars.next(), chars.next())
        else {
            return Err("expected <theirs>SP<ours>EOF, got {s:?}".to_string());
        };
        let theirs = Move::try_from(theirs)?;
        let ours = Move::try_from(ours)?;
        Ok(Self { theirs, ours })
    }
}

impl std::str::FromStr for Round2 {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();
        let (Some(theirs), Some(' '), Some(outcome), None) =
            (chars.next(), chars.next(), chars.next(), chars.next())
        else {
            return Err("expected <theirs>SP<ours>EOF, got {s:?}".to_string());
        };
        let theirs = Move::try_from(theirs)?;
        let outcome = Outcome::try_from(outcome)?;
        Ok(Self { theirs, outcome })
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut sum = 0;
    for round in input.lines().map(|line| line.parse::<Round>()) {
        let round = round.unwrap();
        sum += Score::from(round).score as u32;
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut sum = 0;
    for round in input.lines().map(|line| line.parse::<Round2>()) {
        let round = round.unwrap();
        sum += Score2::from(round).score as u32;
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
