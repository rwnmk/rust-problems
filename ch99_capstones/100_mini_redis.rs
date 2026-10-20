// ===========================================================================
// 100 · Mini Redis
// Capstone
// Difficulty: ★★★★  stretch    Source: Original (cf. Redis)
// FENCE: The whole Book. Everything. Std only unless stated.
// ===========================================================================
//
// TASK
// A key-value server over TCP, pulling together chapters 8 to 21:
// - A line-based text protocol: SET key value [EX seconds], GET, DEL, INCR,
//   EXPIRE, TTL, KEYS pattern (with * and ?), PING, QUIT.
// - Concurrency: your ch. 21 thread pool with shared state behind a Mutex or
//   RwLock, OR async with a real runtime such as tokio. Choose and justify.
// - Expiry: lazy on read plus a periodic background sweep.
// - Persistence: periodic snapshots written to a temp file then renamed; load
//   on start-up.
// - A client binary in the same workspace; integration tests that start the
//   server on a random free port.
// - Bonus: SUBSCRIBE channel / PUBLISH channel message.
//
// CHECK YOUR UNDERSTANDING
// 1. Mutex vs RwLock vs sharded locks: what did you choose, and what did your
//    measurements say?
// 2. Why write-then-rename for snapshots?
// 3. Name a bug the type system made impossible, and one it couldn't help
//    with.
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
