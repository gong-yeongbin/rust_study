fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

    let mut s1 = String::from("hello");
    change(&mut s1);

    println!("The length of '{}' is {}.", s1, len);

    let mut s = String::from("hello");
    let r1 = &mut s;
    let r2 = &mut s;
    // s를 가변으로 두 번 이상 빌려올 수 없음 ? 쉐도잉과 비슷?
    // println!("{}, {}", r1, r2);

    // 중괄호로 새로운 스코프를 만들어 가변 참조자를 여러 개 만들면서 동시에 존재하는 상황을 회피 하는 방법
    {
        let r1 = &mut s;
        println!("{}", r1);
    }
    let r2 = &mut s;
    println!("{}", r2);

    // 가변 참조자와 불변 참조자를 혼용, 유사한 규칙이 적용.
    let mut s = String::from("hello");
    let r1 = &s;
    let r2 = &s;
    println!("{}, {}", r1, r2);
    let r3 = &mut s;
    // 어떤 값에 대한 불변 참조자가 있는 동안 같은 값의 가변 참조자를 만드는 것 또한 불가능.
    // println!("{}", r1, r2, r3);

    // 참조자는 정의된 지점부터 시작하여 해당 참조자가 마지막으로 사용된 부분까지 유효
    println!("{}", r3);

    let reference_to_nothing = dangle();
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

// fn dangle() -> &String { // 스코프가 끝나면 메모리 해제 &참조자 유효하지 않음
//     let &s = String::from("hello");
//     &s
// }
fn dangle() -> String {
    let s = String::from("hello");
    s
}
