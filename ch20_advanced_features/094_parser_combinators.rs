// ===========================================================================
// 094 · Parser Combinators
// Chapter 20: Advanced Features
// Difficulty: ★★★★  stretch    Source: Inspired by Bodil Stokke, "Learning Parser Combinators With Rust"
// FENCE: Use only what chapters 1-20 cover.
//        The whole Book is fair game. Std only unless stated.
// ===========================================================================
//
// TASK
//   type ParseResult<'a, T> = Option<(T, &'a str)>;
//   trait Parser<'a, T> { fn parse(&self, input: &'a str) -> ParseResult<'a, T>; }
// with a blanket impl for any F: Fn(&'a str) -> ParseResult<'a, T>.
// - Primitives: char_p(c), literal("let"), satisfy(pred), whitespace.
// - Combinators returning impl Parser: map, pair, left, right, either,
//   zero_or_more, one_or_more, optional, sep_by.
// - Recursive grammars will need a Box<dyn Parser> somewhere - explain why.
// - Parse "2 * (x + 3) ^ 2 - y / 4" with correct precedence and associativity
//   into your Expr from 068, and evaluate it.
// - Then parse a small JSON subset.
//
// CHECK YOUR UNDERSTANDING
// 1. Why does a recursive grammar force a Box<dyn ...> (or a fn item)?
// 2. What does the full type of a three-level combinator look like before
//    boxing? What happens to compile times?
// 3. How did you get left-associativity without left recursion?
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
