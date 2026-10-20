// ===========================================================================
// 057 · minigrep, Extended
// Chapter 12: An I/O Project (minigrep)
// Difficulty: ★★☆☆  core    Source: Rust Book ch. 12 extension
// FENCE: Use only what chapters 1-12 cover.
//        Not yet: closures and iterator adapters - ch. 13 is where you refactor.
// ===========================================================================
//
// TASK
// Starting from your finished minigrep, add:
// - Flags -i (ignore case), -n (line numbers), -c (count only), -v (invert),
//   -w (whole words); combinable and bundled: `minigrep -in to poem.txt`.
// - Multiple files; prefix each match with its file name when there's more
//   than one.
// - A file named "-" means stdin.
// - Precedence: -i beats the IGNORE_CASE variable; add --case-sensitive to
//   override it the other way.
// - Exit codes like grep: 0 if anything matched, 1 if nothing, 2 on error.
// - Keep main.rs tiny; TDD the new search functions in lib.rs.
//
// RULES
// No clap - parse args by hand. No iterator adapters yet (you'll refactor this
// in 060).
//
// CHECK YOUR UNDERSTANDING
// 1. How did you represent the config? What invalid combinations can it
//    still represent?
// 2. Why do errors go to stderr? Show it with `> out.txt`.
// 3. What did you do to keep the search logic testable without files?
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
