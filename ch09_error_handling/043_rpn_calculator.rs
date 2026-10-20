// ===========================================================================
// 043 · RPN Calculator
// Chapter 9: Error Handling
// Difficulty: ★★☆☆  core    Source: K&R §4.3
// FENCE: Use only what chapters 1-9 cover.
//        Not yet: generics, your own traits, closures (.map_err etc.).
// ===========================================================================
//
// TASK
// Read lines like `3 4 + 2 *` and print 14. Support + - * / with f64s, plus
// `dup`, `swap`, `drop`, and `p` (print the top without popping).
//   #[derive(Debug)]
//   enum CalcError {
//       StackUnderflow { op: String },
//       DivisionByZero,
//       UnknownToken(String),
//       LeftoverValues(usize),
//   }
// - `fn eval_line(line: &str) -> Result<f64, CalcError>`: exactly one value
//   must be left at the end.
// - The REPL never crashes; print errors with {:?} for now.
// - Bonus: infix -> RPN with the shunting-yard algorithm, feeding your
//   evaluator.
//
// CHECK YOUR UNDERSTANDING
// 1. Where did you use `?`, and where did you have to match? Why?
// 2. Float division by zero doesn't panic. What does it give? Why might you
//    still treat it as an error?
// 3. From the user's side, what's the difference between Err and a panic in
//    a REPL?
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
