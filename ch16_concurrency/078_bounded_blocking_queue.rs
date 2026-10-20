// ===========================================================================
// 078 · Bounded Blocking Queue
// Chapter 16: Fearless Concurrency
// Difficulty: ★★★★  stretch    Source: Original
// FENCE: Use only what chapters 1-16 cover.
//        Not yet: async, external crates (no rayon/crossbeam).
// ===========================================================================
//
// TASK
// Build BoundedQueue<T> with a capacity: push blocks when full, pop blocks
// when empty, and close() wakes everyone up; after close, pop drains what's
// left and then returns None.
// - Use Mutex + Condvar. Condvar isn't in the Book - learning it from the std
//   docs is part of the stretch.
// - Build a worker pool on it whose jobs are Box<dyn FnOnce() + Send>
//   (trait objects are ch. 18; ch. 21 uses exactly this type).
// - Use the pool to find all primes below 10_000_000 in segments.
// - Stress test: 8 producers, 8 consumers, 1_000_000 items; check every item
//   arrives exactly once.
//
// CHECK YOUR UNDERSTANDING
// 1. Why must Condvar::wait sit in a loop?
// 2. What's a lost wakeup, and how does holding the mutex prevent it?
// 3. What does your queue offer that sync_channel doesn't?
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
