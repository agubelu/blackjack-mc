mod game;
mod players;
mod rules;
mod sim;

pub use game::{action::Action, hand::Hand, round::Round, shoe::Shoe};
pub use players::player::Player;
pub use rules::Rules;
pub use sim::Sim;

fn main() {
    let mut sim = Sim::new(Rules::parse_or_default());

    let mut total_spent = 0;
    let mut net_gain = 0;
    let mut i = sim.rules.report_every;

    loop {
        if sim.shoe.is_exhausted() {
            sim.shoe.reshuffle();
            sim.player.reset_count();
        }

        let bet = sim.player.place_bet();
        total_spent += bet;

        let mut round = Round::new(&mut sim);
        net_gain += round.play(bet);

        i -= 1;
        if i == 0 {
            i = sim.rules.report_every;
            let perc = net_gain as f32 / total_spent as f32 * 100.0;
            println!("Total spent: {total_spent}, net: {net_gain:+} ({perc:.2} %)");
        }
    }
}
