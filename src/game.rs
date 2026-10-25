use core::fmt;

#[derive(Copy, PartialEq, Eq, Debug, Clone, Hash, PartialOrd, Ord)]
pub enum Move {
    Rock,
    Paper,
    Scissors,
}

impl Move {
    pub const ALL: [Move; 3] = [Move::Rock, Move::Paper, Move::Scissors];

    // returns the moves that beat 'self'
    pub fn counters(self) -> Vec<Self> {
        match self {
            Move::Rock => vec![Move::Paper],
            Move::Paper => vec![Move::Scissors],
            Move::Scissors => vec![Move::Rock],
        }
    }

    pub fn outcome_against(self, other: Self) -> Outcome {
        if self == other {
            return Outcome::Draw;
        }
        if self.counters().contains(&other) {
            return Outcome::Loss;
        }
        return Outcome::Win;
    }
}

impl fmt::Display for Move {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "{self:?}")
    }
}

pub enum Outcome {
    Win,
    Loss,
    Draw,
}

pub struct Round {
    player_move: Move,
    opponent_move: Move,
}

impl Round {
    pub fn new(player_move: Move, opponent_move: Move) -> Self {
        Self {
            player_move,
            opponent_move,
        }
    }

    pub fn outcome(&self) -> Outcome {
        self.player_move.outcome_against(self.opponent_move)
    }
}

impl fmt::Display for Round {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        let verdict = match self.outcome() {
            Outcome::Win => "you win",
            Outcome::Draw => "draw",
            Outcome::Loss => "you lose",
        };
        write!(
            formatter,
            "You: {} | Opponent: {} | {} ",
            self.player_move, self.opponent_move, verdict
        )
    }
}
