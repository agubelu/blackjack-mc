use crate::{Action, Hand, Player};

pub struct Dealer {
    hit_soft_17: bool,
}

impl Dealer {
    pub fn new(hit_soft_17: bool) -> Self {
        Self { hit_soft_17 }
    }
}

impl Player for Dealer {
    fn observe_card(&mut self, _card: u8) {
        unimplemented!();
    }

    fn reset_count(&mut self) {
        unimplemented!();
    }

    fn decide(&self, hand: Hand, _dealer: u8, _allowed: u8) -> Action {
        if hand.value() < 17 || hand == Hand::Soft(17) && self.hit_soft_17 {
            Action::Hit
        } else {
            Action::Stand
        }
    }

    fn place_bet(&self) -> i64 {
        unimplemented!();
    }
}
