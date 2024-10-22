// enum Coin {
//     Penny,
//     Nickel,
//     Dime,
//     Quarter,
// }

use crate::UsState::Alabama;

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    // match 제어 흐름 구조
    value_in_cents(Coin::Penny);

    // 값을 바인딩하는 패턴
    value_in_cents(Coin::Quarter(UsState::Alaska));

    // Option<T>를 이용하는 매칭
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    //포괄 패턴과 _ 자리표시자
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        // other => move_player(other),
        _ => (),
    }
}

fn add_fancy_hat() {}

fn remove_fancy_hat() {}

fn move_player(num_spaces: u8) {
    println!("{}", num_spaces)
}

fn reroll() {}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}", state);
            25
        }
    }
}

// fn value_in_cents(coin: Coin) -> u8 {
//     match coin {
//         Coin::Penny => {
//             println!("Lucky penny!");
//             1
//         }
//         Coin::Nickel => 5,
//         Coin::Dime => 10,
//         Coin::Quarter => 25,
//     }
// }
