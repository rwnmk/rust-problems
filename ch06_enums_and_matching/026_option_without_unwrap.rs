// ===========================================================================
// 026 · Option Without Unwrap
// Chapter 6: Enums and Pattern Matching
// Difficulty: ★☆☆☆  warm-up    Source: Original
// FENCE: Use only what chapters 1-6 cover.
//        Not yet: Vec, HashMap, `?`, generics, traits, closures.
// ===========================================================================
//
// TASK
//   fn checked_div(a: i32, b: i32) -> Option<i32>     // careful: i32::MIN / -1
//   fn max(xs: &[i32]) -> Option<i32>                 // revisit problem 015
//   fn index_of(xs: &[i32], target: i32) -> Option<usize>
//   fn parse_digit(c: char) -> Option<u32>            // no .to_digit()
// Then `fn max_over_first(xs: &[i32]) -> Option<i32>` (max divided by the first
// element) written THREE ways: nested match; if let / else; let-else.
//
// RULES
// No .unwrap(), no .expect(), no `?` (ch. 9), no .map/.and_then (closures are
// ch. 13).
//
// CHECK YOUR UNDERSTANDING
// 1. Why does i32::MIN / -1 need special care?
// 2. Compare your three versions. Which reads best, and when does let-else
//    shine?
// 3. When would you return Option<&T> rather than Option<T>?
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
