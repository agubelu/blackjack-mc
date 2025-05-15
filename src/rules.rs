use clap::Parser;

#[derive(Parser, Debug)]
#[command(about)]
pub struct Rules {
    #[arg(short, long, default_value_t=6)]
    pub n_decks: usize,

    #[arg(short, long, default_value_t=0.8)]
    pub penetration: f32,

    #[arg(short, long, default_value_t=1.5)]
    pub blackjack_pays: f32,

    #[arg(short = 's', long)]
    pub dealer_hits_soft_17: bool,

    #[arg(short = 'q', long)]
    pub can_surrender: bool,

    #[arg(short = 'a', long)]
    pub can_hit_split_aces: bool,

    #[arg(short = 'x', long)]
    pub can_resplit_aces: bool,

    #[arg(short = 'd', long)]
    pub can_double_after_split: bool,

    #[arg(short = 'm', long)]
    pub max_split_hands: Option<u32>,

    #[arg(short = 'c', long)]
    pub player_counts_cards: bool,

    #[arg(short = 'i', long)]
    pub interactive_play: bool,
}
