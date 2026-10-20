// ===========================================================================
// 052 · SortedVec
// Chapter 10: Generic Types, Traits, and Lifetimes
// Difficulty: ★★★☆  challenging    Source: Original
// FENCE: Use only what chapters 1-10 cover.
//        Not yet: closures, iterator adapters, Box, dyn trait objects.
// ===========================================================================
//
// TASK
// - `fn merge_sorted<T: Ord + Clone>(a: &[T], b: &[T]) -> Vec<T>`
// - `fn merge_sort<T: Ord + Clone>(xs: &[T]) -> Vec<T>`
// - `struct SortedVec<T> { items: Vec<T> }` whose public API makes it
//   IMPOSSIBLE to break sortedness: new, from_vec (sorts), insert (use your OWN
//   binary search to find the position), contains, remove(&T) -> bool,
//   as_slice, merge(self, other) -> Self.
// - Bonus: merge_sort taking Vec<T> by value with no Clone bound.
//
// CHECK YOUR UNDERSTANDING
// 1. Why does merge_sorted need Clone with slices, and how does taking Vec<T>
//    by value remove that need?
// 2. What breaks if `items` is pub, or if you offer as_mut_slice?
// 3. Where did you put the T: Ord bounds (struct, impl, or methods) and why?
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
