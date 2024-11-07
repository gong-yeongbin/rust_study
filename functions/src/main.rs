fn main() {
    println!("Hello, world!");

    // [3.3] 함수
    // 함수나 변수 이름의 관례 스네이크 케이스
    // another_function();

    // [3.3.1] 매개변수
    another_function(5);
    print_labeled_measurement(5, 'h');

    // [3.3.2] 구문과 표현식
    // 구문!은 어떤 동작을 수행하고 값을 반환하지 않는 명령입니다.
    // 표현식!은 결괏값을 평가합니다.

    let y = 6; // 구문
    // let x = (let y = 6); // 구문은 값을 반환하지 않음. let 구문을 다른 변수에 할당하려고 하면 에러.

    // c나 다른 언어와의 차이점. 할당문이 할당된 값을 반환 ex) x = y = 6
    // 5 + 6 = 11 11이라는 값을 평하하는 표현식

    // 표현식은 구문의 일부일 수 있음
    let y = 6; // 6은 6이라는 값을 평가 하는 표현식

    // 함수 호출, 매크로 호출도 표현식
    //1 let 구문의 일부로서 y에 바인딩
    let y = { //2 4를 평가하는 코드 블록
        let x = 3;
        //3 표현식은 종결을 나타내는 세미콜론을 쓰지 않음
        // 세미콜론을 사용하면 구문으로 변경
        x + 1
    };
    println!("The value of y is: {y}");

    // [3.3.3] 반환값을 갖는 함수
    let x = five();
    println!("The value of x is: {x}");

    let x = plus_one(5);
    println!("The value of x is: {x}");
}

// fn another_function() {
//     println!("Another function.");
// }

fn another_function(x: i32) { // 매개변수의 타입을 반드시 선언
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}