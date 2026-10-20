// ===========================================================================
// 015 · Slice Tools
// Chapter 4: Understanding Ownership
// Difficulty: ★★☆☆  core    Source: Original
// FENCE: Use only what chapters 1-4 cover.
//        Not yet: structs, enums/Option, Vec, HashMap, closures, iterator adapters.
// ===========================================================================
//
// TASK
//   fn sum(xs: &[i32]) -> i32
//   fn max(xs: &[i32]) -> i32          // you decide what an empty slice gives
//   fn is_sorted(xs: &[i32]) -> bool
//   fn reverse_in_place(xs: &mut [i32])     // swap by hand: no .swap/.reverse
//   fn rotate_left(xs: &mut [i32], k: usize)
// rotate_left must work by calling reverse_in_place three times on sub-slices
// (work out how - draw it on paper). Test with arrays `[i32; N]` passed as
// `&arr`, and with sub-slices like `&arr[1..4]`.
//
// CHECK YOUR UNDERSTANDING
// 1. What's the difference between [i32; 5], &[i32; 5] and &[i32]?
// 2. In rotate_left you mutably borrow xs[..k] and later xs[k..]. Why isn't
//    that two simultaneous mutable borrows?
// 3. What did max return for an empty slice? What's unsatisfying about that?
//    Keep your answer - ch. 6 gives you a better tool.
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
