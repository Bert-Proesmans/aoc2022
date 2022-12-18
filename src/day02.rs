#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Play {
    Rock = 0,
    Paper = 1,
    Scissors = 2,
}

#[derive(Debug)]
pub struct Round {
    pub mine: Play,
    pub theirs: Play,
}

#[derive(Debug, Clone, Copy)]
pub enum Outcome {
    Loss,
    Draw,
    Win,
}

impl Play {
    pub fn move_to_lose(&self) -> Self {
        match self {
            Play::Rock => Play::Scissors,
            Play::Paper => Play::Rock,
            Play::Scissors => Play::Paper,
        }
    }

    pub fn move_to_win(&self) -> Self {
        match self {
            Play::Rock => Play::Paper,
            Play::Paper => Play::Scissors,
            Play::Scissors => Play::Rock,
        }
    }
}

pub fn rock_paper_scissors_score(it: impl Iterator<Item = Round>) -> usize {
    rock_paper_scissors_rules(it).fold(0, |score, results| {
        let play_score = match results.0.mine {
            Play::Rock => 1,
            Play::Paper => 2,
            Play::Scissors => 3,
        };
        let round_score = match results.1 {
            Outcome::Loss => 0,
            Outcome::Draw => 3,
            Outcome::Win => 6,
        };
        score + (play_score + round_score) as usize
    })
}

pub fn rock_paper_scissors_rules(
    it: impl Iterator<Item = Round>,
) -> impl Iterator<Item = (Round, Outcome)> {
    use Outcome::*;

    it.map(|round| match round {
        Round {
            ref mine,
            ref theirs,
        } if mine == theirs => (round, Draw),
        Round {
            ref mine,
            ref theirs,
        } if theirs.move_to_lose() == *mine => (round, Loss),
        _ => (round, Win),
    })
}
