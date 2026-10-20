// ===========================================================================
// 014 · Words as Slices
// Chapter 4: Understanding Ownership
// Difficulty: ★★☆☆  core    Source: Rust Book ch. 4 extension
// FENCE: Use only what chapters 1-4 cover.
//        Not yet: structs, enums/Option, Vec, HashMap, closures, iterator adapters.
// ===========================================================================
//
// TASK
// Using `as_bytes()` and indexing (no `split`, no `trim`, no iterator
// adapters), implement:
//   fn first_word(s: &str) -> &str
//   fn last_word(s: &str) -> &str
//   fn nth_word(s: &str, n: usize) -> &str   // "" if there is no nth word
//   fn word_count(s: &str) -> usize
//   fn my_trim(s: &str) -> &str              // spaces only
// Words are separated by one or more spaces; leading and trailing spaces are
// allowed. Test each in main with "hello world", "   padded   words  ", "",
// "one" and "   ".
//
// Finally, write code that calls first_word on a String, then calls `.clear()`
// on that String, then prints the word. Read the error and explain it in your
// own words.
//
// CHECK YOUR UNDERSTANDING
// 1. Why take &str rather than &String? What can callers now pass that they
//    couldn't otherwise?
// 2. Your functions return &str. Whose data does it point to, and why didn't
//    the compiler make you say so? (Ch. 10 gives this a name.)
// 3. What would go wrong if you returned indices (usize) instead of slices?
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
