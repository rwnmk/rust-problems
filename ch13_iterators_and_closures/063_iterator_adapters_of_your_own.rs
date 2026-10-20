// ===========================================================================
// 063 · Iterator Adapters of Your Own
// Chapter 13: Iterators and Closures
// Difficulty: ★★★☆  challenging    Source: Original
// FENCE: Use only what chapters 1-13 cover.
//        Not yet: Box/Rc/RefCell, threads, dyn trait objects.
// ===========================================================================
//
// TASK
// Build an extension trait so these work on ANY iterator:
//   trait MyIterExt: Iterator + Sized {
//       fn every_nth(self, n: usize) -> EveryNth<Self>;
//       fn dedup_consecutive(self) -> Dedup<Self> where Self::Item: PartialEq;
//       fn pairwise(self) -> Pairwise<Self> where Self::Item: Clone;
//       fn my_map<B, F: FnMut(Self::Item) -> B>(self, f: F) -> MyMap<Self, F>;
//   }
//   impl<I: Iterator> MyIterExt for I {}
// (Copy that first line as given; `: Iterator + Sized` is a supertrait,
// explained in ch. 20.) pairwise yields (a, b), (b, c), ... Each adapter is a
// struct implementing Iterator, and must be lazy - prove it with println!s in
// the source iterator.
//
// CHECK YOUR UNDERSTANDING
// 1. Why does the trait need Sized?
// 2. Why does the PartialEq bound sit on the method rather than the trait?
// 3. How did you demonstrate laziness?
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
