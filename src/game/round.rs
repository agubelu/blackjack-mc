use crate::{Action::*, Hand, Player, Sim};
use debug_print::debug_println;

pub struct Round<'a> {
    ctx: &'a mut Sim,
    dealer_upcard: u8,
    bet: i64,
    spent: i64,
    splits: u32,
}

// Final value of a hand and its associated bet before the dealer starts drawing
type HandBet = (u8, i64);

impl<'a> Round<'a> {
    pub fn new(sim: &'a mut Sim) -> Self {
        Self { ctx: sim, bet: 0, dealer_upcard: 0, spent: 0, splits: 0 }
    }

    /// Simulates a round of play and returns how much net money the player earned or lost.
    /// 0 => player got his bet back, positive or negative values indicate gain or loss.
    pub fn play(&mut self, bet: i64) -> i64 {
        self.bet = bet;
        self.spent = bet;

        // We can let the player observe the hole card now because the bet has already been made,
        // and it will be revealed anyways at the end of the round, so that logic is simplified.
        self.dealer_upcard = self.draw();
        let mut dealer_hand = Hand::from_cards(self.dealer_upcard, self.draw());
        let player_hand = Hand::from_cards(self.draw(), self.draw());

        // Check for blackjacks
        let player_val = player_hand.value();
        let dealer_val = dealer_hand.value();

        if dealer_val == 21 {
            debug_println!("Dealer blackjack");
            return if player_val == 21 { 0 } else { -self.bet };
        }

        if player_val == 21 {
            // Dealer doesn't have BJ, player wins
            debug_println!("Player blackjack");
            return (self.bet as f32 * self.ctx.rules.blackjack_pays) as i64;
        }

        // Game's on
        let player_hands = self.get_player_bets(player_hand);
        if player_hands.is_empty() {
            debug_println!("You busted!");
            return -self.spent;
        }

        // Player has some hands alive, simulate dealer behavior.
        // Dealer will always either hit or stand.
        debug_println!("Dealer has: {:?}", dealer_hand);
        while dealer_hand.value() <= 21 && self.ctx.dealer.decide(dealer_hand, 0, 0) == Hit {
            dealer_hand = dealer_hand.add_card(self.draw());
            debug_println!("Dealer has: {:?}", dealer_hand);
        }

        let dealer_hand = dealer_hand.value();
        let payout: i64 = player_hands.into_iter().map(|x| self.hand_payout(x, dealer_hand)).sum();
        payout - self.spent
    }

    /// Simulates player actions for a given initial hand and returns the final hands with their bets.
    /// Only hands that haven't busted are included in the results.
    /// Generaly only one hand is returned, but it could be more if hands are (recursively) split.
    fn get_player_bets(&mut self, mut hand: Hand) -> Vec<HandBet> {
        let mut this_bet = self.bet;
        let mut actions = self.get_initial_actions(hand);

        while hand.value() <= 21 {
            debug_println!("Your hand: {hand:?}, dealer card: {}", self.dealer_upcard);
            let action = self.ctx.player.decide(hand, self.dealer_upcard, actions);
            assert!(action.bitmap() & actions != 0, "Illegal action");

            // After the first action, the only possible moves are hit, stand or surrender (if allowed).
            actions = Hit | Stand | self.ctx.surr_flag;

            match action {
                Hit | Double => {
                    hand = hand.add_card(self.draw());

                    if action == Double {
                        self.spent += self.bet;
                        this_bet *= 2;
                        break;
                    }
                },
                Surrender => {
                    // Half the bet is refunded; hand ends
                    self.spent -= self.bet / 2;
                    return vec![];
                },
                Split => return self.split(hand.unpack()),
                Stand => break,
            }
        }

        let hand_value = hand.value();
        if hand_value <= 21 { vec![(hand_value, this_bet)] } else { vec![] }
    }

    /// Returns the final bets after splitting a hard on a given card
    fn split(&mut self, card: u8) -> Vec<HandBet> {
        self.spent += self.bet;
        self.splits += 1;

        // Fun fact: 21's after splitting aren't considered blackjacks,
        // so we don't have to handle that edge case.
        let hand_r = Hand::from_cards(card, self.draw());
        let hand_l = Hand::from_cards(card, self.draw());

        // If we split aces and we're not allowed to hit, both hands end immediately
        if card == 1 && !self.ctx.rules.can_hit_split_aces {
            vec![(hand_r.value(), self.bet), (hand_l.value(), self.bet)]
        } else {
            // Otherwise, play on
            let r = self.get_player_bets(hand_r);
            let l = self.get_player_bets(hand_l);
            [r, l].concat()
        }
    }

    fn get_initial_actions(&self, hand: Hand) -> u8 {
        // This function is only called for the first action of every hand.
        // Two edge cases to handle here: splitting and doubling

        // Splitting is allowed if:
        // Hands is doubles AND max splits not reached AND (first split OR hand isn't aces OR aces can be resplit)
        let can_split = matches!(hand, Hand::Doubles(_)) &&
                        self.splits + 1 < self.ctx.max_split_hands &&
                        (self.splits == 0 || hand.unpack() != 1 || self.ctx.rules.can_resplit_aces);
        let split = if can_split { Split.bitmap() } else { 0 };

        // Doubling is allowed if:
        // Hand is not a split OR split hands can be doubled on
        let can_double = self.splits == 0 || self.ctx.rules.can_double_after_split;
        let double = if can_double { Double.bitmap() } else { 0 };

        // Other actions always allowed.
        // Hitting split aces is handled in the split() method since those hands may end immediately,
        // so no handling needed here.
        double | split | Hit | Stand | self.ctx.surr_flag
    }

    /// Draws a card, allowing the player to see it
    fn draw(&mut self) -> u8 {
        let card = self.ctx.shoe.draw();
        self.ctx.player.observe_card(card);
        card
    }

    /// Payout for a given player hand. It is assumed that the value of the player hand is <= 21.
    fn hand_payout(&self, (player, bet): HandBet, dealer: u8) -> i64 {
        if dealer > 21 || player > dealer {
            debug_println!("You win!");
            2 * bet
        } else if player == dealer && !self.ctx.rules.dealer_wins_ties {
            debug_println!("Tie");
            bet
        } else {
            debug_println!("You lost :(");
            0
        }
    }
}

