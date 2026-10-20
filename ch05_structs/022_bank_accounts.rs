// ===========================================================================
// 022 · Bank Accounts
// Chapter 5: Using Structs
// Difficulty: ★★☆☆  core    Source: Original
// FENCE: Use only what chapters 1-5 cover.
//        Not yet: enums/Option, Vec, HashMap, traits beyond #[derive], closures.
// ===========================================================================
//
// TASK
// `struct Account { owner: String, bank: String, balance: i64, overdraft: i64 }`
// where money is in pence.
// - `Account::new(owner: &str, bank: &str) -> Account`
// - deposit(&mut self, pence), withdraw(&mut self, pence) -> bool (refuses to
//   go past the overdraft limit), balance(&self)
// - `fn formatted(&self) -> String` giving e.g. "£12.34" or "-£0.05"
// - `fn transfer(&mut self, to: &mut Account, pence: i64) -> bool`
// Then:
// - Try `a.transfer(&mut a, 100);` and explain the error.
// - Create `let joint = Account { owner: String::from("Joint"), ..alice };`
//   then try using each of alice's fields afterwards.
//
// CHECK YOUR UNDERSTANDING
// 1. Why does the self-transfer not compile? Is there a real bug it prevents?
// 2. After `..alice`, which fields of alice are still usable, and why?
// 3. Why store money as integer pence rather than f64 pounds?
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
