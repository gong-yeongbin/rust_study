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

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
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
}


fn route(ip_kind: IpAddrKind) {}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25
    }
}