struct InportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    // [10.4] 라이프타임으로 댕글링 참조 방지하기
    // let r;
    // {
    //     let x = 5;
    //     r = &x;
    //     println!("r: {}", r);
    // }
    // println!("r: {}", r);

    let x = 5;
    let r = &x;
    println!("r: {}", r);

    // [10.4.3] 함수에서의 제네릭 라이프타임
    let string1 = String::from("abcd");
    let string2 = "xyz";
    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }

    // [10.4.7] 구조체 정의에서 라이프타임 명시하기
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = InportantExcerpt {
        part: first_sentence
    };
}

// fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }

fn longest<'a>(x: &'a str, y: &str) -> &'a str {
    x
}