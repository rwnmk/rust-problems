// ===========================================================================
// 069 · Symbolic Differentiation
// Chapter 15: Smart Pointers
// Difficulty: ★★★☆  challenging    Source: SICP §2.3.2
// FENCE: Use only what chapters 1-15 cover.
//        Not yet: threads, async, dyn trait objects (beyond Box<dyn Error>).
// ===========================================================================
//
// TASK
// Using Expr from 068:
// - `fn deriv(e: &Expr, var: &str) -> Expr`: sums, products, quotients,
//   powers with constant exponents, and the chain rule.
// - `fn simplify(e: Expr) -> Expr`: 0 + x -> x, 1 * x -> x, 0 * x -> 0,
//   constant folding, x - x -> 0, ... applied until nothing changes.
// - d/dx of x^3 + 3*x*y + y^2 should simplify to 3 * x^2 + 3 * y.
// - Check numerically: compare eval(deriv(e)) with a finite difference
//   (f(x + h) - f(x - h)) / 2h at a few points.
//
// CHECK YOUR UNDERSTANDING
// 1. Does simplify take Expr by value or by reference? How did that affect
//    how much cloning you did?
// 2. How do you know simplify terminates?
// 3. Why is a canonical form for algebraic expressions hard in general?
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
