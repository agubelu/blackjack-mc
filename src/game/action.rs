#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Action {
    // Actions are sorted below in decreasing order of priority
    // to allow for strategies like "double if possible, otherwise hit"
    Surrender   = 0,
    Split       = 1, // The numeric values are used to construct action bitmaps,
    Double      = 2, // to inform the player of which actions they're allowed
    Hit         = 3, // to perform. The first action from the strategy book
    Stand       = 4, // that is allowed (according to enum order) will be chosen.
}
