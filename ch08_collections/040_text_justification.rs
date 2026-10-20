// ===========================================================================
// 040 · Text Justification
// Chapter 8: Common Collections
// Difficulty: ★★★☆  challenging    Source: LeetCode 68; K&R Ex. 1-22
// FENCE: Use only what chapters 1-8 cover.
//        Not yet: `?` (use match/expect), generics, traits, closures.
// ===========================================================================
//
// TASK
// `fn justify(words: &[&str], width: usize) -> Vec<String>`
// - Fill lines greedily. Spread spaces as evenly as possible; leftover spaces
//   go to the LEFTMOST gaps.
// - The last line, and any line with a single word, is left-justified and
//   padded to width.
// - Words longer than width: hard-split them with a hyphen.
// - Then read a paragraph from stdin and print it justified to width 40
//   between | borders so you can see the alignment.
// EXAMPLE (width 16)
//   |This    is    an|
//   |example  of text|
//   |justification.  |
//
// CHECK YOUR UNDERSTANDING
// 1. How did you compute the gaps? Show it for 3 words with 7 spare spaces.
// 2. How many allocations happen per line? Could each line be built with
//    exactly one (String::with_capacity)?
//
// ===========================================================================
// MY ANSWERS (write them before asking Claude to review)
// 1.
// 2.
// ===========================================================================

fn main() {
    // Your code here.
}
