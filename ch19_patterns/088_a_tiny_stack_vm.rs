// ===========================================================================
// 088 · A Tiny Stack VM
// Chapter 19: Patterns and Matching
// Difficulty: ★★★☆  challenging    Source: Original
// FENCE: Use only what chapters 1-19 cover.
//        Std only.
// ===========================================================================
//
// TASK
//   enum Instr { Push(i64), Pop, Add, Sub, Mul, Div, Dup, Swap, Over,
//                Jmp(usize), Jz(usize), Call(usize), Ret, Print, Halt }
// - `fn step(&mut self) -> Result<Control, VmError>` whose core is ONE match on
//   (instruction, the stack as a slice) - e.g. (Instr::Add, [.., a, b]).
// - Programs: count down from 10; factorial of 10 (a loop with Jz); print the
//   Fibonacci numbers below 1000 using Call/Ret and a return stack.
// - Errors carry the program counter: StackUnderflow, DivisionByZero,
//   BadJump.
// - Bonus: an assembler for text with labels ("loop:", "jz end").
//
// CHECK YOUR UNDERSTANDING
// 1. What borrow problem did matching on the stack slice cause when you then
//    needed to push or pop? How did you solve it?
// 2. One big match vs a method per instruction: when does the big match stop
//    being a good idea?
//
// ===========================================================================
// MY ANSWERS (write them before asking Claude to review)
// 1.
// 2.
// ===========================================================================

fn main() {
    // Your code here.
}
