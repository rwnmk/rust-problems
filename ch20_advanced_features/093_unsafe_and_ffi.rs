// ===========================================================================
// 093 · Unsafe and FFI
// Chapter 20: Advanced Features
// Difficulty: ★★★☆  challenging    Source: Rust Book ch. 20
// FENCE: Use only what chapters 1-20 cover.
//        The whole Book is fair game. Std only unless stated.
// ===========================================================================
//
// TASK
// 1. Write your own split_at_mut with raw pointers and
//    std::slice::from_raw_parts_mut - without peeking at the Book's version.
//    Then `split3_mut(xs, a, b)` returning three mutable slices.
// 2. `fn get_two_mut<T>(xs: &mut [T], i: usize, j: usize) -> Option<(&mut T,
//    &mut T)>`: a safe API with unsafe inside; None when i == j or either
//    index is out of bounds. Compare with std's get_disjoint_mut.
// 3. FFI: call C's abs, strlen (via CString) and qsort with an extern "C"
//    comparison callback for i32s.
// 4. Run your code under Miri (`rustup +nightly component add miri`,
//    `cargo +nightly miri run`). Then break get_two_mut so i == j aliases,
//    and record what Miri reports.
// 5. Put a // SAFETY: comment above every unsafe block.
//
// CHECK YOUR UNDERSTANDING
// 1. What extra abilities does unsafe give you, and what checks stay on?
// 2. Why are two &mut to the same element undefined behaviour even if you
//    never write through both?
// 3. Which invariants do your SAFETY comments depend on?
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
