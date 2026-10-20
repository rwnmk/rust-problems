// ===========================================================================
// 008 · The Twelve Days of Christmas
// Chapter 3: Common Programming Concepts
// Difficulty: ★★☆☆  core    Source: Rust Book ch. 3 suggestion
// FENCE: Use only what chapters 1-3 cover.
//        Not yet: String manipulation, Vec, structs, enums/Option, closures, iterator adapters.
// ===========================================================================
//
// TASK
// Print all twelve verses of the carol using loops and arrays rather than a
// pile of println!s.
// - The ordinals ("first", "second", ...) and the gifts live in two arrays of
//   string literals.
// - Each verse lists the gifts counting DOWN from the current day using an
//   inner loop over `(1..=day).rev()`.
// - Day one's last line is "A partridge in a pear tree"; on later days it's
//   "And a partridge in a pear tree".
//
// CHECK YOUR UNDERSTANDING
// 1. What's the full type of your gifts array, including its length?
// 2. What happens if you index it with 12? Is that caught at compile time or
//    at run time in your program, and why can't Rust always tell in advance?
// 3. Where did you need `usize` rather than i32, and why does Rust insist?
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
