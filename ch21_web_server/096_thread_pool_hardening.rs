// ===========================================================================
// 096 · Thread Pool Hardening
// Chapter 21: Final Project: Web Server
// Difficulty: ★★★★  stretch    Source: Rust Book ch. 21 suggestions
// FENCE: Use only what chapters 1-21 cover.
//        Std only unless stated.
// ===========================================================================
//
// TASK
// - Graceful shutdown on GET /shutdown: stop accepting, finish in-flight
//   requests, join the workers, exit 0. (Optionally Ctrl-C too, via the ctrlc
//   crate.)
// - A job that panics must not permanently kill its worker: detect it and
//   respawn (or research catch_unwind and UnwindSafe).
// - A /stats endpoint: requests per worker, in-flight count and uptime, using
//   atomics. Justify your memory Ordering.
// - Defend against slow clients with read timeouts on the TcpStream.
// - Load test: 10,000 requests from 50 concurrent clients - none lost, no
//   hang on shutdown.
// - Bonus: port it to async and compare.
//
// CHECK YOUR UNDERSTANDING
// 1. How does the accept loop find out it should stop while it's blocked in
//    accept()?
// 2. Relaxed vs SeqCst for your counters: why is Relaxed enough (or not)?
// 3. What happens to a Mutex a worker held when it panicked?
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
