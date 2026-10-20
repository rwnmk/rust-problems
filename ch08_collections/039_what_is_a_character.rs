// ===========================================================================
// 039 · What Is a Character?
// Chapter 8: Common Collections
// Difficulty: ★★☆☆  core    Source: Original
// FENCE: Use only what chapters 1-8 cover.
//        Not yet: `?` (use match/expect), generics, traits, closures.
// ===========================================================================
//
// TASK
// For each of: "hello", "Здравствуйте", "नमस्ते", "\u{e9}", "e\u{301}",
// "🇬🇧", "👩‍💻" print len(), chars().count(), the bytes in hex, and each char
// with its code point ({:04X} of `c as u32`). Then:
// - `fn safe_truncate(s: &str, max_bytes: usize) -> &str` - never panics,
//   never splits a character (look up is_char_boundary).
// - `fn reverse_chars(s: &str) -> String` - and show what happens to
//   "e\u{301}" and "🇬🇧" when reversed by chars. Explain.
// - Go back to problem 017 and make longest_run Unicode-safe (by chars).
//
// CHECK YOUR UNDERSTANDING
// 1. Why does &s[0..1] panic on "Здравствуйте" but not "hello"?
// 2. A text editor's cursor moves "one character". Which of bytes, chars or
//    graphemes does it need?
// 3. Why doesn't Rust allow s[i] on strings at all?
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
