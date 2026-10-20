// ===========================================================================
// 076 · Parallel Word Count
// Chapter 16: Fearless Concurrency
// Difficulty: ★★☆☆  core    Source: Original
// FENCE: Use only what chapters 1-16 cover.
//        Not yet: async, external crates (no rayon/crossbeam).
// ===========================================================================
//
// TASK
// Count words across many files (a folder of Project Gutenberg books works
// well), three ways:
//   A) Arc<Mutex<HashMap<String, usize>>>, locking once per word.
//   B) Each thread builds its own HashMap; main merges them at the end.
//   C) Like A, but each thread locks once per FILE to merge a local map.
// Time all three and explain the differences.
// Then make one thread panic while holding the lock. Observe lock poisoning,
// and recover from it.
//
// CHECK YOUR UNDERSTANDING
// 1. Why was A slow?
// 2. What is lock poisoning, and how did you recover?
// 3. Why won't Rc<RefCell<...>> compile here? What do Send and Sync mean?
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
