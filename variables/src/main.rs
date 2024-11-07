use std::io;

fn main() {
    // [3.1] 변수와 가변성
    let mut x = 5;
    println!("The value of x is: {x}");

    x = 6;
    println!("The value of x is: {x}");

    // [3.1.1] 상수
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    // [3.1.2] 섀도잉
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }
    println!("The value of x is: {x}");

    let spaces = "     ";
    let spases = spaces.len(); // 다른 타입의 값 저장

    // let mut spaces = "     ";
    // spases = spaces.len(); // 재할당 다른 타입 컴파일 에러

    // [3.2] 데이터 타입
    let guess: u32 = "42".parse().expect("Not a number!"); // 타입 변환: 타입 명시

    // [3.2.1] 스칼라 타입
    // 부동소수점 타입
    // f32 1배수 정밀도, f64 2배수 정밀도
    let x = 2.0; // 기본 f64 현대의 cpu상에서 f64가 f32와 대략 비슷한 속도, 더 정밀
    let y: f32 = 3.0; // f32

    // 수치 연산
    let sum = 5 + 10; // 덧셈
    let difference = 95.5 - 4.3; // 뺄셈
    let quotient = 56.7 / 32.2; // 곱셈
    let truncated = -5 / 3; // 나눗셈
    let remainder = 43 % 5; // 나머지 연산

    // 불리언 타입
    let t = true;
    let f: bool = false;

    // 문자 타입
    // 문자열 리터럴 "", char 타입 ''
    // char 4byte
    let c = 'z';
    let z: char = 'Z';
    let heart_eyed_cat = '🐱';

    // [3.2.2] 복합 타입
    // 튜플 타입 (다양한 타입의 여러 값을 묶어 하나의 복합 타입으로 만드는 방법)
    // 고정 길이
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    // 구조 해체
    let (x, y, z) = tup;

    println!("The value of y is: {y}");

    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;
    let unit = (); // 유닛(아무 값도 없는 튜플)

    // 배열 타입
    // 벡터 타입처럼 유연하지 않음
    // 요소의 개수가 바뀔 피요가 없다면 배열이 더 유용
    let months = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];
    let a = [1, 2, 3, 4, 5];
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    // let a:[i32:5] = [3,3,3,3,3];
    let a = [3; 5];

    // 배열 요소에 접근하기
    let a = [1, 2, 3, 4, 5];
    let first = a[0];
    let second = a[1];

    // 유효하지 않은 배열 요소에 대한 접근
    // 런타임 에러
    // let a = [1, 2, 3, 4, 5];
    //
    // println!("Please enter an array index.");
    //
    // let mut index = String::new();
    //
    // io::stdin().read_line(&mut index).expect("Failed to read line");
    //
    // let index: usize = index.trim().parse().expect("Index entered w4as not a number");
    //
    // let element = a[index];
    // println!("The value of the element at index {index} is: {element}");
}