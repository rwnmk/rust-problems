// ===========================================================================
// 048 · Display Everything
// Chapter 10: Generic Types, Traits, and Lifetimes
// Difficulty: ★☆☆☆  warm-up    Source: Original
// FENCE: Use only what chapters 1-10 cover.
//        Not yet: closures, iterator adapters, Box, dyn trait objects.
// ===========================================================================
//
// TASK
// Implement std::fmt::Display for:
// - Rational (023): "3/4", "-2", "0".
// - `Matrix2 { a, b, c, d: f64 }` printed as two aligned rows, honouring the
//   caller's precision: `{:.2}` should give two decimal places (f.precision()).
// - Card (029): e.g. "Q♠".
// Then `trait Summary { fn author(&self) -> String; fn summarise(&self) ->
// String { ... } }` with a default summarise that calls author, implemented
// for two types, overriding it in one.
//
// CHECK YOUR UNDERSTANDING
// 1. You never implemented to_string(), yet it works. Why?
// 2. How did you honour {:.2}?
// 3. Display vs Debug: who is each one for?
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
