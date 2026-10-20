# Rust Book Problem Set: 100 problems, chapter by chapter

One folder per chapter of *The Rust Programming Language* (2024-edition numbering: ch. 17 is async,
ch. 18 OOP, ch. 19 patterns, ch. 20 advanced features, ch. 21 the web server), plus a capstone folder.
Each `.rs` file states the problem in a header comment and ends with a stub you can drop into a fresh project.

## How to use it

1. Read the chapter. Then do its problems **in order** before moving on.
2. Start a problem: `cargo new p012 && cp ch03_common_concepts/012_*.rs p012/src/main.rs`
   (a few problems - ch. 7, 11, 14, 21 - ask you to lay out a library, tests/ folder or workspace instead).
3. **Respect the fence.** Every header has a `FENCE:` line saying which chapters you may draw on and what
   is explicitly off-limits. If you find yourself wanting a later feature, that's the point: solve it with
   what you have, then note in a comment what you'd reach for later. Several later problems ask you to come
   back and redo an earlier one with new tools (e.g. 015 -> 026, 017 -> 039, 057 -> 060, 047 -> 083).
4. **Answer the "CHECK YOUR UNDERSTANDING" questions** in the `MY ANSWERS` block, in your own words,
   *before* asking for a review. Then paste the file to Claude and ask it to review your code and probe
   your answers.
5. A chapter is done when every problem works, you can explain every answer without looking anything up,
   and a reviewer couldn't find a gap.

Provided code (predict-the-compiler snippets in 013, 033, 049, 061, 071, 086, 089; the buggy functions in
055; the monolith in 031) is part of the problem, not a solution.

## Difficulty

| Stars | Meaning |
|---|---|
| ★☆☆☆ | Warm-up: direct application of the chapter |
| ★★☆☆ | Core: needs the chapter's ideas to really click |
| ★★★☆ | Challenging: combines ideas, has a trap or two |
| ★★★★ | Stretch: tricky even for someone who's just finished the Book |

Each chapter ends on its hardest problem, so if that one feels comfortable, you've earned the next chapter.

## Index


### Ch. 1 - Getting Started

| # | Problem | Difficulty | Source |
|---|---|---|---|
| 001 | [Hello, Cargo](ch01_getting_started/001_hello_cargo.rs) | ★ | Rust Book ch. 1 |
| 002 | [Read the Compiler](ch01_getting_started/002_read_the_compiler.rs) | ★ | Original |

### Ch. 2 - Programming a Guessing Game

| # | Problem | Difficulty | Source |
|---|---|---|---|
| 003 | [Echo and Greet](ch02_guessing_game/003_echo_and_greet.rs) | ★ | Rust Book ch. 2 style |
| 004 | [Reverse Guessing Game](ch02_guessing_game/004_reverse_guessing_game.rs) | ★★ | Inspired by the Rust Book ch. 2 |
| 005 | [Guessing Game Plus](ch02_guessing_game/005_guessing_game_plus.rs) | ★★ | Rust Book ch. 2 extension |

### Ch. 3 - Common Programming Concepts

| # | Problem | Difficulty | Source |
|---|---|---|---|
| 006 | [Temperature Tables](ch03_common_concepts/006_temperature_tables.rs) | ★ | Rust Book ch. 3 suggestion; K&R §1.2 |
| 007 | [Fibonacci to the Edge](ch03_common_concepts/007_fibonacci_to_the_edge.rs) | ★★ | Rust Book ch. 3 suggestion; Project Euler 2 |
| 008 | [The Twelve Days of Christmas](ch03_common_concepts/008_the_twelve_days_of_christmas.rs) | ★★ | Rust Book ch. 3 suggestion |
| 009 | [Multiples of 3 or 5](ch03_common_concepts/009_multiples_of_3_or_5.rs) | ★ | Project Euler 1 |
| 010 | [Primes and the Stack](ch03_common_concepts/010_primes_and_the_stack.rs) | ★★ | Project Euler 7 and 10 |
| 011 | [Square Roots by Newton's Method](ch03_common_concepts/011_square_roots_by_newton_s_method.rs) | ★★ | SICP §1.1.7, Exercises 1.7 and 1.8 |
| 012 | [Longest Collatz Chain](ch03_common_concepts/012_longest_collatz_chain.rs) | ★★★ | Project Euler 14 |

### Ch. 4 - Understanding Ownership

| # | Problem | Difficulty | Source |
|---|---|---|---|
| 013 | [Predict the Borrow Checker](ch04_ownership/013_predict_the_borrow_checker.rs) | ★ | Original |
| 014 | [Words as Slices](ch04_ownership/014_words_as_slices.rs) | ★★ | Rust Book ch. 4 extension |
| 015 | [Slice Tools](ch04_ownership/015_slice_tools.rs) | ★★ | Original |
| 016 | [Three Ways to Shout](ch04_ownership/016_three_ways_to_shout.rs) | ★ | Original |
| 017 | [Longest Run](ch04_ownership/017_longest_run.rs) | ★★ | Original |
| 018 | [Two Pointers, In Place](ch04_ownership/018_two_pointers_in_place.rs) | ★★ | LeetCode 26, 27, 283 |
| 019 | [String Compression](ch04_ownership/019_string_compression.rs) | ★★★ | LeetCode 443 |

### Ch. 5 - Using Structs

| # | Problem | Difficulty | Source |
|---|---|---|---|
| 020 | [Rectangles, Extended](ch05_structs/020_rectangles_extended.rs) | ★ | Rust Book ch. 5 extension |
| 021 | [Points, Segments, Units](ch05_structs/021_points_segments_units.rs) | ★ | Original |
| 022 | [Bank Accounts](ch05_structs/022_bank_accounts.rs) | ★★ | Original |
| 023 | [Rational Numbers](ch05_structs/023_rational_numbers.rs) | ★★ | SICP §2.1.1 |
| 024 | [Life on a Torus](ch05_structs/024_life_on_a_torus.rs) | ★★★ | Conway's Game of Life |

### Ch. 6 - Enums and Pattern Matching

| # | Problem | Difficulty | Source |
|---|---|---|---|
| 025 | [Making Change in Sterling](ch06_enums_and_matching/025_making_change_in_sterling.rs) | ★ | Rust Book ch. 6 (UK edition) |
| 026 | [Option Without Unwrap](ch06_enums_and_matching/026_option_without_unwrap.rs) | ★ | Original |
| 027 | [Traffic Lights and a Pelican Crossing](ch06_enums_and_matching/027_traffic_lights_and_a_pelican_crossing.rs) | ★★ | Original |
| 028 | [Rock Paper Scissors Lizard Spock](ch06_enums_and_matching/028_rock_paper_scissors_lizard_spock.rs) | ★ | Original |
| 029 | [Who Wins the Trick?](ch06_enums_and_matching/029_who_wins_the_trick.rs) | ★★ | Whist rules |
| 030 | [Tic-Tac-Toe](ch06_enums_and_matching/030_tic_tac_toe.rs) | ★★★ | Classic; minimax |

### Ch. 7 - Packages, Crates, and Modules

| # | Problem | Difficulty | Source |
|---|---|---|---|
| 031 | [Break Up the Monolith](ch07_packages_and_modules/031_break_up_the_monolith.rs) | ★★ | Original |
| 032 | [Library Plus Binaries](ch07_packages_and_modules/032_library_plus_binaries.rs) | ★ | Original |
| 033 | [Privacy Puzzle](ch07_packages_and_modules/033_privacy_puzzle.rs) | ★★ | Original |

### Ch. 8 - Common Collections

| # | Problem | Difficulty | Source |
|---|---|---|---|
| 034 | [Median and Mode](ch08_collections/034_median_and_mode.rs) | ★ | Rust Book ch. 8 suggestion |
| 035 | [Pig Latin, Properly](ch08_collections/035_pig_latin_properly.rs) | ★★ | Rust Book ch. 8 suggestion |
| 036 | [Employee Directory](ch08_collections/036_employee_directory.rs) | ★★ | Rust Book ch. 8 suggestion |
| 037 | [Word Frequencies and Length Histogram](ch08_collections/037_word_frequencies_and_length_histogram.rs) | ★★ | McIlroy's word count; K&R Ex. 1-13 |
| 038 | [Two Sum and Group Anagrams](ch08_collections/038_two_sum_and_group_anagrams.rs) | ★★ | LeetCode 1 and 49 |
| 039 | [What Is a Character?](ch08_collections/039_what_is_a_character.rs) | ★★ | Original |
| 040 | [Text Justification](ch08_collections/040_text_justification.rs) | ★★★ | LeetCode 68; K&R Ex. 1-22 |

### Ch. 9 - Error Handling

| # | Problem | Difficulty | Source |
|---|---|---|---|
| 041 | [Config File Loader](ch09_error_handling/041_config_file_loader.rs) | ★ | Original |
| 042 | [Validated Types](ch09_error_handling/042_validated_types.rs) | ★ | Rust Book ch. 9 (Guess) extension |
| 043 | [RPN Calculator](ch09_error_handling/043_rpn_calculator.rs) | ★★ | K&R §4.3 |
| 044 | [Collect All the Errors](ch09_error_handling/044_collect_all_the_errors.rs) | ★★ | Original |
| 045 | [Dates](ch09_error_handling/045_dates.rs) | ★★★ | Original |

### Ch. 10 - Generic Types, Traits, and Lifetimes

| # | Problem | Difficulty | Source |
|---|---|---|---|
| 046 | [Generic Extremes](ch10_generics_traits_lifetimes/046_generic_extremes.rs) | ★ | Rust Book ch. 10 extension |
| 047 | [A Shape Trait](ch10_generics_traits_lifetimes/047_a_shape_trait.rs) | ★★ | Original |
| 048 | [Display Everything](ch10_generics_traits_lifetimes/048_display_everything.rs) | ★ | Original |
| 049 | [Lifetime Workout](ch10_generics_traits_lifetimes/049_lifetime_workout.rs) | ★★ | Original |
| 050 | [Generic Stack and Queue](ch10_generics_traits_lifetimes/050_generic_stack_and_queue.rs) | ★★ | LeetCode 20 and 232 |
| 051 | [A Borrowing Tokenizer](ch10_generics_traits_lifetimes/051_a_borrowing_tokenizer.rs) | ★★★ | Original |
| 052 | [SortedVec](ch10_generics_traits_lifetimes/052_sortedvec.rs) | ★★★ | Original |

### Ch. 11 - Writing Automated Tests

| # | Problem | Difficulty | Source |
|---|---|---|---|
| 053 | [Roman Numerals, Test-First](ch11_testing/053_roman_numerals_test_first.rs) | ★ | LeetCode 12 and 13 |
| 054 | [Retrofit a Test Suite](ch11_testing/054_retrofit_a_test_suite.rs) | ★★ | Original |
| 055 | [Bug Hunt](ch11_testing/055_bug_hunt.rs) | ★★ | Original |
| 056 | [Homemade Property Testing](ch11_testing/056_homemade_property_testing.rs) | ★★★ | Original (cf. QuickCheck) |

### Ch. 12 - An I/O Project (minigrep)

| # | Problem | Difficulty | Source |
|---|---|---|---|
| 057 | [minigrep, Extended](ch12_io_project/057_minigrep_extended.rs) | ★★ | Rust Book ch. 12 extension |
| 058 | [wc](ch12_io_project/058_wc.rs) | ★★ | K&R §1.5.4 |
| 059 | [csvq](ch12_io_project/059_csvq.rs) | ★★★ | Original |

### Ch. 13 - Iterators and Closures

| # | Problem | Difficulty | Source |
|---|---|---|---|
| 060 | [Loop-Free Rewrites](ch13_iterators_and_closures/060_loop_free_rewrites.rs) | ★ | Original; SICP Ex. 1.12 |
| 061 | [Closure Zoo](ch13_iterators_and_closures/061_closure_zoo.rs) | ★★ | Original |
| 062 | [Write Your Own Iterators](ch13_iterators_and_closures/062_write_your_own_iterators.rs) | ★★ | Project Euler 2, 7, 14 |
| 063 | [Iterator Adapters of Your Own](ch13_iterators_and_closures/063_iterator_adapters_of_your_own.rs) | ★★★ | Original |
| 064 | [Memoiser](ch13_iterators_and_closures/064_memoiser.rs) | ★★ | Rust Book (earlier editions' Cacher) |
| 065 | [Look-and-Say](ch13_iterators_and_closures/065_look_and_say.rs) | ★★★ | Conway's look-and-say sequence |

### Ch. 14 - More about Cargo and Crates.io

| # | Problem | Difficulty | Source |
|---|---|---|---|
| 066 | [Document It Like a Crate](ch14_cargo_and_crates/066_document_it_like_a_crate.rs) | ★ | Rust Book ch. 14 |
| 067 | [Workspace and Profiles](ch14_cargo_and_crates/067_workspace_and_profiles.rs) | ★★ | Rust Book ch. 14 |

### Ch. 15 - Smart Pointers

| # | Problem | Difficulty | Source |
|---|---|---|---|
| 068 | [Expression Trees](ch15_smart_pointers/068_expression_trees.rs) | ★★ | SICP §2.3 |
| 069 | [Symbolic Differentiation](ch15_smart_pointers/069_symbolic_differentiation.rs) | ★★★ | SICP §2.3.2 |
| 070 | [A Linked List with Box](ch15_smart_pointers/070_a_linked_list_with_box.rs) | ★★★ | Cf. "Learning Rust With Entirely Too Many Linked Lists" |
| 071 | [Smart Pointer Mechanics](ch15_smart_pointers/071_smart_pointer_mechanics.rs) | ★ | Rust Book ch. 15 |
| 072 | [Friendships with Rc and Weak](ch15_smart_pointers/072_friendships_with_rc_and_weak.rs) | ★★ | Rust Book ch. 15 |
| 073 | [A File System Tree](ch15_smart_pointers/073_a_file_system_tree.rs) | ★★★★ | Inspired by Advent of Code 2022, day 7 |

### Ch. 16 - Fearless Concurrency

| # | Problem | Difficulty | Source |
|---|---|---|---|
| 074 | [Parallel Sum](ch16_concurrency/074_parallel_sum.rs) | ★ | Original |
| 075 | [Channel Pipeline](ch16_concurrency/075_channel_pipeline.rs) | ★★ | Original |
| 076 | [Parallel Word Count](ch16_concurrency/076_parallel_word_count.rs) | ★★ | Original |
| 077 | [Dining Philosophers](ch16_concurrency/077_dining_philosophers.rs) | ★★★ | Dijkstra |
| 078 | [Bounded Blocking Queue](ch16_concurrency/078_bounded_blocking_queue.rs) | ★★★★ | Original |

### Ch. 17 - Async and Await

| # | Problem | Difficulty | Source |
|---|---|---|---|
| 079 | [Async Countdown](ch17_async/079_async_countdown.rs) | ★ | Rust Book ch. 17 |
| 080 | [Concurrent Page Titles](ch17_async/080_concurrent_page_titles.rs) | ★★ | Rust Book ch. 17 extension |
| 081 | [Timeouts and Retries](ch17_async/081_timeouts_and_retries.rs) | ★★★ | Rust Book ch. 17 extension |
| 082 | [Async Chat Room](ch17_async/082_async_chat_room.rs) | ★★★★ | Original |

### Ch. 18 - Object-Oriented Programming Features

| # | Problem | Difficulty | Source |
|---|---|---|---|
| 083 | [ASCII GUI with Trait Objects](ch18_oop/083_ascii_gui_with_trait_objects.rs) | ★ | Rust Book ch. 18 extension |
| 084 | [Blog Post, Two Ways](ch18_oop/084_blog_post_two_ways.rs) | ★★ | Rust Book ch. 18 suggestions |
| 085 | [Command Registry and dyn Compatibility](ch18_oop/085_command_registry_and_dyn_compatibility.rs) | ★★★ | Original |

### Ch. 19 - Patterns and Matching

| # | Problem | Difficulty | Source |
|---|---|---|---|
| 086 | [Pattern Zoo](ch19_patterns/086_pattern_zoo.rs) | ★ | Original |
| 087 | [Slice Patterns](ch19_patterns/087_slice_patterns.rs) | ★★ | Original |
| 088 | [A Tiny Stack VM](ch19_patterns/088_a_tiny_stack_vm.rs) | ★★★ | Original |
| 089 | [Refutability](ch19_patterns/089_refutability.rs) | ★ | Original |

### Ch. 20 - Advanced Features

| # | Problem | Difficulty | Source |
|---|---|---|---|
| 090 | [Operator Overloading](ch20_advanced_features/090_operator_overloading.rs) | ★★ | Rust Book ch. 20 |
| 091 | [Advanced Traits Tour](ch20_advanced_features/091_advanced_traits_tour.rs) | ★★ | Rust Book ch. 20 |
| 092 | [macro_rules!](ch20_advanced_features/092_macro_rules.rs) | ★★★ | Rust Book ch. 20 |
| 093 | [Unsafe and FFI](ch20_advanced_features/093_unsafe_and_ffi.rs) | ★★★ | Rust Book ch. 20 |
| 094 | [Parser Combinators](ch20_advanced_features/094_parser_combinators.rs) | ★★★★ | Inspired by Bodil Stokke, "Learning Parser Combinators With Rust" |

### Ch. 21 - Final Project: Web Server

| # | Problem | Difficulty | Source |
|---|---|---|---|
| 095 | [Web Server Features](ch21_web_server/095_web_server_features.rs) | ★★★ | Rust Book ch. 21 extension |
| 096 | [Thread Pool Hardening](ch21_web_server/096_thread_pool_hardening.rs) | ★★★★ | Rust Book ch. 21 suggestions |

### Capstones (after finishing the Book)

| # | Problem | Difficulty | Source |
|---|---|---|---|
| 097 | [Huffman Compression](ch99_capstones/097_huffman_compression.rs) | ★★★★ | SICP §2.3.4 |
| 098 | [A Tiny Lisp](ch99_capstones/098_a_tiny_lisp.rs) | ★★★★ | SICP ch. 4 |
| 099 | [JSON Parser and Pretty-Printer](ch99_capstones/099_json_parser_and_pretty_printer.rs) | ★★★★ | RFC 8259 |
| 100 | [Mini Redis](ch99_capstones/100_mini_redis.rs) | ★★★★ | Original (cf. Redis) |

## Sources and credits

Problems marked with a source are adapted from or inspired by: The Rust Programming Language (end-of-chapter suggestions), Project Euler, LeetCode, K&R's *The C Programming Language*, SICP, Advent of Code (format only - the sample data here is original), and the other works named in each header. Unmarked problems are original.
