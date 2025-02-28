fn main() {
    /* 반복문을 이용한 반복 */

    // loop로 코드 반복하기
    println!("----- loop로 코드 반복하기 -----");
    // loop {
    //     println!("again!");
    // }

    // 반복문에서 값 반환하기
    println!("----- 반복문에서 값 반환하기 -----");
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");

    // 루프 라벨로 여러 반복문 사이에 모호함 없애기
    println!("----- 루프 라벨로 여러 반복문 사이에 모호함 없애기 -----");
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

    // while을 이용한 조건 반복문
    println!("----- while을 이용한 조건 반복문 -----");
    let mut number = 3;

    while number != 0 {
        println!("{number}");

        number -= 1;
    }

    println!("LIFTOFF!!!");

    // for를 이용한 컬렉션에 대한 반복문
    println!("----- for를 이용한 컬렉션에 대한 반복문 -----");
    let a = [10, 20, 30, 40, 50];
    // let mut index = 0;

    // while index < 5 {
    //     println!("the value is: {}", a[index]);
    //
    //     index += 1;
    // }

    for element in a {
        println!("the value is: {}", element);
    }
}
