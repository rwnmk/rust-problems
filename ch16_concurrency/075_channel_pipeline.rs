// ===========================================================================
// 075 · Channel Pipeline
// Chapter 16: Fearless Concurrency
// Difficulty: ★★☆☆  core    Source: Original
// FENCE: Use only what chapters 1-16 cover.
//        Not yet: async, external crates (no rayon/crossbeam).
// ===========================================================================
//
// TASK
// Four stages, each its own thread, joined by mpsc channels:
//   generate 1..=1_000_000 -> square -> keep palindromes -> collect
// The collector prints how many it got and the largest.
// - Then use 4 generator threads (cloned Senders), each producing a quarter
//   of the range.
// - Clean shutdown: every thread must exit; join them all and prove it.
// - Swap in sync_channel(0) and sync_channel(1000) and time each against the
//   unbounded channel.
//
// CHECK YOUR UNDERSTANDING
// 1. How does a downstream stage learn there's no more data coming?
// 2. What happens if you forget to drop the original Sender after cloning?
// 3. What is backpressure, and when do you want it?
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
