// ===========================================================================
// 029 · Who Wins the Trick?
// Chapter 6: Enums and Pattern Matching
// Difficulty: ★★☆☆  core    Source: Whist rules
// FENCE: Use only what chapters 1-6 cover.
//        Not yet: Vec, HashMap, `?`, generics, traits, closures.
// ===========================================================================
//
// TASK
// - `enum Suit`, a rank type (your choice of design - see question 1), and
//   `struct Card { rank, suit }`.
// - `fn beats(challenger: Card, current: Card, led: Suit,
//   trumps: Option<Suit>) -> bool`: a trump beats any non-trump; otherwise a
//   card can only win by following the led suit; within a suit, higher wins.
// - `fn trick_winner(cards: &[Card; 4], trumps: Option<Suit>) -> usize`
//   (cards[0] leads).
// - Hand-work 8 scenarios, including no-trumps tricks and someone trumping in,
//   and print expected vs actual for each.
//
// CHECK YOUR UNDERSTANDING
// 1. Did you use 13 rank variants, or something like Number(u8) plus faces?
//    What invalid values can each design represent?
// 2. trumps is Option<Suit>. Argue for and against a Trumps enum with a
//    NoTrumps variant instead.
//
// ===========================================================================
// MY ANSWERS (write them before asking Claude to review)
// 1.
// 2.
// ===========================================================================

fn main() {
    // Your code here.
}
