mod game;
mod players;
mod rules;
mod sim;

use clap::Parser;
pub use game::hand::Hand;
pub use game::action::Action;
use game::round::Round;
pub use game::shoe::Shoe;
pub use players::player::Player;
pub use rules::Rules;
pub use sim::Sim;

fn main() {
    let mut sim = Sim::new(Rules::parse());

    let mut total_spent: i64 = 0;
    let mut net_gain: i64 = 0;
    let mut to_report = sim.rules.report_every;

    loop {
        if sim.shoe.is_exhausted() {
            sim.shoe.reshuffle();
            sim.player.reset_count();
        }

        let bet = sim.player.place_bet();
        total_spent += bet as i64;

        let mut round = Round::new(&mut sim);
        net_gain += round.play(bet) as i64;

        to_report -= 1;
        if to_report == 0 {
            to_report = sim.rules.report_every;
            let perc = net_gain as f32 / total_spent as f32 * 100.0;
            println!("Total spent: {total_spent}, net: {net_gain:+} ({perc:.2} %)");
        }
    }
}
