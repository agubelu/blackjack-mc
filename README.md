MonteCarlo simulator to determine house/player edges in BlackJack under any arbitrary set of rules.

The ruleset can be configured using the following flags:

- `--n-decks`
    - Number of decks to use for the shoe (default 6).
- `--penetration`
    - Percentage of the shoe that can be used before a reshuffle (default 0.75).
- `--blackjack-pays`
    - Payout for a player blackjack (default 1.5).
- `--report-every`
    - Number of rounds to play before displaying statistics (default 10M)
- `--dealer-hits-soft-17`
    - Whether the dealer hits on a soft 17 hand. Otherwise the dealer stands on all 17s.
- `--can-surrender`
    - Allow the player to surrender their hand and get half the bet back.
- `--can-hit-split-aces`
    - Allow the player to continue hitting after splitting two aces. Otherwise, each ace gets an additional card and the round ends immediately.
- `--can-resplit-aces`
    - Allow the player to resplit two aces after a previous ace split.
- `--can-double-after-split`
    - Allow the player to double down after splitting a hand.
- `--max-split-hands`
    - Maximum number of hands the player can have as a result of splitting. If unset, there is no limit.
- `--dealer-wins-ties`
    - The player loses their bet if the result is a tie.
- `--player-counts-cards`
    - Allow the player to count cards and adjust their bet accordingly. The player uses the Hi-Lo counting system and the bet increases linearly with true count.

If no flags are provided, the default ruleset is:
- 6 decks with 75% penetration
- Blackjack pays 3 to 2
- Dealer hits soft 17
- No surrendering allowed
- Player cannot hit split aces
- Player can re-split aces
- Player can double down after a split
- Max. 4 split hands
