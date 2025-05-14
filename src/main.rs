mod game;
mod hand;
mod players;
mod shoe;
mod sim;

pub use game::Action;
use game::Game;
pub use hand::Hand;
pub use players::player::Player;
pub use shoe::Shoe;
pub use sim::Sim;

fn main() {
    let mut shoe = Shoe::new(100, 0.9);
    let mut player = Box::new(players::interactive::InteractivePlayer{});
    let mut dealer = Box::new(players::dealer::Dealer{});
    let mut sim = Sim{player, dealer, shoe};

    loop {
        let mut game = Game::new(&mut sim, 10);
        let result = game.play();
        println!("Result: {result:+}");
        println!("--------------")
    }
}
