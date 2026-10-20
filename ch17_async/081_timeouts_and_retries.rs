// ===========================================================================
// 081 · Timeouts and Retries
// Chapter 17: Async and Await
// Difficulty: ★★★☆  challenging    Source: Rust Book ch. 17 extension
// FENCE: Use only what chapters 1-17 cover.
//        Allowed crate: trpl (the Book's helper). Not yet: tokio directly.
// ===========================================================================
//
// TASK
// Write your own combinators - attempt them BEFORE reading the Book's version
// of timeout:
// - `async fn timeout<F: Future>(f: F, max: Duration) -> Result<F::Output,
//   Duration>`
// - `async fn retry<F, Fut, T, E>(attempts: u32, base_delay: Duration,
//   make: F) -> Result<T, E>` where F: FnMut() -> Fut and
//   Fut: Future<Output = Result<T, E>>, doubling the delay after each failure.
// - A fake flaky operation that fails the first k times and is sometimes slow.
// - Combine them: retry 5 times, each attempt wrapped in a 300ms timeout.
//
// CHECK YOUR UNDERSTANDING
// 1. Why does retry take a closure that MAKES futures rather than a future?
// 2. When the timeout wins, what happens to the losing future's work?
// 3. Where, if anywhere, did the compiler make you think about Pin?
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
