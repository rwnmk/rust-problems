// ===========================================================================
// 047 · A Shape Trait
// Chapter 10: Generic Types, Traits, and Lifetimes
// Difficulty: ★★☆☆  core    Source: Original
// FENCE: Use only what chapters 1-10 cover.
//        Not yet: closures, iterator adapters, Box, dyn trait objects.
// ===========================================================================
//
// TASK
//   trait Shape {
//       fn area(&self) -> f64;
//       fn perimeter(&self) -> f64;
//       fn name(&self) -> String { /* default */ }
//       fn describe(&self) -> String { /* default using the others */ }
//   }
// - Implement it for Circle, Rect and Triangle; override name in at least one.
// - `fn total_area<T: Shape>(shapes: &[T]) -> f64`
// - `fn print_bigger(a: &impl Shape, b: &impl Shape)`
// - Try calling total_area on a slice holding a Circle AND a Rect. Record the
//   error. Without `dyn` (ch. 18), find two ways to total a mixed set.
//
// CHECK YOUR UNDERSTANDING
// 1. Why can't a &[T] hold both Circles and Rects?
// 2. With two arguments, how does `a: &impl Shape, b: &impl Shape` differ from
//    `<T: Shape>(a: &T, b: &T)`?
// 3. Could you implement Shape for f64? Display for Vec<Circle>? State the
//    rule.
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
