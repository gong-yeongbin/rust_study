use std::io;

fn main() {
    let x = 2.0;
    let y: f32 = 3.0;

    // 덧셈
    let sum = 5 + 10;
    // 뺄셈
    let difference = 95.5 - 4.3;
    // 곱셈
    let product = 4 * 30;
    // 나눗셈
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3;
    // 나머지 연산
    let remainder = 43 % 5;

    // 부울린
    let t = true;
    let f: bool = false;

    // 문자
    let c = 'z';
    let z: char = 'ℤ';
    let heart_eyed_cat = '😻';

    // 복합타입
    // 튜플
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {y}");

    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;

    // 배열
    let a = [1, 2, 3, 4, 5];
    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let a = [3; 5];

    let a = [1, 2, 3, 4, 5];
    let first = a[0];
    let second = a[1];

    // 컴파일은 되지만, 배열 길이보다 같거나 큰 값이 입력되는 경우 패닉 일으킨다.
    // 코드가 실행되고 어떤 값을 입력할지 컴파일러는 알수 없다.
    let a = [1, 2, 3, 4, 5];
    println!("Please enter an array index.");

    let mut index = String::new();
    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];
    println!("The value of the element at index {index} is: {element}");
}
