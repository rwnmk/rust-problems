// ===========================================================================
// 033 · Privacy Puzzle
// Chapter 7: Packages, Crates, and Modules
// Difficulty: ★★☆☆  core    Source: Original
// FENCE: Use only what chapters 1-7 cover.
//        Not yet: collections, `?`, generics, traits.
// ===========================================================================
//
// TASK
// In main, uncomment the numbered lines one at a time, CUMULATIVELY. Before
// each, write your prediction (compiles? if not, why?). If a line fails,
// record the reason and comment it back out before moving on.
//
// CHECK YOUR UNDERSTANDING
// 1. Line 12 works even though `payroll` is private. State the rule that
//    allows it.
// 2. Why can you read `o.dish` but not `o.table`? How could Order be made
//    impossible to build without `new`?
// 3. What does line 13 tell you about what `pub use` is for?
//
// ===========================================================================
// MY ANSWERS (write them before asking Claude to review)
// 1.
// 2.
// 3.
// ===========================================================================

#[allow(dead_code)]
mod kitchen {
    pub struct Order {
        pub dish: String,
        table: u8,
    }

    impl Order {
        pub fn new(dish: &str, table: u8) -> Order {
            Order { dish: String::from(dish), table }
        }

        fn secret_recipe(&self) -> String {
            String::from("salt")
        }

        pub fn describe(&self) -> String {
            format!("{} for table {} ({})", self.dish, self.table, self.secret_recipe())
        }
    }

    pub enum Course {
        Starter,
        Main,
        Dessert,
    }

    pub mod staff {
        pub fn chef() -> String {
            super::head_chef()
        }

        pub(crate) fn rota() -> u8 {
            3
        }

        fn payroll() -> u32 {
            1000
        }

        pub mod porter {
            pub fn wash() {
                println!("payroll is {}", super::payroll());
            }
        }
    }

    fn head_chef() -> String {
        String::from("Delia")
    }

    pub use self::staff::chef as who_cooks;
}

fn main() {
    // 1.  let o = kitchen::Order::new("soup", 4);
    // 2.  let o2 = kitchen::Order { dish: String::from("pie"), table: 2 };
    // 3.  println!("{}", o.dish);
    // 4.  println!("{}", o.table);
    // 5.  println!("{}", o.describe());
    // 6.  o.secret_recipe();
    // 7.  let c = kitchen::Course::Dessert;
    // 8.  println!("{}", kitchen::staff::chef());
    // 9.  println!("{}", kitchen::head_chef());
    // 10. println!("{}", kitchen::staff::rota());
    // 11. println!("{}", kitchen::staff::payroll());
    // 12. kitchen::staff::porter::wash();
    // 13. println!("{}", kitchen::who_cooks());
}
