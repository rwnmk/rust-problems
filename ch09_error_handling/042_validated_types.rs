// ===========================================================================
// 042 · Validated Types
// Chapter 9: Error Handling
// Difficulty: ★☆☆☆  warm-up    Source: Rust Book ch. 9 (Guess) extension
// FENCE: Use only what chapters 1-9 cover.
//        Not yet: generics, your own traits, closures (.map_err etc.).
// ===========================================================================
//
// TASK
// In the style of the Book's Guess type:
// - `Percentage` (0..=100): `new(u8) -> Result<Percentage, String>`, `value()`.
// - `Email` wrapping a String: exactly one '@', non-empty local part, a '.'
//   somewhere in the domain. It must be IMPOSSIBLE to construct unchecked from
//   outside its module.
// - `NonEmptyName`, which also trims.
// Then for each scenario, decide panic! or Result and justify it in a comment:
//   a) an index "that can't be out of range" in your own private helper
//   b) a user typed a malformed email
//   c) a config file embedded in your binary with include_str! fails to parse
//   d) a network request fails
//   e) a library function gets a negative length from its caller
//   f) a quick prototype to explore an idea
//
// CHECK YOUR UNDERSTANDING
// 1. What stops other code writing `Email { address: ... }` directly? Where
//    must Email live for that to work?
// 2. Why is a validated type better than re-validating in every function that
//    uses the value?
//
// ===========================================================================
// MY ANSWERS (write them before asking Claude to review)
// 1.
// 2.
// ===========================================================================

fn main() {
    // Your code here.
}
