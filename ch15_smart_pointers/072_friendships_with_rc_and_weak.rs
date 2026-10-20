// ===========================================================================
// 072 · Friendships with Rc and Weak
// Chapter 15: Smart Pointers
// Difficulty: ★★☆☆  core    Source: Rust Book ch. 15
// FENCE: Use only what chapters 1-15 cover.
//        Not yet: threads, async, dyn trait objects (beyond Box<dyn Error>).
// ===========================================================================
//
// TASK
//   struct Person { name: String, friends: RefCell<Vec<Weak<Person>>> }
// - `fn befriend(a: &Rc<Person>, b: &Rc<Person>)` (mutual).
// - `fn friends_of_friends(p: &Rc<Person>) -> Vec<String>`, excluding p and p's
//   direct friends, with no duplicates.
// - Print strong_count and weak_count at each step.
// - FIRST build it with RefCell<Vec<Rc<Person>>>, give Person a Drop impl that
//   prints, and show that two friends are never dropped. Then switch to Weak
//   and show they are.
// - Drop one person's last Rc. What do their friends see when they upgrade?
//
// CHECK YOUR UNDERSTANDING
// 1. Why does an Rc cycle leak? Isn't Rust supposed to prevent leaks?
// 2. When does upgrade() return None?
// 3. Why is the RefCell inside Person rather than using Rc<RefCell<Person>>?
//    What can you mutate with each?
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
