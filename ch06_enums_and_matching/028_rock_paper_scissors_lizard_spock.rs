// ===========================================================================
// 028 · Rock Paper Scissors Lizard Spock
// Chapter 6: Enums and Pattern Matching
// Difficulty: ★☆☆☆  warm-up    Source: Original
// FENCE: Use only what chapters 1-6 cover.
//        Not yet: Vec, HashMap, `?`, generics, traits, closures.
// ===========================================================================
//
// TASK
// - `enum Move` (five variants) and `enum Outcome { Win, Lose, Draw }`.
// - `fn parse_move(s: &str) -> Option<Move>`, case-insensitive; accept full
//   names and unambiguous prefixes ("sc" for scissors, "sp" for Spock).
// - `fn play(a: Move, b: Move) -> Outcome` with ONE match on the tuple (a, b),
//   using as few arms as you can.
// - `fn verb(winner: Move, loser: Move) -> &'static str` ("cuts", "covers",
//   ...). Treat 'static as "a string literal baked into the binary" for now -
//   ch. 10 explains it.
// - Best-of-five against the computer (rand from ch. 2).
//
// CHECK YOUR UNDERSTANDING
// 1. How did you keep the arm count down? Is the compiler still checking
//    exhaustiveness for you?
// 2. Which of Clone, Copy, PartialEq did you derive, and which did you
//    actually need? Remove each and see.
//
// ===========================================================================
// MY ANSWERS (write them before asking Claude to review)
// 1.
// 2.
// ===========================================================================

fn main() {
    // Your code here.
}
