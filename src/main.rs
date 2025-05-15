mod game;
mod players;
mod rules;
mod sim;

use clap::Parser;
pub use game::hand::Hand;
pub use game::action::Action;
use game::round::Round;
pub use game::shoe::Shoe;
use players::dealer::Dealer;
pub use players::player::Player;
pub use rules::Rules;
pub use sim::Sim;

fn main() {
    let rules = Rules::parse();
    let player = Box::new(players::interactive::InteractivePlayer{});
    let dealer = Dealer{};
    let shoe = Shoe::new(rules.n_decks, rules.penetration);
    let surr_flag = if rules.can_surrender { Action::Surrender.bitmap() } else { 0 };
    let mut sim = Sim { rules, player, dealer, shoe, surr_flag };

    loop {
        let mut round = Round::new(&mut sim);
        let net = round.play(10);
        println!("Net: {net:+}");
        println!("----------------------");
    }
}
