// ===========================================================================
// 091 · Advanced Traits Tour
// Chapter 20: Advanced Features
// Difficulty: ★★☆☆  core    Source: Rust Book ch. 20
// FENCE: Use only what chapters 1-20 cover.
//        The whole Book is fair game. Std only unless stated.
// ===========================================================================
//
// TASK
// 1. Newtype: `struct Lines(Vec<String>)` with Display (one per line) and
//    Deref<Target = Vec<String>>. Is the Deref a good idea? Argue it.
// 2. Fully qualified syntax: a type with its own fly() plus Pilot::fly and
//    Wizard::fly; call all three. Then the associated-function version (no
//    self) that needs <Type as Trait>::name().
// 3. Supertrait: `trait Boxed: Display` with a default method drawing a box
//    around the Display output - correctly for MULTI-LINE output.
// 4. Associated types: `trait Graph { type Node; fn neighbours(&self, n:
//    &Self::Node) -> Vec<Self::Node>; }` and a BFS generic over any Graph.
//    Implement it for an adjacency list and for a grid maze read from text.
// 5. Default type parameters: Millimetres + Metres via Add<Metres>.
//
// CHECK YOUR UNDERSTANDING
// 1. When do you pick an associated type over a generic parameter? Why would
//    Graph<N> be worse here?
// 2. What does a newtype cost at run time?
// 3. Why is implementing Deref on a newtype controversial?
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
