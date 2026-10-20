// ===========================================================================
// 013 · Predict the Borrow Checker
// Chapter 4: Understanding Ownership
// Difficulty: ★☆☆☆  warm-up    Source: Original
// FENCE: Use only what chapters 1-4 cover.
//        Not yet: structs, enums/Option, Vec, HashMap, closures, iterator adapters.
// ===========================================================================
//
// TASK
// Ten snippets below. For each one: uncomment it, then BEFORE compiling write
// your prediction under it (compiles? if not, which line errors, and which
// ownership/borrowing rule is broken?). Compile, compare, re-comment.
//
// CHECK YOUR UNDERSTANDING
// 1. Which snippets compile only because of WHERE the last use of a reference
//    is? What is that feature of the borrow checker called?
// 2. snippet_h: you own `s`, so why can't you move out of `*t`?
// 3. For each failing snippet, give the smallest change that makes it compile,
//    and say what it costs (an extra allocation? a restriction?).
//
// ===========================================================================
// MY ANSWERS (write them before asking Claude to review)
// 1.
// 2.
// 3.
// ===========================================================================

fn snippet_a() {
    // let s1 = String::from("hello");
    // let s2 = s1;
    // println!("{s1}, {s2}");
    // PREDICTION:
}

fn snippet_b() {
    // let x = 5;
    // let y = x;
    // println!("{x}, {y}");
    // PREDICTION:
}

fn snippet_c() {
    // let s = String::from("hi");
    // takes(s);
    // takes(s);
    // PREDICTION:
}

fn snippet_d() {
    // let mut s = String::from("hello");
    // let r1 = &mut s;
    // let r2 = &mut s;
    // r1.push('!');
    // r2.push('?');
    // PREDICTION:
}

fn snippet_e() {
    // let mut s = String::from("hello");
    // let r1 = &s;
    // println!("{r1}");
    // let r2 = &mut s;
    // r2.push('!');
    // PREDICTION:
}

fn snippet_f() {
    // let mut s = String::from("abc");
    // let first = &s[0..1];
    // s.push_str("def");
    // println!("{first}");
    // PREDICTION:
}

fn snippet_g() {
    // let r;
    // {
    //     let s = String::from("inner");
    //     r = &s;
    // }
    // println!("{r}");
    // PREDICTION:
}

fn snippet_h() {
    // let s = String::from("hello");
    // let t = &s;
    // let u = *t;
    // println!("{u}");
    // PREDICTION:
}

fn snippet_i() {
    // let mut s = String::from("hi");
    // let len = calc_len(&s);
    // s.push_str(" there");
    // println!("{len} {s}");
    // PREDICTION:
}

// snippet_j: does this function compile?
// fn dangle() -> &String {
//     let s = String::from("dangling");
//     &s
// }
// PREDICTION:

#[allow(dead_code)]
fn takes(_s: String) {}

#[allow(dead_code)]
fn calc_len(s: &String) -> usize {
    s.len()
}

fn main() {
    snippet_a();
    snippet_b();
    snippet_c();
    snippet_d();
    snippet_e();
    snippet_f();
    snippet_g();
    snippet_h();
    snippet_i();
}
