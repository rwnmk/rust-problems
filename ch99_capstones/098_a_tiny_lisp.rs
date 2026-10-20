// ===========================================================================
// 098 · A Tiny Lisp
// Capstone
// Difficulty: ★★★★  stretch    Source: SICP ch. 4
// FENCE: The whole Book. Everything. Std only unless stated.
// ===========================================================================
//
// TASK
// An interpreter for a small Scheme:
// - tokenizer -> parser -> eval, with an enum Value covering numbers, symbols,
//   booleans, lists, lambdas (params, body, captured environment), builtins
//   (fn pointers) and nil.
// - Lexically scoped environments: Rc<RefCell<Env>> with parent links.
// - Special forms: define, lambda, if, let, begin, quote, set!. Builtins: + - *
//   / = < > car cdr cons list null?
// - A REPL where errors never crash the interpreter.
// - Must run:
//     (define (fact n) (if (< n 2) 1 (* n (fact (- n 1)))))
//     (fact 20)
//     (define (make-counter) (let ((n 0)) (lambda () (set! n (+ n 1)) n)))
//     (define c (make-counter))
//     (c) (c)                     ; 1 then 2
//     (define (map f xs) (if (null? xs) '() (cons (f (car xs)) (map f (cdr xs)))))
//     (map (lambda (x) (* x x)) '(1 2 3))
// - Bonus: tail calls, so (define (loop n) (if (= n 0) 'done (loop (- n 1))))
//   (loop 1000000) doesn't overflow the stack.
//
// CHECK YOUR UNDERSTANDING
// 1. Why do closures like make-counter need shared, mutable environments, and
//    which Rust types gave you that?
// 2. Does a recursive function defined at top level form an Rc cycle with the
//    environment it captures? Does it leak? How could you check?
// 3. How did you implement tail calls?
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
