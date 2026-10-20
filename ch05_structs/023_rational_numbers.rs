// ===========================================================================
// 023 · Rational Numbers
// Chapter 5: Using Structs
// Difficulty: ★★☆☆  core    Source: SICP §2.1.1
// FENCE: Use only what chapters 1-5 cover.
//        Not yet: enums/Option, Vec, HashMap, traits beyond #[derive], closures.
// ===========================================================================
//
// TASK
// `struct Rational { num: i64, den: i64 }` with the invariants: den > 0 and the
// fraction is fully reduced.
// - `Rational::new(n, d)` normalises the sign and reduces with your own
//   `fn gcd(a: i64, b: i64) -> i64` (Euclid). Panic if d == 0 for now.
// - add, sub, mul, div returning new Rationals; `to_f64`; `fn show(&self) ->
//   String` giving "3/4", "-2", "0".
// - Compute 1/1 + 1/2 + ... + 1/20 exactly.
// - Keep going: find the first n where your harmonic sum overflows (debug
//   build) and explain why it overflows earlier than the final answer needs.
// Derive Debug, Clone, Copy and PartialEq (see the Book's Appendix C).
//
// CHECK YOUR UNDERSTANDING
// 1. Why does keeping the invariant make the DERIVED PartialEq correct? Give
//    an example that breaks without it.
// 2. Should add take self or &self? Does it matter once the type is Copy?
// 3. Where did overflow first bite? How could you delay it (hint: when do you
//    reduce)?
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
