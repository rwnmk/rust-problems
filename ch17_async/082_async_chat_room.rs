// ===========================================================================
// 082 · Async Chat Room
// Chapter 17: Async and Await
// Difficulty: ★★★★  stretch    Source: Original
// FENCE: Use only what chapters 1-17 cover.
//        Allowed crate: trpl (the Book's helper). Not yet: tokio directly.
// ===========================================================================
//
// TASK
// Simulate a chat server in one process with async tasks and channels:
// - A broker task owns all room state (user -> sender) and receives Join,
//   Leave and Message events on one channel.
// - N user tasks each join, send a few messages at random intervals, receive
//   everyone else's messages on their own channel, and leave.
// - Messages reach everyone except the sender, in order per sender.
// - Add a SLOW user who reads slowly. What happens to the broker? Fix it so
//   one slow user can't stall the room, and explain what you traded away.
// - Expose room events as a Stream and use stream adapters to build a
//   moderator that prints only messages containing a keyword, for 5 seconds.
//
// CHECK YOUR UNDERSTANDING
// 1. Why is one broker task owning the state simpler than Arc<Mutex<...>>?
// 2. How did you handle the slow consumer, and what did it cost?
// 3. What is the essential difference between a Stream and an Iterator?
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
