// ===========================================================================
// 031 · Break Up the Monolith
// Chapter 7: Packages, Crates, and Modules
// Difficulty: ★★☆☆  core    Source: Original
// FENCE: Use only what chapters 1-7 cover.
//        Not yet: collections, `?`, generics, traits.
// ===========================================================================
//
// TASK
// The code below all lives in one file. Restructure it into a Cargo project:
//   src/main.rs
//   src/geometry.rs            (or geometry/mod.rs - pick one and justify it)
//   src/geometry/shapes.rs     Point, Circle, Rect and the area functions
//   src/geometry/transform.rs  translate_point, scale_circle
//   src/display.rs             fmt_point, fmt_circle, internal_round
// Requirements:
// - Circle's fields are public. Rect's fields are PRIVATE, so add a
//   `Rect::new` constructor and whatever accessors main needs.
// - internal_round must not be visible outside display.
// - Use the fewest `pub`s that still compile. main uses `use` statements, not
//   full paths everywhere.
//
// CHECK YOUR UNDERSTANDING
// 1. Why can main construct a Circle with a struct literal but not a Rect?
//    How is this different for enum variants?
// 2. What's the difference between `mod geometry;` and `use geometry;`?
// 3. When transform.rs needs Point, which path did you use: crate::, super::,
//    or something else? Why?
//
// ===========================================================================
// MY ANSWERS (write them before asking Claude to review)
// 1.
// 2.
// 3.
// ===========================================================================

#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
}

#[derive(Debug)]
struct Circle {
    centre: Point,
    radius: f64,
}

#[derive(Debug)]
struct Rect {
    top_left: Point,
    width: f64,
    height: f64,
}

fn circle_area(c: &Circle) -> f64 {
    std::f64::consts::PI * c.radius * c.radius
}

fn rect_area(r: &Rect) -> f64 {
    r.width * r.height
}

fn translate_point(p: &mut Point, dx: f64, dy: f64) {
    p.x += dx;
    p.y += dy;
}

fn scale_circle(c: &mut Circle, k: f64) {
    c.radius *= k;
}

// Helper: should NOT be visible outside the display module.
fn internal_round(x: f64) -> f64 {
    (x * 100.0).round() / 100.0
}

fn fmt_point(p: &Point) -> String {
    format!("({}, {})", internal_round(p.x), internal_round(p.y))
}

fn fmt_circle(c: &Circle) -> String {
    format!("Circle at {} r={}", fmt_point(&c.centre), internal_round(c.radius))
}

fn main() {
    let mut c = Circle {
        centre: Point { x: 1.0, y: 2.0 },
        radius: 1.5,
    };
    let r = Rect {
        top_left: Point { x: 0.0, y: 0.0 },
        width: 3.0,
        height: 4.0,
    };
    translate_point(&mut c.centre, 0.333, -1.0);
    scale_circle(&mut c, 2.0);
    println!("{}", fmt_circle(&c));
    println!("circle area {:.3}", circle_area(&c));
    println!("rect area {:.3} at {}", rect_area(&r), fmt_point(&r.top_left));
}
