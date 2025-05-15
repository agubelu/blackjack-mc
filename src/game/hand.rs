use Hand::*;

#[derive(Debug, Copy, Clone)]
pub enum Hand {
    Soft(u8),
    Hard(u8),
    Doubles(u8),
}

impl Hand {
    pub fn from_cards(c1: u8, c2: u8) -> Self {
        match (c1, c2) {
            _ if c1 == c2 => Doubles(c1),
            (1, _) | (_, 1) => Soft(c1 + c2 + 10),
            _ => Hard(c1 + c2),
        }
    }

    pub fn value(&self) -> u8 {
        match self {
            Soft(x) => *x,
            Hard(x) => *x,
            Doubles(1) => 12, // edge case because AA == soft 12
            Doubles(x) => 2 * x,
        }
    }

    pub fn unpack(&self) -> u8 {
        match self {
            Soft(x) => *x,
            Hard(x) => *x,
            Doubles(x) => *x,
        }
    }

    pub fn add_card(&self, new_card: u8) -> Self {
        // Hard -> Hard: if new != A
        // Hard -> Soft: 2-10 + A (soft 12 == AA)
        // Soft -> Soft: if (old + new) <= 21
        // Soft -> Hard: if (old + new) > 21
        // All doubles except AA are hard

        let val = self.value();
        let new_val = val + new_card;

        match self {
            Soft(_) | Doubles(1) => {
                if new_val <= 21 { Soft(new_val) } else { Hard(new_val - 10) }
            }
            Hard(_) | Doubles(_) => {
                if new_card == 1 && val <= 10 { Soft(new_val + 10) } else { Hard(new_val) }
            }
        }
    }
}
