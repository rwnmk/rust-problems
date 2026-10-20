// ===========================================================================
// 056 · Homemade Property Testing
// Chapter 11: Writing Automated Tests
// Difficulty: ★★★☆  challenging    Source: Original (cf. QuickCheck)
// FENCE: Use only what chapters 1-11 cover.
//        Not yet: closures, iterator adapters (in your own code), external crates.
// ===========================================================================
//
// TASK
// 1. A tiny seeded PRNG, xorshift64*:
//      x ^= x >> 12;  x ^= x << 25;  x ^= x >> 27;
//      return x.wrapping_mul(0x2545F4914F6CDD1D);
// 2. Tests that generate 10,000 random Vec<i32> (lengths 0..50, duplicates and
//    negatives included) and check your merge_sort (052): the output is
//    sorted; it's a permutation of the input; sorting twice equals sorting
//    once; it agrees with std's sort.
// 3. Properties for Roman numerals (053) and Rational (023): a + b == b + a,
//    (a + b) - b == a, and so on.
// 4. Shrinking: when a property fails, repeatedly try removing elements while
//    it still fails, and report the smallest failing input.
// 5. Plant a bug in merge_sort and check that it's caught AND shrunk.
// (The proptest crate does this for real. Not yet.)
//
// CHECK YOUR UNDERSTANDING
// 1. Why a seeded PRNG rather than true randomness in tests?
// 2. What makes a property useful? Give one that is true but useless.
// 3. How small did your shrinker get the planted bug's input?
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
