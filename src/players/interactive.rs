use crate::{Action, Hand, Player};
use std::io::{stdin, stdout, Read, Write};

pub struct InteractivePlayer;

impl Player for InteractivePlayer {
    fn observe_card(&mut self, _card: u8) { }

    fn reset_count(&mut self) { }

    fn decide(&self, _hand: Hand, _dealer: u8, _allowed: u8) -> Action {
        print!("Action? (h)it / (s)tand / (d)ouble / (q) surrender / (x) split: ");
        stdout().flush().unwrap();
        let mut input = [0u8, 0u8];
        stdin().read_exact(&mut input).unwrap();

        let action = match input[0] as char {
            'h' | 'H' => Action::Hit,
            's' | 'S' => Action::Stand,
            'd' | 'D' => Action::Double,
            'q' | 'Q' => Action::Surrender,
            'x' | 'X' => Action::Split,
             _  => panic!("what"),
        };

        println!();
        action
    }

    fn place_bet(&self) -> i64 { 10 }
}
