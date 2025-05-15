use crate::{Action, Hand};

pub trait Player {
    fn observe_card(&mut self, card: u8);

    fn reset_count(&mut self);

    fn decide(&self, hand: Hand, card: u8, allowed: u8) -> Action;

    fn place_bet(&self) -> i32;
}