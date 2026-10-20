// ===========================================================================
// 092 · macro_rules!
// Chapter 20: Advanced Features
// Difficulty: ★★★☆  challenging    Source: Rust Book ch. 20
// FENCE: Use only what chapters 1-20 cover.
//        The whole Book is fair game. Std only unless stated.
// ===========================================================================
//
// TASK
// - `hashmap!{ "a" => 1, "b" => 2 }` - trailing comma and empty both work.
// - `max!(x)`, `max!(x, y, ...)` recursively.
// - `strings!["a", "b"]` producing a Vec<String>.
// - `assert_approx_eq!(a, b)` and `assert_approx_eq!(a, b, eps)` with failure
//   messages that include the source text of the expressions (stringify!).
// - `named_enum!{ enum Colour { Red, Green, Blue } }` generating the enum,
//   `fn name(&self) -> &'static str`, and `const ALL: [Colour; N]` with N
//   computed by the macro (that's the hard part).
// - Optional: view expansions with cargo-expand.
//
// CHECK YOUR UNDERSTANDING
// 1. $x:expr vs $x:tt - when did you need tt?
// 2. Does your max! evaluate any argument more than once? Why would that
//    matter?
// 3. What is macro hygiene? Show a case where it protected you.
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
