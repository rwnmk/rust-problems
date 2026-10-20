// ===========================================================================
// 019 · String Compression
// Chapter 4: Understanding Ownership
// Difficulty: ★★★☆  challenging    Source: LeetCode 443
// FENCE: Use only what chapters 1-4 cover.
//        Not yet: structs, enums/Option, Vec, HashMap, closures, iterator adapters.
// ===========================================================================
//
// TASK
// `fn compress(buf: &mut [u8]) -> usize` - compress runs in place: a run of
// length 1 is just the byte; a longer run is the byte followed by its count in
// decimal (counts can exceed 9). Return the new length.
//   b"aabbccc" -> "a2b2c3"      b"a" -> "a"      b"abbbbbbbbbbbb" -> "ab12"
// - No allocation at all: write the digits of the count straight into the
//   buffer without .to_string().
// - A mutable byte array: `let mut buf = *b"aabbccc";`
//
// CHECK YOUR UNDERSTANDING
// 1. Why can you never overwrite bytes you haven't read yet?
// 2. How did you write a multi-digit count without allocating?
// 3. What is the type of `b"abc"`, and of `*b"abc"`? Why was the `*` needed?
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
