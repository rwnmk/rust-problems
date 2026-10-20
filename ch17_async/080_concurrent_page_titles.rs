// ===========================================================================
// 080 · Concurrent Page Titles
// Chapter 17: Async and Await
// Difficulty: ★★☆☆  core    Source: Rust Book ch. 17 extension
// FENCE: Use only what chapters 1-17 cover.
//        Allowed crate: trpl (the Book's helper). Not yet: tokio directly.
// ===========================================================================
//
// TASK
// Given URLs on the command line:
// - Fetch them all concurrently (the Book's trpl::get and Html approach) and
//   print each page's <title>, first in COMPLETION order, then in INPUT
//   order.
// - One failing URL must not stop the others.
// - Then race them and print only the fastest.
// - Then limit concurrency to 2 requests at a time.
//
// CHECK YOUR UNDERSTANDING
// 1. Why does join_all need every future to have the same type?
// 2. How did you print in input order while fetching concurrently?
// 3. Is any of this running on multiple threads? How could you find out?
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
