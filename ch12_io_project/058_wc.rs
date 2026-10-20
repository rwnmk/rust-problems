// ===========================================================================
// 058 · wc
// Chapter 12: An I/O Project (minigrep)
// Difficulty: ★★☆☆  core    Source: K&R §1.5.4
// FENCE: Use only what chapters 1-12 cover.
//        Not yet: closures and iterator adapters - ch. 13 is where you refactor.
// ===========================================================================
//
// TASK
// `wc [-l] [-w] [-c] [-m] [FILE...]`
// - No flags: lines, words and bytes, like GNU wc. -m counts characters.
// - Multiple files print a total line; no files means stdin.
// - An unreadable file: message to stderr, carry on, exit 1 at the end.
// - Compare your output with the real wc on several files, including one with
//   no trailing newline and one with non-ASCII text.
// - Bonus: process big files in chunks with BufReader instead of
//   read_to_string.
//
// CHECK YOUR UNDERSTANDING
// 1. What does wc mean by a "line"? What happens with no trailing newline?
// 2. How did you define a word? What counts as whitespace?
// 3. read_to_string vs streaming: memory, and what about invalid UTF-8?
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
