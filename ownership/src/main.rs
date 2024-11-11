fn main() {
    // 소유권. 가비지 컬렉터 없이 메모리 안정성을 보장

    // [4.1.2] 변수의 스코프
    { // s는 아직 선언되지 않음
        let s = "hello"; // 문자열 리터럴
    } // 스코프 종료, s가 더이상 유효하지 않음

    // [4.1.3] String 타입
    let mut s = String::from("hello");
    s.push_str(", world!"); // 문자열에 리터럴을 추가
    println!("{}", s);

    // [4.1.4] 메모리와 할당
    {
        let s = String::from("hello"); // 메모리 할당
    } // 메모리 해제

    // 변수와 데이터 간 상호작용 방식: 이동
    let x = 5; // 5를 x에 바인딩
    let y = x; // x의 복사본을 y에 바인딩

    let s1 = String::from("hello"); // 포인터, 길이, 용량 바인딩
    let s2 = s1; // s1의 포인터, 길이, 용량 바인딩
    // s1의 복사본을 생성 한다면 굉장히 느려질것
    // s1, s2 같은 포인터를 가리킨다면 스코프 밖으로 벗어날때 중복 해제 에러, 보안 취약
    // let s2 = s1 이후로 s1은 더 이상 유효하지 않다고 판단
    // s1이 s2로 이동(move) 되었다 표현

    // println!("{}, world!", s1); 에러!

    // 변수와 데이터 간 상호작용 방식: 클론
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);

    // 스택에만 저장되는 데이터: 복사
    // 정수형 등 컴파일 타임에 크기가 고정되는 타입은 스택에 저장
    let x = 5;
    let y = x;
    println!("x = {}, y = {}", x, y);

    // [4.1.5] 소유권과 함수
    let s = String::from("hello"); // s가 스코프 안으로 들어옴

    takes_ownership(s); // s의 값이 함수로 이동
    // s의 값이 더이상 유효하지 않음

    let x = 5; // x가 스코프 안으로 들어옴

    makes_copy(x); // x가 함수로 이동
    // i32는 copy이므로 x는 유효

    println!("{x}");

    // [4.1.6] 반환값과 스코프
    // 힙에 데이터를 갖는 변수가 스코프를 벗어나면 사전에 해당 데이터가 이동하여 소유권이 다른 변수에 이동 되지 않은 이상 drop
    let s1 = gives_ownership();

    let s2 = String::from("hello");

    let s3 = takes_and_gives_back(s2);
    // s1,s3 스코프 밖으로 벗어나면 버려짐, s2는 아무일도 일어나지 않음

    let s1 = String::from("hello");
    let (s2, len) = calculate_length1(s1);
    println!("The length of '{}' is {}.", s2, len);


    // [4.2] 참조와 대여
    let s1 = String::from("hello");
    let len = calculate_length2(&s1);
    println!("The length of '{}' is {}.", s1, len);

    // let s = String::from("hello");
    // change(&s);

    // [4.2.1] 가변 참조자
    let mut s = String::from("hello");
    change(&mut s);

    // 어떤 값에 대한 가변 참조자가 있다면, 그 값에 대한 참조자는 더 이상 만들 수 없음
    // 데이터 경합을 방지
    // - 둘 이상의 포인터가 동시에 같은 데이터에 접근
    // - 포인터 중 하나 이상이 데이터에 쓰기 작업을 수행
    // - 데이터 접근 동기화 메커니즘이 없음
    let mut s = String::from("hello");
    let r1 = &mut s;
    let r2 = &mut s; // 에러!

    // println!("{}, {}", r1, r2);

    let mut s = String::from("hello");
    {
        let r1 = &mut s;
    } // 스코프 밖으로 벗어나면, 새 참조자 만들수 있음
    let r2 = &mut s;

    // 가변 참조자와 불변 참조자를 혼용할 때도 유사한 규칙 적용
    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &s;
    let r3 = &mut s;

    // 불변 참조자가 마지막 사용되는 printls! 이후 가변 참조자 정의가 있음, 사용 가능
    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &s;
    println!("{} and {}", r1, r2);
    // 이 지점 이후로 변수 r1과 r2는 사용되지 않음
    let r3 = &mut s;
    println!("{}", r3);

    // [4.2.2] 댕글링 참조
    /* 댕글링 포인터랑, 어떤 메모리를 가리키는 포인터가 남아 있는 상황에서 일부 메모리를 해제해버림으로써,
    다른 개체가 할당받았을지도 모르는 메모리를 참조하게 된 포인터 */
    let reference_to_nothing = dangle();

    // [4.3] 슬라이스 타입
    /* 슬라이스(slice)는 컬렉션을 통째로 참조하는 것이 아닌, 컬렉션의 연속된 일련의 요소를 참조.
    슬라이스는 참조자의 일종으로서 소유권을 갖지 않음 */
    let mut s = String::from("hello");
    let word = first_word(&s);
    s.clear();
    // word를 의미있게 쓸 수 있는 문자열은 더 이상 없음

    // [4.3.1] 문자열 슬라이스
    /* String의 일부를 가리키는 참조자 */
    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];

    let slice = &s[0..2];
    let slice = &s[..2];

    let len = s.len();

    let slice = &s[3..len];
    let slice = &s[3..];

    let slice = &s[0..len];
    let slice = &s[..];

    let mut s = String::from("hello");
    let word = first_word(&s);
    // s.clear();
    /* 에러!
    특정 대상의 불변 참조자가 이미 존재할 경우, 가변 참조자를 만들수 없다는 규칙
    clear 함수는 String의 길이를 변경해야 함, 가변 참조자가 필요.
    */
    println!("the first word is: {}", word);

    let s = "Hello, world!";

    // [4.3.2] 그 외 슬라이스
    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];
    assert_eq!(slice, &[2, 3]);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
} // drop 호출 메모리 해제

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
} // 별다른 일이 발생 하지 않음

fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string
} // some_string 호출자 함수로 이동

fn takes_and_gives_back(a_string: String) -> String {
    a_string
} // a_string 호출자 함수 쪽으로 이동

fn calculate_length1(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}

fn calculate_length2(s: &String) -> usize {
    s.len()
} // 참조하는 것을 소유하고 있진 않으므로, 버려지지 않음

// fn change(some_string: &String) {
//     some_string.push_str(" ,world"); // 작동하지 않음
// }

fn change(some_string: &mut String) {
    some_string.push_str(" ,world"); // 작동하지 않음
}

// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s // String s의 참조자를 반환
// } // s는 스코프 밖으로 벗어나고 버려짐, 메모리 해제

fn dangle() -> String {
    let s = String::from("hello");
    s // 직접 반환
} // 소유권 이동, 할당 해제 되지 않음

// fn first_word(s: &String) -> usize {
//     let bytes = s.as_bytes();
//
//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return i;
//         }
//     }
//
//     s.len()
// }

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}