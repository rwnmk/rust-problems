// ===========================================================================
// 006 · Temperature Tables
// Chapter 3: Common Programming Concepts
// Difficulty: ★☆☆☆  warm-up    Source: Rust Book ch. 3 suggestion; K&R §1.2
// FENCE: Use only what chapters 1-3 cover.
//        Not yet: String manipulation, Vec, structs, enums/Option, closures, iterator adapters.
// ===========================================================================
//
// TASK
// Write `fn f_to_c(f: f64) -> f64` and `fn c_to_f(c: f64) -> f64`.
// 1. Print the K&R table: Fahrenheit from 0 to 300 in steps of 20, with the
//    Celsius equivalent, columns right-aligned to one decimal place.
// 2. Print a Celsius -> Fahrenheit table from -40 to 100 in steps of 10.
// 3. For every row, check that c_to_f(f_to_c(f)) is within 1e-9 of f and print
//    OK or MISMATCH.
//
// RULES
// Step with `while` or `loop` (no `.step_by` - that's an iterator adapter).
// Format specs like {:>8.1} are fine (see the std::fmt docs).
//
// CHECK YOUR UNDERSTANDING
// 1. Why f64 and not integers? What does `(f - 32) * 5 / 9` give in i32 for
//    f = 33, and why?
// 2. Why compare floats with a tolerance rather than `==`?
// 3. Where do the two scales meet? Prove it algebraically.
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
