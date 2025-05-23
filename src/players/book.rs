
/*
    Taken from: https://en.wikipedia.org/wiki/Blackjack#Basic_strategy
    Each value is a bitmap of the preferred actions for each case,
    with the following values and decreasing order of priority:

    Surrender:  00001
    Split:      00010
    Double:     00100
    Hit:        01000
    Stand:      10000

    The stand bit is always set as a backup in case no other actions are available.
*/

use crate::Hand;

static STRATEGY: [u8; 360] = [
/*           Dealer's face up card          */
/*   A   2   3   4   5   6   7   8   9  10  */
// 0 -------------------------------------  // Hard hands
    24, 24, 24, 24, 24, 24, 24, 24, 24, 24, // 5
    24, 24, 24, 24, 24, 24, 24, 24, 24, 24, // 6
    24, 24, 24, 24, 24, 24, 24, 24, 24, 24, // 7
    24, 24, 24, 24, 24, 24, 24, 24, 24, 24, // 8
    24, 24, 28, 28, 28, 28, 24, 24, 24, 24, // 9
    24, 28, 28, 28, 28, 28, 28, 28, 28, 24, // 10
    28, 28, 28, 28, 28, 28, 28, 28, 28, 28, // 11
    24, 24, 24, 16, 16, 16, 24, 24, 24, 24, // 12
    24, 16, 16, 16, 16, 16, 24, 24, 24, 24, // 13
    24, 16, 16, 16, 16, 16, 24, 24, 24, 24, // 14
    25, 16, 16, 16, 16, 16, 24, 24, 24, 25, // 15
    25, 16, 16, 16, 16, 16, 24, 24, 25, 25, // 16
    17, 16, 16, 16, 16, 16, 16, 16, 16, 16, // 17
    16, 16, 16, 16, 16, 16, 16, 16, 16, 16, // 18
    16, 16, 16, 16, 16, 16, 16, 16, 16, 16, // 19
    16, 16, 16, 16, 16, 16, 16, 16, 16, 16, // 20
    16, 16, 16, 16, 16, 16, 16, 16, 16, 16, // 21
// 170 -----------------------------------  // Soft hands
    24, 24, 24, 24, 28, 28, 24, 24, 24, 24, // 13
    24, 24, 24, 24, 28, 28, 24, 24, 24, 24, // 14
    24, 24, 24, 28, 28, 28, 24, 24, 24, 24, // 15
    24, 24, 24, 28, 28, 28, 24, 24, 24, 24, // 16
    24, 24, 28, 28, 28, 28, 24, 24, 24, 24, // 17
    24, 20, 20, 20, 20, 20, 16, 16, 24, 24, // 18
    16, 16, 16, 16, 16, 20, 16, 16, 16, 16, // 19
    16, 16, 16, 16, 16, 16, 16, 16, 16, 16, // 20
    16, 16, 16, 16, 16, 16, 16, 16, 16, 16, // 21
// 260 -----------------------------------  // Doubles
    26, 26, 26, 26, 26, 30, 26, 26, 26, 26, // A-A
    24, 18, 18, 18, 18, 18, 18, 24, 24, 24, // 2-2
    24, 18, 18, 18, 18, 18, 18, 24, 24, 24, // 3-3
    24, 24, 24, 24, 18, 18, 24, 24, 24, 24, // 4-4
    24, 28, 28, 28, 28, 28, 28, 28, 28, 24, // 5-5
    24, 18, 18, 18, 18, 18, 24, 24, 24, 24, // 6-6
    24, 18, 18, 18, 18, 18, 18, 24, 24, 24, // 7-7
    19, 18, 18, 18, 18, 18, 18, 18, 18, 18, // 8-8
    16, 18, 18, 18, 18, 18, 16, 18, 18, 16, // 9-9
    16, 16, 16, 16, 16, 16, 16, 16, 16, 16, // 10-10
];

pub fn best_actions(hand: Hand, dealer: u8) -> u8 {
    match hand {
        Hand::Hard(x) => STRATEGY[(x as usize - 5) * 10 + dealer as usize - 1],
        Hand::Soft(x) => STRATEGY[170 + (x as usize - 13) * 10 + dealer as usize - 1],
        Hand::Doubles(x) => STRATEGY[260 + (x as usize - 1) * 10 + dealer as usize - 1],
    }
}
