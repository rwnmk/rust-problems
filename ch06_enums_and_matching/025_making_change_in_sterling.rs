// ===========================================================================
// 025 · Making Change in Sterling
// Chapter 6: Enums and Pattern Matching
// Difficulty: ★☆☆☆  warm-up    Source: Rust Book ch. 6 (UK edition)
// FENCE: Use only what chapters 1-6 cover.
//        Not yet: Vec, HashMap, `?`, generics, traits, closures.
// ===========================================================================
//
// TASK
// - `enum Coin { OnePence, TwoPence, FivePence, TenPence, TwentyPence,
//   FiftyPence(Design), OnePound, TwoPounds }` and
//   `enum Design { Standard, KewGardens, Olympic }`.
// - `fn value_in_pence(&self) -> u32` via match.
// - `fn collector_value(&self) -> u32` - same as face value, except a Kew
//   Gardens 50p is worth 5000p and an Olympic one 250p.
// - `fn largest_coin_not_exceeding(p: u32) -> Option<Coin>`.
// - `fn print_change(p: u32)` prints the coins a greedy cashier would hand
//   over, e.g. 388p -> £2, £1, 50p, 20p, 10p, 5p, 2p, 1p.
//
// CHECK YOUR UNDERSTANDING
// 1. Add a new Coin variant and "forget" one match. What happens, and why is
//    that valuable?
// 2. When would you use `_ =>`, and what do you give up?
// 3. Is greedy change-making always optimal? Find a coin system where it isn't
//    (try coins 1, 3, 4). Is it optimal for UK coins - how could you check?
//
// ===========================================================================
// MY ANSWERS (write them before asking Claude to review)
// 1.
// 2.
// 3.
// ===========================================================================

fn main() {
    // Your code here.
}
