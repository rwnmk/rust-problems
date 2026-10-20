// ===========================================================================
// 071 · Smart Pointer Mechanics
// Chapter 15: Smart Pointers
// Difficulty: ★☆☆☆  warm-up    Source: Rust Book ch. 15
// FENCE: Use only what chapters 1-15 cover.
//        Not yet: threads, async, dyn trait objects (beyond Box<dyn Error>).
// ===========================================================================
//
// TASK
// - `struct Loud<T> { name: &'static str, value: T }` with
//   `fn new(name, value) -> Self`, implementing Deref, DerefMut, and Drop
//   (prints "dropping {name}").
// - Show deref coercion: call a function taking &str with a &Loud<String>.
// - Predict the full drop order of main below in a comment, THEN run it.
//
// CHECK YOUR UNDERSTANDING
// 1. Why does `let _ = x` drop immediately but `let _kept = x` doesn't?
// 2. Does shadowing drop the shadowed value? When does it go?
// 3. Why can't you call `.drop()` yourself?
//
// ===========================================================================
// MY ANSWERS (write them before asking Claude to review)
// 1.
// 2.
// 3.
// ===========================================================================

// Implement Loud first, then predict.

fn main() {
    // let a = Loud::new("a", 1);
    // let b = Loud::new("b", String::from("bee"));
    // {
    //     let _c = Loud::new("c", 3);
    //     let a = Loud::new("a-shadow", 4);
    //     println!("inner scope ends");
    // }
    // let _v = vec![Loud::new("v0", 0), Loud::new("v1", 1)];
    // let _moved = b;
    // drop(a);
    // Loud::new("temp", 5);
    // let _ = Loud::new("underscore", 6);
    // let _kept = Loud::new("kept", 7);
    // takes(Loud::new("arg", 8));
    // println!("main ends");
}

// fn takes<T>(_x: Loud<T>) {
//     println!("in takes");
// }

// PREDICTED ORDER:
