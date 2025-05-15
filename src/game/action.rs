use std::ops::BitOr;
use Action::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum Action {
    // Actions are sorted below in decreasing order of priority
    // to allow for strategies like "double if possible, otherwise hit"
    Surrender   = 0,
    Split       = 1, // The numeric values are used to construct action bitmaps,
    Double      = 2, // to inform the player of which actions they're allowed
    Hit         = 3, // to perform. The first action from the strategy book
    Stand       = 4, // that is allowed (according to enum order) will be chosen.
}

impl Action {
    pub const fn bitmap(&self) -> u8 {
        1 << *self as usize
    }

    pub const fn from_bitmap(bitmap: u8) -> Self {
        match bitmap.trailing_zeros() {
            0 => Surrender,
            1 => Split,
            2 => Double,
            3 => Hit,
            4 => Stand,
            _ => unreachable!(),
        }
    }
}

impl BitOr<Self> for Action {
    type Output = u8;

    fn bitor(self, rhs: Self) -> Self::Output {
        self.bitmap() | rhs.bitmap()
    }
}

impl BitOr<u8> for Action {
    type Output = u8;

    fn bitor(self, rhs: u8) -> Self::Output {
        self.bitmap() | rhs
    }
}

impl BitOr<Action> for u8 {
    type Output = u8;

    fn bitor(self, rhs: Action) -> Self::Output {
        self | rhs.bitmap()
    }
}
