fn main() {
    another_function(5, 'h');

    // 구문: 어떤 동작을 수행하고 값을 반환하지 않는 명령
    // 표현식: 결괏값을 평가

    let y = 6; // 구문

    // 구문을 다른 변수에 할당 -> error
    // let x = (let y = 6);

    let y = {
        let x = 3; // 구문
        x + 1 // 표현식 x + 1; 구문
    };
    println!("The value of y is: {y}");

    let x = five();
    println!("The value of x is: {x}");

    let x = plus_one(5);
    println!("The value of x is: {x}");
}

// 함수의 정의 = 구문
fn another_function(x: i32, unit_label: char) {
    println!("The measurement is: {x}{unit_label}");
}

fn five() -> i32 {
    5 // 표현식 5; 구문
}

fn plus_one(x: i32) -> i32 {
    x + 1 // 표현식 x + 1; 구문
}
