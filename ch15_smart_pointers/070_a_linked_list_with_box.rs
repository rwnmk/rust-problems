// ===========================================================================
// 070 · A Linked List with Box
// Chapter 15: Smart Pointers
// Difficulty: ★★★☆  challenging    Source: Cf. "Learning Rust With Entirely Too Many Linked Lists"
// FENCE: Use only what chapters 1-15 cover.
//        Not yet: threads, async, dyn trait objects (beyond Box<dyn Error>).
// ===========================================================================
//
// TASK
//   pub struct List<T> { head: Option<Box<Node<T>>>, len: usize }
//   struct Node<T> { val: T, next: Option<Box<Node<T>>> }
// - push_front, pop_front, peek, peek_mut, len, and reverse (in place, no
//   allocation).
// - iter() (borrowing), iter_mut(), and into_iter().
// - Build a 1,000,000-element list and let it go out of scope in a debug
//   build. Explain what happens, then implement Drop iteratively to fix it.
// - Bonus: why is an O(1) push_back hard with Box alone?
// Option::take will be your best friend.
//
// CHECK YOUR UNDERSTANDING
// 1. Why did dropping the long list blow the stack?
// 2. What does Option::take do, and why does it keep coming up?
// 3. Why is a doubly-linked list awkward with Box? Which ch. 15 tools would
//    it need?
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
