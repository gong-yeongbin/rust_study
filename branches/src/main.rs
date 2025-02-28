fn main() {
    /* if 표현식 */
    let number = 7;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // if number {
    //     println!("number was three");
    // }
    if number != 0 {
        println!("number was something other than zero");
    }

    // else if로 여러 조건식 다루기
    println!("----- else if로 여러 조건식 다루기 -----");
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // let 구문에서 if 사용하기
    println!("----- let 구문에서 if 사용하기 -----");
    let condition = true;
    let number = if condition { 5 } else { 6 };
    // let number = if condition { 5 } else { "six" };

    println!("The value of number is: {number}");
}
