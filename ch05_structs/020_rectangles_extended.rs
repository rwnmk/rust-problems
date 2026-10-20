// ===========================================================================
// 020 · Rectangles, Extended
// Chapter 5: Using Structs
// Difficulty: ★☆☆☆  warm-up    Source: Rust Book ch. 5 extension
// FENCE: Use only what chapters 1-5 cover.
//        Not yet: enums/Option, Vec, HashMap, traits beyond #[derive], closures.
// ===========================================================================
//
// TASK
// Starting from the Book's Rectangle, add:
// - `fn square(size: u32) -> Self` (associated function)
// - `perimeter`, `is_square`
// - `can_hold(&self, other: &Rectangle) -> bool` - but now the other rectangle
//   may be ROTATED 90 degrees to fit
// - `fn scale(&mut self, k: u32)`
// - `fn rotated(&self) -> Rectangle` AND `fn into_rotated(self) -> Rectangle`
// - `fn side_by_side(a: &Rectangle, b: &Rectangle) -> Rectangle` - the smallest
//   rectangle holding both placed side by side (try both orientations of each)
// Use dbg! at least once and notice where its output goes.
//
// CHECK YOUR UNDERSTANDING
// 1. &self vs &mut self vs self: what does each promise the caller? When is
//    taking `self` by value the right call?
// 2. `rect.area()` works though area takes &self. Who added the `&`?
// 3. Where does dbg! print, and what does it return?
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
