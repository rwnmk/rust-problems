// ===========================================================================
// 034 · Median and Mode
// Chapter 8: Common Collections
// Difficulty: ★☆☆☆  warm-up    Source: Rust Book ch. 8 suggestion
// FENCE: Use only what chapters 1-8 cover.
//        Not yet: `?` (use match/expect), generics, traits, closures.
// ===========================================================================
//
// TASK
// For a Vec<i32>, write functions returning:
// - the median as Option<f64> (even length: mean of the middle two)
// - the mode(s) as Vec<i32>: every value sharing the highest count, sorted
// - the mean and the range, as Options
// Do NOT mutate the caller's vector. Use a HashMap for counting.
// Test with vec![3, 7, 7, 2, 9, 2, 5], vec![1], vec![] and vec![4, 4, 1, 1, 9].
//
// CHECK YOUR UNDERSTANDING
// 1. To avoid mutating the input you had to sort a copy. How, and what did it
//    cost?
// 2. Does HashMap iteration order affect your mode output? Make sure it can't.
// 3. What type does `.entry(k).or_insert(0)` return, and why `*count += 1`?
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
