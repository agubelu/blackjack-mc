use crate::{Player, Shoe};

pub struct Sim {
    pub player: Box<dyn Player>,
    pub dealer: Box<dyn Player>,
    pub shoe: Shoe,
}