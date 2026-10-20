// ===========================================================================
// 065 · Look-and-Say
// Chapter 13: Iterators and Closures
// Difficulty: ★★★☆  challenging    Source: Conway's look-and-say sequence
// FENCE: Use only what chapters 1-13 cover.
//        Not yet: Box/Rc/RefCell, threads, dyn trait objects.
// ===========================================================================
//
// TASK
// 1. Run-length encoding: `fn encode(s: &str) -> String` ("aaabcc" ->
//    "3a1b2c") and `decode`, iterator-based with no indexing. slice::chunk_by
//    is off-limits: build the grouping yourself (fold, or a peekable with
//    while let).
// 2. Look-and-say: "1" -> "11" -> "21" -> "1211" -> "111221" -> ... Print the
//    lengths of the 40th and 50th terms.
// 3. Make it fast: work on Vec<u8> digits instead of Strings; compare times.
// 4. Print len(n + 1) / len(n) up to n = 50. What does it approach?
//
// CHECK YOUR UNDERSTANDING
// 1. How did you group consecutive equal items? What made it awkward?
// 2. Why was Vec<u8> faster? Where were allocations happening before?
//
// ===========================================================================
// MY ANSWERS (write them before asking Claude to review)
// 1.
// 2.
// ===========================================================================

fn main() {
    // Your code here.
}
