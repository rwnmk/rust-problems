// ===========================================================================
// 067 · Workspace and Profiles
// Chapter 14: More about Cargo and Crates.io
// Difficulty: ★★☆☆  core    Source: Rust Book ch. 14
// FENCE: Use only what chapters 1-14 cover.
//        Not yet: smart pointers, concurrency.
// ===========================================================================
//
// TASK
// Create a workspace:
//   euler/      library: the Fib, Primes and Collatz iterators from 062
//   euler-cli/  binary: `euler-cli 14` runs problem 14
//   bench/      binary: times every solution
// - Give two members the same external dependency; confirm there's a single
//   Cargo.lock, at the root.
// - In the root Cargo.toml add a custom profile `release-lto` (inherits =
//   "release", lto = true, codegen-units = 1, panic = "abort"). For dev,
//   release and release-lto, tabulate build time, binary size and run time of
//   problem 14.
// - `cargo install --path euler-cli` and run it from another directory.
//
// CHECK YOUR UNDERSTANDING
// 1. Why does a workspace share one target/ and one Cargo.lock?
// 2. What do lto and codegen-units change, and why do they slow the build?
// 3. What behaviour changes with panic = "abort"?
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
