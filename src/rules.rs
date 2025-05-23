
use std::env::args;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(about)]
pub struct Rules {
    #[arg(short, long, default_value_t=6)]
    pub n_decks: usize,

    #[arg(short, long, default_value_t=0.75)]
    pub penetration: f32,

    #[arg(short, long, default_value_t=1.5)]
    pub blackjack_pays: f32,

    #[arg(short, long, default_value_t=10_000_000)]
    pub report_every: u32,

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

    #[arg(short = 't', long)]
    pub dealer_wins_ties: bool,
}

impl Default for Rules {
    fn default() -> Self {
        Self {
            n_decks: 6,
            penetration: 0.75,
            blackjack_pays: 1.5,
            report_every: 10_000_000,
            dealer_hits_soft_17: true,
            can_surrender: false,
            can_hit_split_aces: false,
            can_resplit_aces: true,
            can_double_after_split: true,
            max_split_hands: Some(4),
            player_counts_cards: false,
            dealer_wins_ties: false,
         }
    }
}

impl Rules {
    pub fn parse_or_default() -> Self {
        if args().len() < 2 {
            Self::default()
        } else {
            Self::parse()
        }
    }
}