fn main() {
    println!("Hello, world!");

    print_labeled_measurement(5, 'h');

    /*
    구문과 표현식
    구문. 어떤 동작을 수행하고 값을 반환하지 않는 명령
    표현식. 결괏값을 평가합니다.
    */
    let y = 6;

    // 반환 값을 갖는 함수
    let x = five();
    println!("The value of x is: {x}");

    let x = plus_one(5);

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
