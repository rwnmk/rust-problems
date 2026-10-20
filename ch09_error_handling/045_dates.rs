// ===========================================================================
// 045 · Dates
// Chapter 9: Error Handling
// Difficulty: ★★★☆  challenging    Source: Original
// FENCE: Use only what chapters 1-9 cover.
//        Not yet: generics, your own traits, closures (.map_err etc.).
// ===========================================================================
//
// TASK
// `struct Date { year: i32, month: u8, day: u8 }` with PRIVATE fields.
// - `Date::parse("2026-10-20") -> Result<Date, DateError>` where DateError is
//   an enum with variants carrying useful detail, e.g. Day { day, max }.
// - Gregorian leap years; days_in_month.
// - `fn day_of_week(&self) -> Weekday` (your own enum).
// - `fn days_since_epoch(&self) -> i64` (1970-01-01 = 0), `days_between`.
// - `fn add_days(&self, n: i64) -> Date`, with negative n allowed.
// Checks: 2000-02-29 valid; 1900-02-29 invalid; 2026-10-20 is a Tuesday.
// Verify days_since_epoch against your shell:
//   echo $(( $(date -u -d 2026-10-20 +%s) / 86400 ))
//
// CHECK YOUR UNDERSTANDING
// 1. Which failures are Results and which are panics in your design? Why?
// 2. How would you convince yourself that add_days(n) then add_days(-n) always
//    gets back where it started? (Ch. 11 will give you tools.)
//
// ===========================================================================
// MY ANSWERS (write them before asking Claude to review)
// 1.
// 2.
// ===========================================================================

fn main() {
    // Your code here.
}
