use std::net::{Ipv4Addr, Ipv6Addr};

enum IpAddrKind {
    V4,
    V6,
}

// enum IpAddr {
//     V4(Ipv4Addr),
//     V6(Ipv6Addr),
// }

// struct IpAddr {
//     kind: IpAddrKind,
//     address: String,
// }

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {}
}
// struct QuitMessage;
// struct MoveMessage {
//     x: i32,
//     y: i32,
// }
// struct WriteMessage(String);
// struct ChangeColorMessage(i32, i32, i32);

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    route(IpAddrKind::V4);

    // let home = IpAddr {
    //     kind: IpAddrKind::V4,
    //     address: String::from("127.0.0.1"),
    // };
    // let loopback = IpAddr {
    //     kind: IpAddrKind::V6,
    //     address: String::from("::1"),
    // };

    // let home = IpAddr::V4(String::from("127.0.0.1"));
    // let home = IpAddr::V4(127, 0, 0, 1);
    // let loopback = IpAddr::V6(String::from("::1"));

    let m = Message::Write(String::from("hello"));
    m.call();

    // Option 열거형이 널 값보다 좋은 점들
    let some_number = Some(5);
    let some_char = Some('e');

    let absent_number: Option<i32> = None;

    let x: i8 = 5;
    let y: Option<i8> = Some(5);
    // let sum = x + y;
}

fn route(ip_kind: IpAddrKind) {}
