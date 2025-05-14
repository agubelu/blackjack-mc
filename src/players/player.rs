use crate::game::Action;
use crate::hand::Hand;

pub trait Player {
    fn observe_card(&mut self, card: u8);

    fn reset_count(&mut self);

    fn decide(&self, hand: Hand, card: u8, first: bool) -> Action; // Add ref to game rules

    fn place_bet(&self) -> i32;
}