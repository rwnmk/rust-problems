// ===========================================================================
// 068 · Expression Trees
// Chapter 15: Smart Pointers
// Difficulty: ★★☆☆  core    Source: SICP §2.3
// FENCE: Use only what chapters 1-15 cover.
//        Not yet: threads, async, dyn trait objects (beyond Box<dyn Error>).
// ===========================================================================
//
// TASK
//   enum Expr {
//       Num(f64), Var(String), Neg(Box<Expr>),
//       Add(Box<Expr>, Box<Expr>), Sub(Box<Expr>, Box<Expr>),
//       Mul(Box<Expr>, Box<Expr>), Div(Box<Expr>, Box<Expr>),
//       Pow(Box<Expr>, i32),
//   }
// - Helper constructors so trees aren't painful to build, e.g.
//   `fn add(a: Expr, b: Expr) -> Expr`.
// - `fn eval(&self, env: &HashMap<String, f64>) -> Result<f64, String>`
//   (unknown variables, division by zero).
// - Display with MINIMAL parentheses, respecting precedence and
//   associativity: a - (b - c) needs them, (a - b) - c doesn't.
// - `count_nodes`, `depth`.
//
// CHECK YOUR UNDERSTANDING
// 1. Why does Expr need Box? What is size_of::<Expr>(), and why that number?
// 2. How did you decide when a child needs parentheses? Which case was
//    hardest?
//
// ===========================================================================
// MY ANSWERS (write them before asking Claude to review)
// 1.
// 2.
// ===========================================================================

fn main() {
    // Your code here.
}
