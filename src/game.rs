use crate::players::player::Player;
use crate::hand::Hand;
use crate::shoe::Shoe;

pub enum Action {
    Hit,
    Stand,
    Double,
    Split,
    Surrender,
}

pub struct Game<'a> {
    player_hand: Hand,
    dealer_hand: Hand,
    dealer_card: u8,
    bet: i32,
    shoe: &'a mut Shoe
}

impl Game<'_> {
    pub fn play(&mut self, player: &impl Player, dealer: &impl Player) -> i32 {
        // Check for blackjacks
        let player_val = self.player_hand.value();
        let dealer_val = self.dealer_hand.value();

        if dealer_val == 21 {
            return if player_val == 21 { self.bet } else { 0 };
        }

        if player_val == 21 {
            // Dealer doesn't have BJ, player wins
            return (self.bet as f32 * 1.5) as i32;
        }

        0
    }
}