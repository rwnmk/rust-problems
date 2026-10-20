// ===========================================================================
// 077 · Dining Philosophers
// Chapter 16: Fearless Concurrency
// Difficulty: ★★★☆  challenging    Source: Dijkstra
// FENCE: Use only what chapters 1-16 cover.
//        Not yet: async, external crates (no rayon/crossbeam).
// ===========================================================================
//
// TASK
// Five philosophers, five forks (Mutexes). Each: think (random short sleep),
// pick up left fork, then right, eat, put both down - 100 times.
// 1. The naive version: make deadlock likely (tune the sleeps) and add a
//    watchdog thread that reports "no progress for 2s".
// 2. Fix it with resource ordering (always take the lower-numbered fork
//    first). Argue why it can't deadlock.
// 3. Another fix: a "waiter" allowing at most 4 philosophers to reach for
//    forks at once. Record how often each philosopher eats in a fixed time
//    under each fix and compare fairness.
//
// CHECK YOUR UNDERSTANDING
// 1. Name the four Coffman conditions. Which one does each fix break?
// 2. Is your solution starvation-free? How would your data show it?
// 3. Exactly where is each MutexGuard dropped in your code?
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
