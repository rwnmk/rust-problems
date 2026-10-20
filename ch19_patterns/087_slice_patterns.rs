// ===========================================================================
// 087 · Slice Patterns
// Chapter 19: Patterns and Matching
// Difficulty: ★★☆☆  core    Source: Original
// FENCE: Use only what chapters 1-19 cover.
//        Std only.
// ===========================================================================
//
// TASK
// Without indexing, using slice patterns:
// - `fn sum(xs: &[i32]) -> i32`, recursively: [] and [first, rest @ ..].
// - `fn is_palindrome(xs: &[char]) -> bool` via [first, middle @ .., last].
// - `fn describe(xs: &[i32]) -> String`: "empty", "one: 5", "two: 1 and 2",
//   "starts 1, ends 9 (7 items)".
// - Redo the tic-tac-toe win check (030) by matching each line as a
//   three-element array pattern with a guard.
// - `fn parse_command(words: &[&str]) -> Result<Cmd, String>`: ["go", dir] and
//   ["go", "to", dir] -> Go(dir); ["take", item @ ..] -> Take(item.join(" "));
//   ["quit"] -> Quit; anything else is an error.
// - Use let-else at least twice.
//
// CHECK YOUR UNDERSTANDING
// 1. Matching [first, rest @ ..] on a &[i32], what are the types of first and
//    rest? (Look up "default binding modes".)
// 2. Can you match a Vec directly against a slice pattern? What did you do?
//
// ===========================================================================
// MY ANSWERS (write them before asking Claude to review)
// 1.
// 2.
// ===========================================================================

fn main() {
    // Your code here.
}
