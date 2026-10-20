// ===========================================================================
// 095 · Web Server Features
// Chapter 21: Final Project: Web Server
// Difficulty: ★★★☆  challenging    Source: Rust Book ch. 21 extension
// FENCE: Use only what chapters 1-21 cover.
//        Std only unless stated.
// ===========================================================================
//
// TASK
// Starting from the Book's finished multithreaded server:
// - A router mapping (method, path pattern) to handler function pointers
//   `fn(&Request) -> Response`, with path params: /hello/:name.
// - Request parsing: method, path, query string with percent-decoding,
//   headers, and a body using Content-Length (POST /echo).
// - Static files from ./public with correct Content-Type - and REJECT path
//   traversal (/../../etc/passwd, and the %2e%2e encoded form). Test it.
// - 404, 405 for a wrong method, 400 for malformed requests, correct
//   Content-Length on every response.
// - Exercise it with curl and a browser.
//
// CHECK YOUR UNDERSTANDING
// 1. How did you block path traversal: string checks or canonicalisation?
//    What are the pitfalls of each?
// 2. fn pointers vs Box<dyn Fn> for handlers: what can closures capture that
//    fn pointers can't, and did you need it?
//
// ===========================================================================
// MY ANSWERS (write them before asking Claude to review)
// 1.
// 2.
// ===========================================================================

fn main() {
    // Your code here.
}
