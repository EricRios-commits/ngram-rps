use core::fmt;

use crate::game::Outcome;

#[derive(Default)]
pub struct Scoreboard {
    wins: u32,
    losses: u32,
    draws: u32,
}

impl Scoreboard {
    pub fn record(&mut self, outcome: Outcome) {
        match outcome {
            Outcome::Win => self.wins += 1,
            Outcome::Loss => self.losses += 1,
            Outcome::Draw => self.draws += 1,
        }
    }

    fn decisive_rounds(&self) -> u32 {
        self.wins + self.losses
    }

    fn win_percentage(&self) -> Option<f64> {
        match self.decisive_rounds() {
            0 => None,
            rounds => Some(f64::from(self.wins) / f64::from(rounds) * 100.0),
        }
    }
}

impl fmt::Display for Scoreboard {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        let win_rate = self.win_percentage().map_or_else(
            || "n/a".to_string(),
            |percentage| format!("{percentage:.1}%"),
        );
        write!(
            formatter,
            "Wins: {}, Losses: {}, Draws: {} | Win rate: {win_rate}",
            self.wins, self.losses, self.draws
        )
    }
}