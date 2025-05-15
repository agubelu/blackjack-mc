mod game;
mod players;
mod rules;
mod sim;

pub use game::hand::Hand;
pub use game::action::Action;
pub use game::shoe::Shoe;
pub use players::player::Player;
pub use rules::Rules;
pub use sim::Sim;

fn main() {
    let x = 8u8.trailing_zeros();
}
