use crate::{Action, Player, Rules, Shoe};
use crate::players;

pub struct Sim {
    pub rules: Rules,
    pub player: Box<dyn Player>,
    pub dealer: players::Dealer,
    pub shoe: Shoe,
    pub surr_flag: u8, // Bitmap for whether surrender is available or not
    pub max_split_hands: u32,
}

impl Sim {
    pub fn new(rules: Rules) -> Self {
        let player: Box<dyn Player> = if rules.player_counts_cards {
            Box::new(players::CardCounter::new(rules.n_decks))
        } else {
            Box::new(players::StandardPlayer)
        };

        let dealer = players::Dealer::new(rules.dealer_hits_soft_17);
        let shoe = Shoe::new(rules.n_decks, rules.penetration);
        let surr_flag = if rules.can_surrender { Action::Surrender.bitmap() } else { 0 };
        let max_split_hands = rules.max_split_hands.unwrap_or(u32::MAX);

        Self { rules, player, dealer, shoe, surr_flag, max_split_hands }
    }
}