// ===========================================================================
// 051 · A Borrowing Tokenizer
// Chapter 10: Generic Types, Traits, and Lifetimes
// Difficulty: ★★★☆  challenging    Source: Original
// FENCE: Use only what chapters 1-10 cover.
//        Not yet: closures, iterator adapters, Box, dyn trait objects.
// ===========================================================================
//
// TASK
//   enum Token<'a> { Ident(&'a str), Number(i64), Symbol(char), Str(&'a str) }
//   struct Tokenizer<'a> { src: &'a str, pos: usize }
// - `fn new(src: &'a str) -> Self`
// - `fn next_token(&mut self) -> Option<Result<Token<'a>, String>>`: skips
//   whitespace; Str holds the contents WITHOUT quotes; unterminated strings
//   and bad characters are errors that include the position.
// - Zero allocation for identifiers and strings: tokens borrow from the
//   source.
// - In main, tokenize `let total = price * 3 + "tax";` into a Vec, DROP the
//   tokenizer, then still use the tokens.
// (You'll make this an Iterator in ch. 13.)
//
// CHECK YOUR UNDERSTANDING
// 1. Why return Token<'a> rather than a token tied to &mut self? What would
//    break with the latter?
// 2. Can the tokens outlive the source String? Show the error.
// 3. Why might a real compiler allocate owned tokens anyway?
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
