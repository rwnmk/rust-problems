// ===========================================================================
// 046 · Generic Extremes
// Chapter 10: Generic Types, Traits, and Lifetimes
// Difficulty: ★☆☆☆  warm-up    Source: Rust Book ch. 10 extension
// FENCE: Use only what chapters 1-10 cover.
//        Not yet: closures, iterator adapters, Box, dyn trait objects.
// ===========================================================================
//
// TASK
//   fn largest<T: PartialOrd>(xs: &[T]) -> Option<&T>
//   fn min_max<T: PartialOrd>(xs: &[T]) -> Option<(&T, &T)>   // one pass
//   fn largest_copy<T: PartialOrd + Copy>(xs: &[T]) -> Option<T>
//   struct Pair<T> { a: T, b: T }
//     - `fn swap(self) -> Pair<T>` for every T
//     - `fn larger(&self) -> &T` only when T: PartialOrd
// Test with i32, f64 (include f64::NAN, in different positions!), char, &str
// and String.
//
// CHECK YOUR UNDERSTANDING
// 1. What did NaN do to largest? Why does PartialOrd (not Ord) allow that?
// 2. Why does largest work for String while largest_copy doesn't?
// 3. What is monomorphisation? What does it cost and what does it save?
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
