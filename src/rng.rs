use std::time::{SystemTime, UNIX_EPOCH};

use crate::game::Move;

pub trait RngMoveProvider {
    fn random_move(&mut self) -> Move;
}

pub struct XorShiftRng {
    state: u64,
}

impl XorShiftRng {
    pub fn from_system_time() -> Self {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_or(1, |elapsed| elapsed.as_nanos() as u64);
        Self { state: nanos | 1 }
    }

    fn next_u64(&mut self) -> u64 {
        self.state ^= self.state << 13;
        self.state ^= self.state >> 7;
        self.state ^= self.state << 17;
        self.state
    }
}

impl RngMoveProvider for XorShiftRng {
    fn random_move(&mut self) -> Move {
        let index = self.next_u64() % Move::ALL.len() as u64;
        Move::ALL[index as usize]
    }
}
