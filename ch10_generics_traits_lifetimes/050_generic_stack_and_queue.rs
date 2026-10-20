// ===========================================================================
// 050 · Generic Stack and Queue
// Chapter 10: Generic Types, Traits, and Lifetimes
// Difficulty: ★★☆☆  core    Source: LeetCode 20 and 232
// FENCE: Use only what chapters 1-10 cover.
//        Not yet: closures, iterator adapters, Box, dyn trait objects.
// ===========================================================================
//
// TASK
// - `struct Stack<T>` (wrapping a Vec): push, pop -> Option<T>, peek ->
//   Option<&T>, peek_mut, len, is_empty.
// - `struct Queue<T>` built from TWO Stacks (LeetCode 232), amortised O(1).
// - `impl<T: Display> Display for Stack<T>`.
// - `trait Container<T> { fn put(&mut self, x: T); fn take(&mut self) ->
//   Option<T>; fn size(&self) -> usize; }` for both, plus
//   `fn drain_all<C: Container<i32>>(c: &mut C) -> Vec<i32>`.
// - Use your Stack for LeetCode 20, Valid Parentheses: "([]{})" ok, "(]" not.
//
// CHECK YOUR UNDERSTANDING
// 1. Why is the two-stack queue amortised O(1)? Give a sequence where a
//    single dequeue costs O(n).
// 2. While you hold the Option<&mut T> from peek_mut, what can't you do?
// 3. Why put the Display bound on the impl rather than on the struct?
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
