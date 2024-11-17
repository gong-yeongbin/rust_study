// mod front_of_house {
//     pub mod hosting {
//         pub fn add_to_waitlist() {}
//
//         fn seat_at_table() {}
//     }
//
//     mod serving {
//         fn take_order() {}
//
//         fn serve_order() {}
//
//         fn take_payment() {}
//     }
// }
//
// fn deliver_order() {}
//
// mod back_of_house {
//     fn fix_incorrect_order() {
//         cook_order();
//         super::deliver_order();
//     }
//
//     fn cook_order() {}
//
//     pub struct Breakfast {
//         pub toast: String,
//         seasonal_fruit: String,
//     }
//
//     impl Breakfast {
//         pub fn summer(toast: &str) -> Breakfast {
//             crate::back_of_house::Breakfast {
//                 toast: String::from(toast),
//                 seasonal_fruit: String::from("peaches"),
//             }
//         }
//     }
//
//     pub enum Appetizer {
//         Soup,
//         Salad,
//     }
// }
//
// use crate::front_of_house::hosting::add_to_waitlist;

// pub fn eat_at_restaurant() {
//     // 절대 경로
//     // crate::front_of_house::hosting::add_to_waitlist();
//     // 상대 경로
//     // front_of_house::hosting::add_to_waitlist();
//
//     // 호밀(Rye) 토스트를 곁들인 여름철 조식 주문하기
//     let mut meal = back_of_house::Breakfast::summer("Rye");
//     // 먹고 싶은 빵 바꾸기
//     meal.toast = String::from("Wheat");
//     println!("I'd like {} toast please", meal.toast);
//
//
//     // 컴파일 에러
//     // meal.seasonal_fruit = String::from("blueberrires")
//
//     let order1 = back_of_house::Appetizer::Soup;
//     let order2 = back_of_house::Appetizer::Salad;
// }

// pub fn eat_at_restaurant() {
//     add_to_waitlist();
// }

use std::fmt::Result;
use std::io::Result as IoResult;

// fn function1() -> Result {}
//
// fn function2() -> IoResult<()> {}

// pub use crate::front_of_house::hosting;

mod front_of_house;

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}