use crate::{Action, Hand, Player};

pub struct Dealer;

impl Player for Dealer {
    fn observe_card(&mut self, _card: u8) {
        unimplemented!();
    }

    fn reset_count(&mut self) {
        unimplemented!();
    }

    fn decide(&self, hand: Hand, _card: u8, _allowed: u8) -> Action {
        if hand.value() < 17 { Action::Hit } else { Action::Stand } // TODO: custom hard/soft thresholds
    }

    fn place_bet(&self) -> i32 {
        unimplemented!();
    }
}
