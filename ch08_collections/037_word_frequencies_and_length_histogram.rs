// ===========================================================================
// 037 · Word Frequencies and Length Histogram
// Chapter 8: Common Collections
// Difficulty: ★★☆☆  core    Source: McIlroy's word count; K&R Ex. 1-13
// FENCE: Use only what chapters 1-8 cover.
//        Not yet: `?` (use match/expect), generics, traits, closures.
// ===========================================================================
//
// TASK
// Read all of stdin until EOF.
// 1. Print the N most frequent words (N a const), case-insensitive, with
//    surrounding punctuation stripped. Ties broken alphabetically.
// 2. Print a horizontal histogram of word lengths:   3 | ########## 10
// 3. Bonus: a VERTICAL histogram (the harder half of K&R 1-13).
// Try it on a whole book: `cargo run --release < book.txt`.
//
// RULES
// No closures yet, so no sort_by. Hint: tuples sort lexicographically with
// plain .sort(), and a small trick gets you descending counts.
//
// CHECK YOUR UNDERSTANDING
// 1. How did you detect EOF with read_line?
// 2. How did you get descending-by-count, ascending-by-word without closures?
// 3. What does to_lowercase do to "ß" and to "İ"? Try it. Why is that tricky?
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
