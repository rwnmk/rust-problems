// ===========================================================================
// 016 · Three Ways to Shout
// Chapter 4: Understanding Ownership
// Difficulty: ★☆☆☆  warm-up    Source: Original
// FENCE: Use only what chapters 1-4 cover.
//        Not yet: structs, enums/Option, Vec, HashMap, closures, iterator adapters.
// ===========================================================================
//
// TASK
// Write three versions of "shout" (uppercase it, append "!"):
//   fn shout_owned(s: String) -> String
//   fn shout_borrowed(s: &str) -> String
//   fn shout_in_place(s: &mut String)
// In main, call each, then try to use the original variable afterwards. At
// every call site write a comment saying who owns the data before and after.
//
// Then write `fn longer_owned(a: String, b: String) -> String` that returns the
// longer of two Strings, and explain what happens to the other one.
// `.to_uppercase()`, `.push()` and `.push_str()` are allowed.
//
// CHECK YOUR UNDERSTANDING
// 1. Which version would you choose for a library API, and why?
// 2. How did you replace the contents in shout_in_place? What happens to the
//    old heap buffer?
// 3. In longer_owned, exactly when is the losing String's memory freed?
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
