// ===========================================================================
// 086 · Pattern Zoo
// Chapter 19: Patterns and Matching
// Difficulty: ★☆☆☆  warm-up    Source: Original
// FENCE: Use only what chapters 1-19 cover.
//        Std only.
// ===========================================================================
//
// TASK
// For each input listed in main, predict what classify returns. Write your
// predictions down, THEN run it. Afterwards, add five patterns of your own
// that destructure nested structs and enums (e.g. Shape::Rect { w, h } if
// w == h).
//
// CHECK YOUR UNDERSTANDING
// 1. Why does arm order matter? Which outputs change if you swap arms 2 and 3?
// 2. Why must both sides of an | pattern bind the same names and types?
// 3. The compiler doesn't count guards when checking exhaustiveness. Why not?
//
// ===========================================================================
// MY ANSWERS (write them before asking Claude to review)
// 1.
// 2.
// 3.
// ===========================================================================

fn classify(v: (i32, Option<&str>, [u8; 3])) -> String {
    match v {
        (0, None, _) => "zero, nothing".to_string(),
        (n @ 1..=9, Some(s), [a, .., c]) if a == c => format!("small {n}, {s}, bookends {a}"),
        (n, Some("magic"), _) | (n, None, [_, 42, _]) => format!("special {n}"),
        (n, Some(s), [first, rest @ ..]) if s.len() > 3 => {
            format!("{n} long {s} starts {first} then {rest:?}")
        }
        (i32::MIN..=-1, _, _) => "negative".to_string(),
        (n, _, [x, y, z]) => format!("{n} sum {}", x as u32 + y as u32 + z as u32),
    }
}

fn main() {
    let inputs = [
        (0, None, [1, 2, 3]),
        (5, Some("hi"), [7, 0, 7]),
        (5, Some("hi"), [7, 0, 8]),
        (-3, Some("magic"), [0, 0, 0]),
        (12, None, [0, 42, 0]),
        (12, Some("hello"), [9, 8, 7]),
        (-7, Some("hello"), [1, 1, 1]),
        (-7, None, [1, 2, 3]),
        (0, Some("magic"), [1, 2, 1]),
        (3, Some("magic"), [4, 4, 4]),
    ];
    // PREDICTIONS (one per input, in order):
    //
    for v in inputs {
        println!("{v:?} => {}", classify(v));
    }
}
