// ===========================================================================
// 074 · Parallel Sum
// Chapter 16: Fearless Concurrency
// Difficulty: ★☆☆☆  warm-up    Source: Original
// FENCE: Use only what chapters 1-16 cover.
//        Not yet: async, external crates (no rayon/crossbeam).
// ===========================================================================
//
// TASK
// - Build a Vec<u64> of 50_000_000 numbers from a deterministic formula.
// - Sum it on one thread.
// - Then split it into N chunks over N threads with thread::spawn. Share the
//   data via Arc - do NOT clone the vector.
// - Then do the same with std::thread::scope (read its docs), borrowing
//   slices directly.
// - Time N = 1, 2, 4, 8 and std::thread::available_parallelism().
//
// CHECK YOUR UNDERSTANDING
// 1. Why does thread::spawn need move and 'static data, while scope doesn't?
// 2. Did the speed-up scale linearly? Why not?
// 3. What does JoinHandle::join return, and what if the thread panicked?
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
