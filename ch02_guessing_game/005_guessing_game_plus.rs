// ===========================================================================
// 005 · Guessing Game Plus
// Chapter 2: Programming a Guessing Game
// Difficulty: ★★☆☆  core    Source: Rust Book ch. 2 extension
// FENCE: Use only what chapters 1-2 cover.
//        Not yet: structs, enums of your own, Vec, closures.
// ===========================================================================
//
// EXPERIMENT (do this first)
// Take your finished ch. 2 guessing game and move
// `let mut guess = String::new();` to ABOVE the loop so one String is reused.
// Predict what happens on the second guess, run it, and explain. Then fix it
// WITHOUT moving the line back.
//
// TASK
// Then add:
// 1. A limit of 7 attempts; reveal the number when you run out.
// 2. Invalid input (non-numbers, numbers outside 1..=100) prints a message and
//    does NOT use an attempt.
// 3. From the second guess on, also say "warmer" or "colder" compared with
//    your previous guess.
// 4. At the end ask "Play again? (y/n)" and loop.
//
// CHECK YOUR UNDERSTANDING
// 1. What did the experiment reveal about read_line? Which method fixed it?
// 2. In `let guess: u32 = match guess.trim().parse() { ... }`, how does Rust
//    know which type `parse` should produce?
// 3. What does `guess.cmp(&secret)` return, and why must the match on it handle
//    every variant?
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
