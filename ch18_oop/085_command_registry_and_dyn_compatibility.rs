// ===========================================================================
// 085 · Command Registry and dyn Compatibility
// Chapter 18: Object-Oriented Programming Features
// Difficulty: ★★★☆  challenging    Source: Original
// FENCE: Use only what chapters 1-18 cover.
//        Std only.
// ===========================================================================
//
// TASK
//   trait Command {
//       fn name(&self) -> &str;
//       fn help(&self) -> &str;
//       fn run(&mut self, args: &[&str]) -> Result<String, String>;
//   }
// - A REPL with a HashMap<String, Box<dyn Command>> registry dispatching on the
//   first word. Commands: echo, count (counts its own invocations), calc
//   (your RPN calculator from 043), history, and help (lists all commands).
//   history and help need state the registry owns - design for that.
// - Try adding to the trait: (a) `fn new() -> Self`, (b) a generic method
//   `fn run_with<T: Display>(&self, x: T)`, (c) `fn clone_box(&self) ->
//   Box<dyn Command>`. Which break dyn compatibility, and why? Rescue (a) with
//   `where Self: Sized`.
// - Implement Clone for Box<dyn Command> via clone_box and add an `alias`
//   command that registers a copy of a command under a new name.
//
// CHECK YOUR UNDERSTANDING
// 1. Why can't a dyn-compatible trait have generic methods?
// 2. What does `where Self: Sized` on a method do?
// 3. How did help and history read registry state without borrow conflicts?
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
