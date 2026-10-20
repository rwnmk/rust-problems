// ===========================================================================
// 041 · Config File Loader
// Chapter 9: Error Handling
// Difficulty: ★☆☆☆  warm-up    Source: Original
// FENCE: Use only what chapters 1-9 cover.
//        Not yet: generics, your own traits, closures (.map_err etc.).
// ===========================================================================
//
// TASK
// Load a file like:
//   # comment
//   name = My App
//   port = 8080
//   debug = true
// - `fn read_config(path: &str) -> Result<String, io::Error>` using `?`
// - `fn parse_config(text: &str) -> Result<HashMap<String, String>, String>`:
//   error messages include the line number for lines without '='; duplicate
//   keys are an error; blank lines and # comments are skipped.
// - `fn get_port(cfg: &HashMap<String, String>) -> Result<u16, String>`
// - main returns Result<(), Box<dyn Error>> and uses `?` throughout.
// Try: a missing file, a malformed line, port = 99999, no port at all.
//
// CHECK YOUR UNDERSTANDING
// 1. Why does `?` work on both io::Error and String inside main? What
//    conversion is happening?
// 2. What gets printed, and what exit code results, when main returns Err?
//    (Check with `echo $?`.)
// 3. Why does parse_config take text rather than a path?
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
