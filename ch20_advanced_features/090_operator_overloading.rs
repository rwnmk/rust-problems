// ===========================================================================
// 090 · Operator Overloading
// Chapter 20: Advanced Features
// Difficulty: ★★☆☆  core    Source: Rust Book ch. 20
// FENCE: Use only what chapters 1-20 cover.
//        The whole Book is fair game. Std only unless stated.
// ===========================================================================
//
// TASK
// - `Vec2 { x: f64, y: f64 }`: Add, Sub, Neg, AddAssign, Mul<f64> (scale),
//   Mul<Vec2> (dot product, Output = f64), and `impl Mul<Vec2> for f64` so
//   that 2.0 * v works.
// - `Matrix2`: Mul<Matrix2>, Mul<Vec2>, Index<(usize, usize)>, IndexMut.
// - Rational (023): Add, Sub, Mul, Div, Neg, also for references (&a + &b),
//   plus PartialOrd and Ord by cross-multiplication, and std::iter::Sum.
// - Rotate a vector 90 degrees with a matrix; sum a Vec<Rational> with .sum().
//
// CHECK YOUR UNDERSTANDING
// 1. Why can you write `impl Mul<Vec2> for f64` but not `impl Display for
//    Vec<Vec2>`?
// 2. Why is Rhs a generic parameter (with a default) while Output is an
//    associated type?
// 3. How did you support &a + &b without duplicating all the logic?
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
