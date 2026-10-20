// ===========================================================================
// 011 · Square Roots by Newton's Method
// Chapter 3: Common Programming Concepts
// Difficulty: ★★☆☆  core    Source: SICP §1.1.7, Exercises 1.7 and 1.8
// FENCE: Use only what chapters 1-3 cover.
//        Not yet: String manipulation, Vec, structs, enums/Option, closures, iterator adapters.
// ===========================================================================
//
// TASK
// 1. `fn sqrt(x: f64) -> f64`: start with guess = 1.0 and repeatedly improve
//    it to (guess + x / guess) / 2, stopping when |guess * guess - x| < 0.001.
// 2. Try it on 2.0, 0.0001, 1e-10 and then a huge number such as 1e50 (have
//    Ctrl-C ready). Explain each result.
// 3. Fix it: stop when the guess changes by only a tiny FRACTION of itself
//    between iterations.
// 4. Cube root by the same method: improve y to (x / (y * y) + 2.0 * y) / 3.0.
// 5. Print the iteration count for each input.
//
// RULES
// No f64::sqrt or f64::cbrt, except to compare against at the end.
//
// CHECK YOUR UNDERSTANDING
// 1. Why does the absolute tolerance fail for tiny numbers? For huge ones?
// 2. Why does a relative test fix both?
// 3. Your loop has no fixed iteration count. How do you know it terminates?
//    Is it actually guaranteed?
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
