// ===========================================================================
// 064 · Memoiser
// Chapter 13: Iterators and Closures
// Difficulty: ★★☆☆  core    Source: Rust Book (earlier editions' Cacher)
// FENCE: Use only what chapters 1-13 cover.
//        Not yet: Box/Rc/RefCell, threads, dyn trait objects.
// ===========================================================================
//
// TASK
//   struct Memo<F: Fn(u64) -> u64> { f: F, cache: HashMap<u64, u64> }
// - `fn get(&mut self, n: u64) -> u64`
// - Memoise something slow (e.g. counting primes below n with is_prime).
// - Generalise to Memo<A, R, F> with A: Eq + Hash + Clone and R: Clone.
// - Now try to memoise RECURSIVE Fibonacci so the recursive calls hit the
//   cache. Why is this awkward? Find a design that works, and compare fib(90)
//   naive vs memoised.
//
// CHECK YOUR UNDERSTANDING
// 1. Why can't the stored closure call memo.get itself?
// 2. get needs &mut self. What does that imply about sharing one Memo between
//    several users? (A glimpse of ch. 15.)
//
// ===========================================================================
// MY ANSWERS (write them before asking Claude to review)
// 1.
// 2.
// ===========================================================================

fn main() {
    // Your code here.
}
