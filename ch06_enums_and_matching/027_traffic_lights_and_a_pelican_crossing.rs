// ===========================================================================
// 027 · Traffic Lights and a Pelican Crossing
// Chapter 6: Enums and Pattern Matching
// Difficulty: ★★☆☆  core    Source: Original
// FENCE: Use only what chapters 1-6 cover.
//        Not yet: Vec, HashMap, `?`, generics, traits, closures.
// ===========================================================================
//
// TASK
// Part 1: `enum Light { Red, RedAmber, Green, Amber }` (the UK sequence) with
// `fn next(self) -> Light` and `fn duration_secs(&self) -> u32`.
//
// Part 2: a pedestrian crossing as a state machine:
//   enum Crossing {
//       Idle,
//       Waiting { pressed_at: u32 },
//       CarsStopping { remaining: u32 },
//       Walk { remaining: u32 },
//       Flashing { remaining: u32 },
//   }
// `fn tick(self, now: u32, button: bool) -> Crossing`, where:
// - Idle + button -> Waiting { pressed_at: now }; presses while Waiting are
//   ignored.
// - Waiting -> CarsStopping { 3 } once 5 seconds have passed since the press.
// - CarsStopping -> Walk { 8 } -> Flashing { 4 } -> Idle, each counting down.
// Simulate 60 seconds with presses at t = 5 and t = 30; print each transition.
//
// CHECK YOUR UNDERSTANDING
// 1. Why take self by value in next/tick rather than &mut self? Trade-offs?
// 2. How did you match a struct-like variant and bind its field?
// 3. What invalid states does this design rule out compared with a struct full
//    of bools and counters?
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
