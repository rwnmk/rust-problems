// ===========================================================================
// 066 · Document It Like a Crate
// Chapter 14: More about Cargo and Crates.io
// Difficulty: ★☆☆☆  warm-up    Source: Rust Book ch. 14
// FENCE: Use only what chapters 1-14 cover.
//        Not yet: smart pointers, concurrency.
// ===========================================================================
//
// TASK
// Take a library from an earlier problem (Rational, Date or SortedVec):
// - Crate docs with //! and every public item documented with ///, with
//   # Examples (as doctests), plus # Panics and # Errors where they apply.
// - At least one ```compile_fail doctest showing misuse your API prevents
//   (e.g. building a validated type directly).
// - Use a hidden `# ` line in a doctest to keep the example clean.
// - Reorganise into internal modules, then flatten the public API with
//   `pub use`. Check the result with `cargo doc --open`.
// - Fill in Cargo.toml metadata (description, license, repository) and run
//   `cargo publish --dry-run`. Don't actually publish.
//
// CHECK YOUR UNDERSTANDING
// 1. Doctests are compiled as separate crates. What does that mean for how
//    they refer to your items?
// 2. Why re-export with pub use instead of making users walk your module tree?
// 3. What did --dry-run complain about?
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
