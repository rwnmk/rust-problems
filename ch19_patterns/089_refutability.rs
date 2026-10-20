// ===========================================================================
// 089 · Refutability
// Chapter 19: Patterns and Matching
// Difficulty: ★☆☆☆  warm-up    Source: Original
// FENCE: Use only what chapters 1-19 cover.
//        Std only.
// ===========================================================================
//
// TASK
// For each numbered line in main, uncomment it alone and predict: compiles,
// compiles with a warning, or error - and why. Then check.
// Afterwards, write a function whose PARAMETERS destructure a struct.
//
// CHECK YOUR UNDERSTANDING
// 1. Define refutable and irrefutable in one sentence each.
// 2. Why is `if let x = 5` only a warning, but `let Some(x) = opt;` an error?
// 3. Why does `let [first, ..] = arr` work while the slice version doesn't?
//
// ===========================================================================
// MY ANSWERS (write them before asking Claude to review)
// 1.
// 2.
// 3.
// ===========================================================================

#![allow(unused)]

fn takes(&(x, y): &(i32, i32)) -> i32 {
    x + y
}

fn main() {
    let opt: Option<i32> = Some(3);
    let mut stack = vec![1, 2, 3];
    let pairs = vec![(1, 2), (3, 4)];
    let arr = [1, 2, 3];
    let slice: &[i32] = &arr;

    // 1.  let Some(x) = opt;
    // 2.  let Some(x) = opt else { return; };
    // 3.  if let x = 5 { println!("{x}"); }
    // 4.  let (a, b) = (1, 2, 3);
    // 5.  let (a, .., b) = (1, 2, 3);
    // 6.  while let Some(top) = stack.pop() { println!("{top}"); }
    // 7.  for (i, (a, b)) in pairs.iter().enumerate() { println!("{i} {a} {b}"); }
    // 8.  let [first, ..] = arr;
    // 9.  let [first, ..] = slice;
    // 10. let Ok(n) = "5".parse::<u8>();
    // 11. let Ok(n): Result<u8, std::convert::Infallible> = Ok(5);
    // 12. println!("{}", takes(&(1, 2)));
}
