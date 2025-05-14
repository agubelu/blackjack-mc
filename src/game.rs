use std::array;

use Action::*;
use crate::{Hand, Sim};
use debug_print::{debug_println, debug_print};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Action {
    Hit,
    Stand,
    Double,
    Split,
    Surrender,
}

pub struct Game<'a> {
    ctx: &'a mut Sim,
    player_hand: Hand,
    dealer_hand: Hand,
    dealer_upcard: u8,
    bet: i32,
    spent: i32,
}

// Final value of a hand and its associated bet before the dealer starts drawing
type HandBet = (u8, i32);

impl<'a> Game<'a> {
    pub fn new(sim: &'a mut Sim, bet: i32) -> Self {
        // Start a new round drawing cards from the shoe
        let drawn: [u8; 4] = array::from_fn(|_| sim.shoe.draw());
        let player_hand = Hand::from_cards(drawn[0], drawn[2]);
        let dealer_hand = Hand::from_cards(drawn[1], drawn[3]);

        // We can let the player observe the hole card now because the bet has already been made,
        // and it will be revealed anyways at the end of the round, so that logic is simplified.
        drawn.iter().for_each(|&c| sim.player.observe_card(c));

        Self { ctx: sim, player_hand, dealer_hand, bet, dealer_upcard: drawn[1], spent: bet }
    }

    /// Simulates a round of play and returns how much net money the player earned or lost.
    /// 0 => player got his bet back, positive or negative values indicate gain or loss.
    pub fn play(&mut self) -> i32 {
        // Check for blackjacks
        let player_val = self.player_hand.value();
        let dealer_val = self.dealer_hand.value();

        if dealer_val == 21 {
            debug_println!("Dealer blackjack");
            return if player_val == 21 { 0 } else { -self.bet };
        }

        if player_val == 21 {
            // Dealer doesn't have BJ, player wins
            debug_println!("Player blackjack");
            return (self.bet as f32 * 1.5) as i32;
        }

        // Game's on
        let player_hands = self.get_player_bets(self.player_hand, 0);
        if player_hands.is_empty() {
            debug_println!("You busted!");
            return -self.spent; // Player busted
        }

        // Player has some hands alive, simulate dealer behavior
        while self.dealer_hand.value() <= 21 {
            let action = self.ctx.dealer.decide(self.dealer_hand, 0, false);
            debug_println!("Dealer has: {:?} -> {action:?}", self.dealer_hand);
            match action {
                Hit => {
                    let card = self.ctx.shoe.draw();
                    debug_println!("Dealer drew {card}");
                    self.ctx.player.observe_card(card);
                    self.dealer_hand = self.dealer_hand.add_card(card);
                }
                Stand => break,
                Double | Split | Surrender => panic!("Suspicious dealer behavior."),
            }
        }

        let dealer_hand = self.dealer_hand.value();
        let payout: i32 = player_hands.into_iter().map(|x| hand_payout(x, dealer_hand)).sum();
        payout - self.spent
    }

    /// Simulates player actions for a given initial hand and returns the final hands with their bets.
    /// Only hands that haven't busted are included in the results.
    /// Generaly only one hand is returned, but it could be more if hands are (recursively) split.
    fn get_player_bets(&mut self, mut hand: Hand, depth: usize) -> Vec<HandBet> {
        let mut first = true;
        let mut bet = self.bet;
        let indent = " ".repeat(depth * 4);

        while hand.value() <= 21 {
            debug_println!("{indent}Your hand: {hand:?}, dealer card: {}", self.dealer_upcard);
            debug_print!("{indent}");
            let action = self.ctx.player.decide(hand, self.dealer_upcard, first);
            first = false;

            match action {
                Hit | Double => {
                    let card = self.ctx.shoe.draw();
                    debug_println!("{indent}You drew: {card}");
                    self.ctx.player.observe_card(card);
                    hand = hand.add_card(card);

                    if action == Double {
                        self.spent += self.bet;
                        bet *= 2;
                        break;
                    }
                },
                Split => {
                    debug_assert!(matches!(hand, Hand::Doubles(_)));
                    self.spent += self.bet;
                    let spl = hand.unpack();

                    // Fun fact: 21's after splitting aren't considered blackjacks,
                    // so we don't have to handle that edge case.
                    let card_r = self.ctx.shoe.draw();
                    debug_println!("{indent}(R) You drew: {card_r}");
                    self.ctx.player.observe_card(card_r);
                    let r = self.get_player_bets(Hand::from_cards(spl, card_r), depth + 1);

                    let card_l = self.ctx.shoe.draw();
                    debug_println!("{indent}(L) You drew: {card_l}");
                    self.ctx.player.observe_card(card_l);
                    let l = self.get_player_bets(Hand::from_cards(spl, card_l), depth + 1);

                    return [r, l].concat();
                },
                Surrender => {
                    // Half the bet is refunded; hand ends
                    self.spent -= self.bet / 2;
                    return vec![];
                },
                Stand => break,
            }
        }

        let hand_value = hand.value();
        if hand_value <= 21 { vec![(hand_value, bet)] } else { vec![] }
    }
}

/// Payout for a given player hand. It is assumed that the value of the player hand is <= 21.
fn hand_payout((player, bet): HandBet, dealer: u8) -> i32 {
    if dealer > 21 || player > dealer {
        debug_println!("You win!");
        2 * bet
    } else if player == dealer {
        debug_println!("Tie");
        bet
    } else {
        debug_println!("You lost :(");
        0
    }
}
