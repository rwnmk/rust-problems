// ===========================================================================
// 021 · Points, Segments, Units
// Chapter 5: Using Structs
// Difficulty: ★☆☆☆  warm-up    Source: Original
// FENCE: Use only what chapters 1-5 cover.
//        Not yet: enums/Option, Vec, HashMap, traits beyond #[derive], closures.
// ===========================================================================
//
// TASK
// - `struct Point { x: f64, y: f64 }` and `struct Segment { a: Point, b: Point }`
//   with methods length, midpoint, and `is_vertical`.
// - Tuple structs `Metres(f64)` and `Feet(f64)` with `Metres::to_feet(self)`
//   and `Feet::to_metres(self)`.
// - `fn add_metres(a: Metres, b: Metres) -> Metres` - confirm that passing Feet
//   is a compile error.
// - Use field init shorthand and struct update syntax at least once each.
// - Write `let p = seg.a;` and then use `seg` afterwards - first as-is, then
//   with #[derive(Clone, Copy)] on Point.
//
// CHECK YOUR UNDERSTANDING
// 1. What does Metres(f64) buy you over a plain f64? What does it cost at run
//    time?
// 2. What happened to `seg` after `let p = seg.a;` with and without Copy?
// 3. Is `..other` in struct update syntax a move or a copy here? Does it
//    depend on the fields?
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
