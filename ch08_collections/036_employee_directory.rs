// ===========================================================================
// 036 · Employee Directory
// Chapter 8: Common Collections
// Difficulty: ★★☆☆  core    Source: Rust Book ch. 8 suggestion
// FENCE: Use only what chapters 1-8 cover.
//        Not yet: `?` (use match/expect), generics, traits, closures.
// ===========================================================================
//
// TASK
// A text interface:
//   Add Sally to Engineering
//   Add Mary Jane to Customer Success
//   List Engineering          (names sorted)
//   List                      (all departments sorted, each with sorted names)
//   Remove Sally from Engineering
//   Quit
// Store data in HashMap<String, Vec<String>>. Names and departments can be
// several words - "to" and "from" are the separators. Unknown commands get a
// helpful message. Removing someone who isn't there says so.
//
// CHECK YOUR UNDERSTANDING
// 1. Why does the map own String keys rather than holding &str?
// 2. How did you look up a department and then modify its Vec without
//    fighting the borrow checker?
// 3. You sort on every List. What would BTreeMap (look it up) buy you?
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
