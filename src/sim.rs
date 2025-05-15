use crate::{Player, Rules, Shoe};
use crate::players::dealer::Dealer;

pub struct Sim {
    pub player: Box<dyn Player>,
    pub dealer: Dealer,
    pub shoe: Shoe,
    pub rules: Rules,
    pub surr_flag: u8, // Bitmap for whether surrender is available or not
}
