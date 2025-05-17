use crate::{Action, Hand, Player};
use super::book::best_actions;

pub struct CardCounter {
    n_decks: usize,
    total: i32,
    running: i32,
}

impl CardCounter {
    pub fn new(n_decks: usize) -> Self {
        Self { n_decks, total: 0, running: 0 }
    }
}

impl Player for CardCounter {
    fn observe_card(&mut self, card: u8) {
        self.total += 1;
        self.running += match card {
            1 | 10 => -1,
            2..=6 => 1,
            _ => 0,
        };
    }

    fn reset_count(&mut self) {
        self.total = 0;
        self.running = 0;
    }

    fn decide(&self, hand: Hand, dealer: u8, allowed: u8) -> Action {
        let actions = best_actions(hand, dealer);
        Action::from_bitmap(actions & allowed)
    }

    fn place_bet(&self) -> i64 {
        let decks_remaining = self.n_decks as f32 - (self.total as f32 / 52.0);
        let true_count = (self.running as f32 / decks_remaining).round() as i64;
        match true_count {
            ..1 => 10,
            x => x * 20,
        }
    }
}
