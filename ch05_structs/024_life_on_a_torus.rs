// ===========================================================================
// 024 · Life on a Torus
// Chapter 5: Using Structs
// Difficulty: ★★★☆  challenging    Source: Conway's Game of Life
// FENCE: Use only what chapters 1-5 cover.
//        Not yet: enums/Option, Vec, HashMap, traits beyond #[derive], closures.
// ===========================================================================
//
// TASK
//   const W: usize = 20;  const H: usize = 10;
//   struct Grid { cells: [[bool; W]; H] }
// - `Grid::new()`, `set(&mut self, x, y, alive)`, `live_neighbours(&self, x, y)
//   -> u8` with WRAP-AROUND edges, `step(&self) -> Grid`, `render(&self)` using
//   '#' and '.'.
// - Seed a glider and animate 40 generations: clear the screen with
//   print!("\x1B[2J\x1B[H") and pause with
//   std::thread::sleep(std::time::Duration::from_millis(100)).
// - Verify in code: after 4 generations a glider is the same shape shifted by
//   one cell diagonally. Write `fn shifted(&self, dx, dy) -> Grid` and compare
//   grids with a derived PartialEq.
//
// CHECK YOUR UNDERSTANDING
// 1. Why must step build a NEW grid rather than update cells in place?
// 2. How did you wrap around at x = 0 using usize without underflowing?
// 3. Should Grid derive Copy? What does a copy cost here?
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
