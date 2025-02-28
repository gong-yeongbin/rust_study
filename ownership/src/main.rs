fn main() {
    // 변수의 스코프
    println!("----- 변수의 스코프 -----");
    {
        // s는 아직 선언되지 않아서 유효하지 않음
        let s = "hello"; // 이 지점부터 s가 유효
        // s로 어떤 작업
    } // 스코프 종료, s가 더 이상 유효하지 않음

    // String 타입
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{}", s);

    {
        let s = String::from("hello"); // s는 이 지접부터 유효
        // s를 가지고 무언가 함
    } // 스코프 종료, s는 더 이상 유효하지 않음

    // 변수와 데이터 간 상호작용 방식: 이동
    println!("----- 변수와 데이터 간 상호작용 방식: 이동 -----");
    let x = 5;
    let y = x;
    println!("x = {}, y = {}", x, y);

    let s1 = String::from("hello");
    let s2 = s1; // 이동
    // println!("{}, world!", s1);

    // 변수와 데이터 간 상호작용 방식: 클론
    let s1 = String::from("hello");
    let s2 = s1.clone(); // 클론
    println!("s1 = {}, s2 = {}", s1, s2);

    // 소유권과 함수
    let s = String::from("hello");
    takes_ownership(s); // 함수로 이동
    // s 유효 하지 않음
    let x = 5;
    makes_copy(x); // 함수로 복사
    // x 유효

    let s1 = gives_ownership(); // gives_ownership의 반환값 s1으로 이동
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2); // s2 takes_and_gives_back 이동, 반환값 s3로 이동

    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);
    println!("The length of '{}' is {}.", s2, len);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}
