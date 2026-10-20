// ===========================================================================
// 053 · Roman Numerals, Test-First
// Chapter 11: Writing Automated Tests
// Difficulty: ★☆☆☆  warm-up    Source: LeetCode 12 and 13
// FENCE: Use only what chapters 1-11 cover.
//        Not yet: closures, iterator adapters (in your own code), external crates.
// ===========================================================================
//
// TASK
// 1. Write FAILING tests first for `to_roman(n: u32) -> Result<String, String>`
//    and `from_roman(s: &str) -> Result<u32, String>`: I, IV, IX, XL, XC, CD,
//    CM, MCMXCIV (1994), 3999, 0 and 4000 (errors), "IIII" and "IC" and ""
//    (errors), lowercase (you decide).
// 2. Make them pass.
// 3. A round-trip test over all of 1..=3999.
// 4. `fn to_roman_unchecked(n: u32) -> String` that panics out of range,
//    tested with #[should_panic(expected = "...")].
// 5. Run only the from_roman tests using a name filter. Run with
//    `-- --nocapture` and see what changes.
//
// CHECK YOUR UNDERSTANDING
// 1. How can you reject non-canonical numerals like "IIII" without a giant
//    list of rules? (You've already written something useful.)
// 2. What does `expected` match against, and why is it worth adding?
// 3. Why does cargo test hide println! output from passing tests?
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
