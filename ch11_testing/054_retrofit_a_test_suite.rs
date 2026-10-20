// ===========================================================================
// 054 · Retrofit a Test Suite
// Chapter 11: Writing Automated Tests
// Difficulty: ★★☆☆  core    Source: Original
// FENCE: Use only what chapters 1-11 cover.
//        Not yet: closures, iterator adapters (in your own code), external crates.
// ===========================================================================
//
// TASK
// Take your Date (045) or Rational (023):
// 1. Move it into a library crate.
// 2. Unit tests in #[cfg(test)] mod tests - including one for a PRIVATE helper
//    (gcd, is_leap, ...).
// 3. Integration tests in tests/ that use only the public API, with shared
//    setup in tests/common/mod.rs.
// 4. An exhaustive test (e.g. every date from 1900 to 2100 survives
//    parse/format and add_days(1)/add_days(-1)) marked #[ignore]; run it
//    explicitly.
// 5. At least one test returning Result<(), String> that uses `?`.
//
// CHECK YOUR UNDERSTANDING
// 1. Why can unit tests call private functions while integration tests can't?
// 2. Why tests/common/mod.rs and not tests/common.rs?
// 3. Why can't a binary-only crate have integration tests? What's the usual
//    fix?
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
