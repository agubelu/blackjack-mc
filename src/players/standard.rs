use crate::{Action, Hand, Player};
use super::book::best_actions;

pub struct StandardPlayer;

impl Player for StandardPlayer {
    fn observe_card(&mut self, _card: u8) { }

    fn reset_count(&mut self) { }

    fn decide(&self, hand: Hand, dealer: u8, allowed: u8) -> Action {
        let actions = best_actions(hand, dealer);
        Action::from_bitmap(actions & allowed)
    }

    fn place_bet(&self) -> i32 { 10 }
}
