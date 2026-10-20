// ===========================================================================
// 003 · Echo and Greet
// Chapter 2: Programming a Guessing Game
// Difficulty: ★☆☆☆  warm-up    Source: Rust Book ch. 2 style
// FENCE: Use only what chapters 1-2 cover.
//        Not yet: structs, enums of your own, Vec, closures.
// ===========================================================================
//
// TASK
// Ask for the user's name and age. Print a greeting and how old they will be
// in 10 years.
// - Trim the name; if it's empty, greet "stranger".
// - If the age isn't a valid whole number, say so and ask again (loop) rather
//   than crashing.
//
// EXAMPLE
//   What's your name? Ada
//   How old are you? twelve
//   That's not a number, try again.
//   How old are you? 36
//   Hello, Ada! In 10 years you'll be 46.
//
// RULES
// No `.unwrap()`. Use `match` on the result of `parse`.
//
// CHECK YOUR UNDERSTANDING
// 1. Why is `.trim()` needed before parsing? What exactly is in the String
//    after read_line returns?
// 2. What type did you choose for age? What happens if the user types -5 or
//    300 with that type? Try it.
// 3. Why does read_line take `&mut name` instead of returning a new String?
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
