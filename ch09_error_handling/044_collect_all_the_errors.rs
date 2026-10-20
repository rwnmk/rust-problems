// ===========================================================================
// 044 · Collect All the Errors
// Chapter 9: Error Handling
// Difficulty: ★★☆☆  core    Source: Original
// FENCE: Use only what chapters 1-9 cover.
//        Not yet: generics, your own traits, closures (.map_err etc.).
// ===========================================================================
//
// TASK
// Input lines such as "12", "x7", "40", "", "999999999999":
// 1. `fn parse_first_error(lines: &[&str]) -> Result<Vec<i32>, String>` stops at
//    the first bad line; the message includes its line number and the reason.
// 2. `fn parse_all_errors(lines: &[&str]) -> Result<Vec<i32>, Vec<String>>`
//    reports EVERY bad line.
// 3. Distinguish "empty", "not a number" and "out of range" by matching on the
//    error's `.kind()` (std::num::IntErrorKind).
// 4. `fn checked_total(nums: &[i32]) -> Result<i32, String>`: overflow is an
//    error (checked_add gives an Option - convert it).
// 5. Chain them in main with `?`.
//
// RULES
// match, if let and `?` only - no closures (.map_err and friends are ch. 13).
//
// CHECK YOUR UNDERSTANDING
// 1. When is fail-fast better than collecting every error, and vice versa?
// 2. How did you turn an Option into a Result without a closure?
// 3. What does IntErrorKind tell you that the Display message doesn't?
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
