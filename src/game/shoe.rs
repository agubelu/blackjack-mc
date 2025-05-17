use std::iter;
use rand::rng;
use rand::seq::SliceRandom;

pub struct Shoe {
    cutoff: usize,
    cards: Vec<u8>,
    ix: usize,
}

impl Shoe {
    pub fn new(n_decks: usize, penetration: f32) -> Self {
        let cutoff = (n_decks as f32 * 52.0 * penetration) as usize;
        let mut cards = vec![];

        for i in 1..=10 {
            let reps = if i != 10 { 4 * n_decks } else { 16 * n_decks };
            cards.extend(iter::repeat_n(i, reps));
        }
        cards.shuffle(&mut rng());
        Self { cutoff, cards, ix: 0 }
    }

    pub fn is_exhausted(&self) -> bool {
        self.ix >= self.cutoff
    }

    pub fn reshuffle(&mut self) {
        self.cards.shuffle(&mut rng());
        self.ix = 0;
    }

    pub fn draw(&mut self) -> u8 {
        // The caller is responsible for re-shuffling the shoe when appropriate to avoid running out of cards.
        let card = self.cards[self.ix];
        self.ix += 1;
        card
    }
}
