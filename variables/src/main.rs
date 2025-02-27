use std::io;

fn main() {
    println!("변수");
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    /* 상수 */
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    /* 섀도잉 */
    println!("섀도잉");
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    let spaces = "    ";
    let spaces = spaces.len();

    // let mut spaces = "    ";
    // spaces = spaces.len();

    /* 데이터 타입 */
    println!("데이터 타입");
    let guess: u32 = "42".parse().expect("Not a number");

    // 부동 소수점 타입
    let x = 2.0;
    let y: f32 = 3.0;

    // 수치 연산
    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let product = 4 * 30;
    let quotient = 56.7 / 32.2;
    let remainder = 43 % 5;

    // 부울린 타입
    let t = true;
    let f: bool = false;

    // 문자 타입
    let c = 'z';
    let z: char = 'ℤ';
    let heart_eyed_cat = '😻';

    // 복합 타입
    println!("튜플 타입");
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {y}");

    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;

    // 배열 타입
    let a = [1, 2, 3, 4, 5];
    let months = [
        "January",
        "February",
        "March",
        "Aprill",
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

    println!("The value of the element at index {index} is:{element}");
}
