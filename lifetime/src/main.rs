use std::fmt::Display;

struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }

    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
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
    let i = ImportantExcerpt {
        part: first_sentence
    };

    // [10.4.10] 정적 라이프타임 'static 해당 참조자가 프로그램 전체 생애주기 동안 살아 있음
    let s: &'static str = "I have a static lifetime.";
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

// [10.5] 제네릭 타입 매개변수, 트레이트 바운드, 라이프타임을 한 곳에 사용해보기
fn logest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}