use std::fmt::Result;
use std::io::Result as IoResult;

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub use crate::front_of_house::hosting;
use crate::front_of_house::hosting::add_to_waitlist;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    // add_to_waitlist();
}

// mod customer {
//     pub fn eat_at_restaurant() {
//         // hosting::add_to_waitlist();
//         super::hosting::add_to_waitlist();
//     }
// }

fn function1() -> Result {}

fn function2() -> IoResult<()> {}
