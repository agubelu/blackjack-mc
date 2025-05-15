use crate::{Player, Rules, Shoe};

pub struct Sim {
    pub player: Box<dyn Player>,
    pub dealer: Box<dyn Player>,
    pub shoe: Shoe,
    pub rules: Rules,
}