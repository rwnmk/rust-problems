// ===========================================================================
// 073 · A File System Tree
// Chapter 15: Smart Pointers
// Difficulty: ★★★★  stretch    Source: Inspired by Advent of Code 2022, day 7
// FENCE: Use only what chapters 1-15 cover.
//        Not yet: threads, async, dyn trait objects (beyond Box<dyn Error>).
// ===========================================================================
//
// TASK
// Parse a terminal transcript like the one below into a directory tree, where
// children are Rc<RefCell<...>> and each directory holds a Weak link to its
// parent (so `cd ..` works).
// - The total size of every directory (recursive).
// - The sum of the totals of all directories of at most 100_000 bytes.
// - With a 70_000_000 disk needing 30_000_000 free: the smallest single
//   directory whose deletion frees enough.
// - `fn pwd(node) -> String` by walking parent links.
// - Print the tree like the `tree` command, with sizes.
// - Then do it AGAIN without Rc/RefCell, as an arena: Vec<Node> with usize
//   indices for parent and children. Compare the two.
// Sample transcript (write your own bigger ones too):
//   $ cd /
//   $ ls
//   dir docs
//   dir src
//   120000 notes.txt
//   $ cd docs
//   $ ls
//   4500 todo.md
//   dir old
//   $ cd old
//   $ ls
//   98000 draft.md
//   $ cd ..
//   $ cd ..
//   $ cd src
//   $ ls
//   30000 main.rs
//   2100 lib.rs
//
// CHECK YOUR UNDERSTANDING
// 1. Where could borrow_mut have panicked, and how did you avoid holding
//    borrows too long?
// 2. Arena vs Rc<RefCell>: which was easier? What does each fail like (stale
//    indices vs runtime borrow panics)?
//
// ===========================================================================
// MY ANSWERS (write them before asking Claude to review)
// 1.
// 2.
// ===========================================================================

fn main() {
    // Your code here.
}
