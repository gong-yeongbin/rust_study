fn main() {
    /* [8.1]벡터에 여러 값의 목록 저장하기
       메모리에서 모든 값을 서로 이웃하도록 배치
       같은 타입의 값만 저장 */

    // [8.1.1] 새 벡터 만들기
    let v: Vec<i32> = Vec::new();

    let v = vec![1, 2, 3];

    // [8.1.2] 벡터 업데이트하기
    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    // [8.1.3] 벡터 요소 읽기
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element.")
    }

    let mut v = vec![1, 2, 3, 4, 5];

    let first = &v[0];
    println!("The third element is {first}"); // 같은 스코프에서는 가변 참조자와 불변 참조자를 가질 수 없다는 규칙! 가능!!
    // v.push(6);
    println!("The third element is {first}"); // 같은 스코프에서는 가변 참조자와 불변 참조자를 가질 수 없다는 규칙! 에러!!

    /* 첫 번째 요소의 참조자가 벡터 끝부분의 변경이랑 무슨 상관일까?
       -> 벡터의 동작 방법 : 벡터는 모든 요소가 서로 붙어서 메모리에 저장. 새로운 요소를 벡터 끝에 추가 할 경우,
                         현재 벡터 메모리 위치에 새로운 요소를 추가할 공간이 없다면, 다른 넉넉한 곳에 메모리를 새로 할당하고
                         기존 요솔ㄹ 새로 할당한 공간에 복사. */

    // [8.1.4] 벡터값에 대해 반복하기
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{i}");
    }

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }
    println!("{:?}", v);

    // [8.1.5] 열거형을 이용해 여러 타입 저장하기
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![SpreadsheetCell::Int(3), SpreadsheetCell::Text(String::from("blue")), SpreadsheetCell::Float(10.12)];

    // [8.1.6] 벡터가 버려지면 벡터의 요소도 버려집니다.
    {
        let v = vec![1, 2, 3, 4];
    } // v가 스코프 밖으러 벗어나고 해제 됨!

    // [8.2.2] 새로운 문자열 생성하기
    let mut s = String::new();

    let data = "inital contents";

    let s = data.to_string();
    let s = "inital contents".to_string();
    let s = String::from("inital contents");

    // [8.2.3] 문자열 업데이트하기
    // push_str과 push를 이용하여 문자열 추가하기
    let mut a = String::from("foo");
    a.push_str("bar");

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {s2}");

    let mut s = String::from("lo");
    s.push('l');

    // + 연산자나 format! 매크로를 이용한 접합
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // s1은 여기로 이동되어 더 이상 사용할 수 없음.!

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    // let s = s1 + "-" + &s2 + "-" + &s3;
    let s = format!("{s1}-{s2}-{s3}");

    // [8.2.4] 문자열 내부의 인덱싱
    let s1 = String::from("hello");
    // let h = s1[0]; 에러!

    /* String은 Vec<u8>을 감싼 것 */
    let hello = String::from("Hola"); // len : 4, 문자열 'Hola'를 저장하고 있는 Vec이 4바이트 길이, UTF-8으로 인코딩되면 각각의 글자들이 1바이트씩 차지한다는 것.
    let hello = String::from("Здравствуйте"); // len : 12 x, 각각의 유니코드 스칼라값이 2바이트씩 차지. len : 24

    // [8.2.5] 문자열 슬라이싱하기
    let hello = "Здравствуйте";
    let s = &hello[0..4]; // 글자들이 각각 2바이트를 차지한다고 언급했으므로 Зд, 0..1 패닉!

    // [8.2.6] 문자열에 대한 반복을 위한 메서드
    for c in "Зд".chars() {
        println!("{c}");
    }
    for b in "Зд".bytes() {
        println!("{b}");
    }
}
