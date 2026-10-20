// ===========================================================================
// 059 · csvq
// Chapter 12: An I/O Project (minigrep)
// Difficulty: ★★★☆  challenging    Source: Original
// FENCE: Use only what chapters 1-12 cover.
//        Not yet: closures and iterator adapters - ch. 13 is where you refactor.
// ===========================================================================
//
// TASK
// A tiny CSV query tool:
//   csvq people.csv --select name,age --where "age>30" --sort age --desc --limit 5
// - The first row is the header. Support RFC 4180 quoting: "Smith, J" and
//   "He said ""hi""". Write the parser yourself as a state machine.
// - --where supports = != < > <= >=, comparing numerically when both sides
//   parse as numbers and as strings otherwise.
// - Output an aligned table.
// - Errors (unknown column, unterminated quote with line number) go to stderr
//   with a non-zero exit code.
// - The parser lives in lib.rs with tests.
//
// RULES
// No closures yet, so no sort_by - sorting by a chosen column is part of the
// challenge (your generic merge_sort from 052 plus a type that implements Ord
// is one way).
//
// CHECK YOUR UNDERSTANDING
// 1. Sketch your parser's states. Which transitions handle "" inside a quoted
//    field?
// 2. How did you sort by an arbitrary column without closures?
//
// ===========================================================================
// MY ANSWERS (write them before asking Claude to review)
// 1.
// 2.
// ===========================================================================

fn main() {
    // Your code here.
}
