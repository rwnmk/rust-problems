// ===========================================================================
// 061 · Closure Zoo
// Chapter 13: Iterators and Closures
// Difficulty: ★★☆☆  core    Source: Original
// FENCE: Use only what chapters 1-13 cover.
//        Not yet: Box/Rc/RefCell, threads, dyn trait objects.
// ===========================================================================
//
// TASK
// 1. `fn make_adder(n: i32) -> impl Fn(i32) -> i32`
// 2. `fn make_counter() -> impl FnMut() -> u32`
// 3. `fn compose<A, B, C>(f: impl Fn(A) -> B, g: impl Fn(B) -> C)
//        -> impl Fn(A) -> C`
// 4. `fn apply_n<T>(f: impl Fn(T) -> T, n: usize, x: T) -> T` - use it to get
//    2^10 from a doubler, and to run Newton's sqrt improvement (011) 6 times.
// 5. For each closure c1..c6 in main, write which of Fn / FnMut / FnOnce it
//    implements. Then uncomment the calls one at a time and check.
// 6. Why does `let c = make_counter(); c();` fail? Fix it.
//
// CHECK YOUR UNDERSTANDING
// 1. c4 is a `move` closure but still Fn. So what does `move` control?
// 2. What is n after c5 has run twice, and why?
// 3. Why must a caller bind make_counter's result with `mut`?
//
// ===========================================================================
// MY ANSWERS (write them before asking Claude to review)
// 1.
// 2.
// 3.
// ===========================================================================

#![allow(unused)]

fn requires_fn<F: Fn()>(f: F) {
    f();
    f();
}

fn requires_fn_mut<F: FnMut()>(mut f: F) {
    f();
    f();
}

fn requires_fn_once<F: FnOnce()>(f: F) {
    f();
}

fn main() {
    let greeting = String::from("hello");
    let c1 = || println!("{greeting}");

    let mut log: Vec<i32> = Vec::new();
    let mut c2 = || log.push(1);

    let owned = String::from("consumed");
    let c3 = move || drop(owned);

    let data = vec![1, 2, 3];
    let c4 = move || println!("{data:?}");

    let mut n = 0;
    let c5 = move || {
        n += 1;
        println!("inside: {n}");
    };

    let s = String::from("x");
    let c6 = || {
        let t = s;
        println!("{t}");
    };

    // c1:      c2:      c3:      c4:      c5:      c6:

    // requires_fn(c1);
    // requires_fn(c2);
    // requires_fn_mut(c2);
    // requires_fn_mut(c3);
    // requires_fn_once(c3);
    // requires_fn(c4);
    // requires_fn_mut(c5); println!("outside: {n}");
    // requires_fn_mut(c6);
    // requires_fn_once(c6);
}
