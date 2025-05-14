use rand::rng;
use rand::seq::SliceRandom;
use std::iter;

pub struct Shoe {
    n_decks: usize,
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
        Self { n_decks, cutoff, cards, ix: 0 }
    }

    pub fn is_exhausted(&self) -> bool {
        self.ix >= self.cutoff
    }

    pub fn reshuffle(&mut self) {
        self.cards.shuffle(&mut rng());
        self.ix = 0;
    }

    pub fn draw(&mut self) -> u8 {
        // If penetration is too high and the shoe runs out of cards mid hand,
        // this panics and the casino will catch fire and blow up.
        let card = self.cards[self.ix];
        self.ix += 1;
        card
    }
}
