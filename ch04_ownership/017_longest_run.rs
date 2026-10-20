// ===========================================================================
// 017 · Longest Run
// Chapter 4: Understanding Ownership
// Difficulty: ★★☆☆  core    Source: Original
// FENCE: Use only what chapters 1-4 cover.
//        Not yet: structs, enums/Option, Vec, HashMap, closures, iterator adapters.
// ===========================================================================
//
// TASK
// `fn longest_run(s: &str) -> &str` returns a slice covering the longest run of
// identical consecutive bytes (assume ASCII for now). Ties go to the first.
//   "aaabbbbcc" -> "bbbb"     "abc" -> "a"     "" -> ""     "zzz" -> "zzz"
// Then `fn runs_report(s: &str)` prints each run and its length.
// Finally, call longest_run("ééé"). What happens? Don't fix it - explain it.
//
// CHECK YOUR UNDERSTANDING
// 1. Why did "ééé" panic, and on which line?
// 2. What would a correct definition of "a run of the same character" need to
//    be for arbitrary Unicode text? (Ch. 8 will come back to this.)
// 3. How many allocations does your longest_run make? Why does that matter?
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
