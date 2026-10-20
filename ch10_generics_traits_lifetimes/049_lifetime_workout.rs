// ===========================================================================
// 049 · Lifetime Workout
// Chapter 10: Generic Types, Traits, and Lifetimes
// Difficulty: ★★☆☆  core    Source: Original
// FENCE: Use only what chapters 1-10 cover.
//        Not yet: closures, iterator adapters, Box, dyn trait objects.
// ===========================================================================
//
// TASK
// 1. `fn longest_common_prefix(a: &str, b: &str) -> &str`. Write it with no
//    annotations first and read the error. Then annotate it so the result is
//    tied ONLY to `a`. Also write `lcp_both<'a>(a: &'a str, b: &'a str) ->
//    &'a str`.
// 2. `struct Highlight<'a> { text: &'a str, start: usize, end: usize }` with
//    `fn excerpt(&self) -> &str`. Which lifetime does elision give the return?
//    Change the signature so an excerpt can outlive the Highlight itself.
// 3. `fn split_sentences(text: &str) -> Vec<&str>`.
// 4. Predict, then test, the snippets in main.
//
// CHECK YOUR UNDERSTANDING
// 1. Lifetime annotations don't change how long anything lives. So what DO
//    they do?
// 2. State the three elision rules and apply them to excerpt.
// 3. When is 'static the right answer, and what's the common wrong reason for
//    reaching for it?
//
// ===========================================================================
// MY ANSWERS (write them before asking Claude to review)
// 1.
// 2.
// 3.
// ===========================================================================

// Write longest_common_prefix, lcp_both and Highlight first.

fn main() {
    // A: with longest_common_prefix (tied only to `a`)
    // let result;
    // let a = String::from("abc");
    // {
    //     let b = String::from("abd");
    //     result = longest_common_prefix(&a, &b);
    // }
    // println!("{result}");
    // PREDICTION:

    // B: identical, but call lcp_both instead
    // PREDICTION:

    // C:
    // let h;
    // {
    //     let s = String::from("some text here");
    //     h = Highlight { text: &s, start: 0, end: 4 };
    // }
    // println!("{}", h.excerpt());
    // PREDICTION:

    // D: try with the elided excerpt, then with your improved signature
    // let text = String::from("some text here");
    // let ex;
    // {
    //     let h = Highlight { text: &text, start: 5, end: 9 };
    //     ex = h.excerpt();
    // }
    // println!("{ex}");
    // PREDICTION:
}
