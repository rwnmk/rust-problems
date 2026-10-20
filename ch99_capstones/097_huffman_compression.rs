// ===========================================================================
// 097 · Huffman Compression
// Capstone
// Difficulty: ★★★★  stretch    Source: SICP §2.3.4
// FENCE: The whole Book. Everything. Std only unless stated.
// ===========================================================================
//
// TASK
// - Count byte frequencies and build a Huffman tree with BinaryHeap (via
//   std::cmp::Reverse or your own Ord).
// - File format: a header (the tree, or code lengths) followed by a bit-packed
//   payload. Write your own BitWriter and BitReader.
// - `huff compress IN OUT` and `huff decompress IN OUT`; byte-for-byte
//   round-trip; report the compression ratio.
// - Edge cases: an empty file, a file with one distinct byte, binary files.
// - Property tests (056) for round-trip.
// - Bonus: canonical Huffman codes so the header is just 256 code lengths.
//
// CHECK YOUR UNDERSTANDING
// 1. Sketch why Huffman coding is optimal among prefix codes.
// 2. How did you deal with the final partial byte?
//
// ===========================================================================
// MY ANSWERS (write them before asking Claude to review)
// 1.
// 2.
// ===========================================================================

fn main() {
    // Your code here.
}
