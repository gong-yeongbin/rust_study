fn main() {
    // [3.5] 제어 흐름

    // [3.5.1] if 표현식
    let number = 7;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // 코드의 조건식이 반드시 bool 이어야 함
    // if number {
    //     println!("number was three");
    // }
    if number != 0 {
        println!("number was something other than zero");
    }

    // else if로 여러 조건식 다루기
    let number = 4;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!(" number is not divisible by 4, 3, or 2");
    }

    // let 구문에서 if 사용하기
    let condition = true;
    let number = if condition { 5 } else { 6 }; // 각 갈래의 결괏값은 같은 타입이어야 함

    println!("The value of number is: {number}");
}