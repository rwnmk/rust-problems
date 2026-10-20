// ===========================================================================
// 055 · Bug Hunt
// Chapter 11: Writing Automated Tests
// Difficulty: ★★☆☆  core    Source: Original
// FENCE: Use only what chapters 1-11 cover.
//        Not yet: closures, iterator adapters (in your own code), external crates.
// ===========================================================================
//
// TASK
// The four functions below contain at least SEVEN bugs between them. For each
// function, first write tests that FAIL because of a bug - before changing
// any code. Give each test a name that says what it checks. Then fix the code
// and keep the tests. Run the suite in debug and in --release.
//
// CHECK YOUR UNDERSTANDING
// 1. Which bugs did a "normal" example miss? What kind of input exposed them?
// 2. Did any test behave differently in release mode? Why?
// 3. Write the checklist of edge cases you'll reach for next time.
//
// ===========================================================================
// MY ANSWERS (write them before asking Claude to review)
// 1.
// 2.
// 3.
// ===========================================================================

/// True for leap years in the Gregorian calendar.
pub fn is_leap_year(y: u32) -> bool {
    y % 4 == 0 && y % 100 != 0
}

/// Index of `target` in a sorted slice, or None.
pub fn binary_search(xs: &[i32], target: i32) -> Option<usize> {
    let mut lo = 0;
    let mut hi = xs.len() - 1;
    while lo < hi {
        let mid = (lo + hi) / 2;
        if xs[mid] == target {
            return Some(mid);
        } else if xs[mid] < target {
            lo = mid + 1;
        } else {
            hi = mid - 1;
        }
    }
    None
}

/// Number of words separated by any whitespace.
pub fn word_count(s: &str) -> usize {
    s.split(' ').count()
}

/// Arithmetic mean, or None if empty.
pub fn mean(xs: &[i32]) -> Option<f64> {
    if xs.is_empty() {
        return None;
    }
    let sum: i32 = xs.iter().sum();
    Some((sum / xs.len() as i32) as f64)
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    // Your failing tests go here.
}
