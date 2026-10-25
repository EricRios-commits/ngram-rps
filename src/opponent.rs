use crate::game::Move;
use crate::predictor::MovePredictor;
use crate::rng::RngMoveProvider;

pub struct Opponent<P: MovePredictor, R: RngMoveProvider> {
    predictor: P,
    rng: R,
}

impl<P: MovePredictor, R: RngMoveProvider> Opponent<P, R> {
    pub fn new(predictor: P, rng: R) -> Self {
        Self { predictor, rng }
    }

    pub fn play(&mut self) -> Move {
        self.choose_move()
    }

    pub fn observe(&mut self, player_move: Move) {
        self.predictor.observe(player_move);
    }

    fn choose_move(&mut self) -> Move {
        match self.predictor.predict() {
            // TODO: extend for multiple possible counters
            Some(predicted_move) => predicted_move.counters()[0],
            None => self.rng.random_move(),
        }
    }
}
