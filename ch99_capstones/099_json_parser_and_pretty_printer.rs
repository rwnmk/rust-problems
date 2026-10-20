// ===========================================================================
// 099 · JSON Parser and Pretty-Printer
// Capstone
// Difficulty: ★★★★  stretch    Source: RFC 8259
// FENCE: The whole Book. Everything. Std only unless stated.
// ===========================================================================
//
// TASK
// - enum Json { Null, Bool(bool), Number(f64), String(String),
//   Array(Vec<Json>), Object(Vec<(String, Json)>) } - why Vec of pairs rather
//   than a HashMap?
// - A recursive-descent parser over &str whose errors carry line:column and
//   what was expected.
// - Full string escapes, including \uXXXX and surrogate pairs.
// - Compact Display and a pretty-printer; round-trip: parse(print(x)) == x.
// - Path queries: get("users[2].name").
// - Try 10,000 levels of nested arrays. Then try the JSONTestSuite collection
//   if you want a gauntlet.
// - Benchmark a few-MB file against serde_json, just to see.
//
// CHECK YOUR UNDERSTANDING
// 1. What happened at 10,000 levels of nesting, and what's your fix?
// 2. With f64 numbers, what goes wrong with 12345678901234567890?
// 3. Where did you avoid allocating, and where couldn't you?
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
