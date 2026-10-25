use std::collections::{BTreeMap, HashMap};

use crate::game::Move;

pub trait MovePredictor {
    fn predict(&self) -> Option<Move>;
    fn observe(&mut self, played: Move);
}

pub struct NGramPredictor {
    context_lengths: Vec<usize>,
    history: Vec<Move>,
    followers: HashMap<Vec<Move>, BTreeMap<Move, u32>>,
}

impl NGramPredictor {
    pub fn new(window_size: usize, backoff: bool) -> Self {
        Self {
            context_lengths: Self::context_lengths_for(window_size, backoff),
            history: vec![],
            followers: HashMap::new(),
        }
    }

    fn context_lengths_for(window_size: usize, backoff: bool) -> Vec<usize> {
        match backoff {
            true => (0..window_size).rev().collect(),
            false => vec![window_size],
        }
    }

    fn context_of_length(&self, length: usize) -> Option<&[Move]> {
        let context_start = self.history.len().checked_sub(length)?;
        Some(&self.history[context_start..])
    }

    fn predict_from_context_of_length(&self, length: usize) -> Option<Move> {
        let follower_counts = self.followers.get(self.context_of_length(length)?)?;
        follower_counts
            .iter()
            .max_by_key(|&(_, &count)| count)
            .map(|(&follower, _)| follower)
    }

    fn record_follower(&mut self, context: Vec<Move>, follower: Move) {
        *self
            .followers
            .entry(context)
            .or_default()
            .entry(follower)
            .or_default() += 1;
    }
}

impl MovePredictor for NGramPredictor {
    fn predict(&self) -> Option<Move> {
        self.context_lengths
            .iter()
            .find_map(|&length| self.predict_from_context_of_length(length))
    }

    fn observe(&mut self, played: Move) {
        let contexts: Vec<Vec<Move>> = self
            .context_lengths
            .iter()
            .filter_map(|&length| self.context_of_length(length))
            .map(<[Move]>::to_vec)
            .collect();
        for context in contexts {
            self.record_follower(context, played);
        }
        self.history.push(played);
    }
}