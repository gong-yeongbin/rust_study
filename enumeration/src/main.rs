enum IpAddrKind {
    V4,
    V6,
}

// enum IpAddr {
//     V4(String),
//     V6(String),
// }

enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {}
}


/*
다른 종류의 구조체들을 정의하는 것과 비슷
struct QuitMessage; 유닛 구조체
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); 튜플 구조체
struct ChangeColorMessage(i32, i32, i32); // 튜플 구조체
*/

// enum Coin {
//     Penny,
//     Nickel,
//     Dime,
//     Quarter,
// }

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
    // [6.1] 열거형 정의하기
    // [6.1.1] 열거형 값
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    route(IpAddrKind::V4);
    route(IpAddrKind::V6);

    // let home = IpAddr {
    //     kind: IpAddrKind::V4,
    //     address: String::from("127.0.0.1"),
    // };
    //
    // let loopback = IpAddr {
    //     kind: IpAddrKind::V6,
    //     address: String::from("::1"),
    // };

    // let home = IpAddr::V4(String::from("127.0.0.1"));
    // let loopback = IpAddr::V6(String::from("::1"));

    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    let m = Message::Write(String::from("hello"));
    m.call();

    // [6.1.2] Option 열거형이 널값보다 좋은 점들
    let some_number = Some(5);
    let some_char = Some('e');

    let absent_number: Option<i32> = None;


    // let x: i8 = 5;
    // let y: Option<i8> = Some(5);
    /* x, y는 서로 다른 타입 */
    // let sum = x + y;

    // [6.2] match 제어 흐름 구조

    // [6.2.1] 값을 바인딩 하는 패턴
    value_in_cents(Coin::Quarter(UsState::Alaska));

    // [6.2.2] Option<T>를 이용하는 매칭
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    // [6.2.4] 포괄패턴과 _ 자리표시자
    let dice_roll = 9;
    // match dice_roll {
    //     3 => add_fancy_hat(),
    //     7 => remove_fancy_hat(),
    //     other => move_player(other)
    // }

    /* 포괄 패턴이 필요한데 그 포괄 패턴의 값을 사용할 필요는 없는 경우 */
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => reroll()
    }
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => ()
    }

    // [6.3] if let을 사용한 간결한 제어 흐름
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to bo {}", max),
        _ => (),
    }
    if let Some(max) = config_max { println!("The maximum is configured to bo {}", max); }

    // let mut count = 0;
    // match coin {
    //     Coin::Quarter(state) => println!("State quarter from {:?}!", state),
    //     _ => count += 1,
    // }
    //
    // if let Coin::Quarter(state) = coin { println!("State quarter from {:?}!", state) } else { count += 1 }
}


fn route(ip_kind: IpAddrKind) {}

// fn value_in_cents(coin: Coin) -> u8 {
//     match coin {
//         Coin::Penny => {
//             println!("Lucky penny!");
//             1
//         }
//         Coin::Nickel => 5,
//         Coin::Dime => 10,
//         Coin::Quarter => 25
//     }
// }

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn add_fancy_hat() {}

fn remove_fancy_hat() {}

fn move_player(num_spaces: u8) {}

fn reroll() {}