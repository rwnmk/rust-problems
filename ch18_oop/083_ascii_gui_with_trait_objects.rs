// ===========================================================================
// 083 · ASCII GUI with Trait Objects
// Chapter 18: Object-Oriented Programming Features
// Difficulty: ★☆☆☆  warm-up    Source: Rust Book ch. 18 extension
// FENCE: Use only what chapters 1-18 cover.
//        Std only.
// ===========================================================================
//
// TASK
//   trait Draw { fn draw(&self, canvas: &mut Canvas, x: usize, y: usize); }
// - Canvas: a fixed grid of chars with put(x, y, c) and render().
// - Components: Button { label }, TextBox { width, text }, Checkbox {
//   checked, label }, and Frame { children: Vec<(usize, usize, Box<dyn
//   Draw>)> } - a composite that draws a border and offsets its children.
// - Screen { components: Vec<(usize, usize, Box<dyn Draw>)> } with run().
// - Revisit 047: put Circles and Rects in a Vec<Box<dyn Shape>> and total
//   their area. Compare with your enum solution.
//
// CHECK YOUR UNDERSTANDING
// 1. Box<dyn Draw> vs <T: Draw>: static vs dynamic dispatch, and what each
//    costs.
// 2. What's in a vtable?
// 3. Enum vs trait object: which suits a closed set of types, and which an
//    open one?
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
