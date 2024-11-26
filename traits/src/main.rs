use std::fmt::{Debug, Display};
use traits::{NewsArticle, Summary, Tweet};

// pub fn notify(item: &impl Summary) {
//     println!("Breaking news! {}", item.summarize());
// }

pub fn notify<T: Summary + Display>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// where 절로 트레이트 바운드 정리하기
// fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {}
fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{ 1 }

// [10.3.5] 트레이트를 구현하는 타입을 반환하기
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}

// [10.3.6] 트레이트 바운드를 사용해 조건부로 메서드 구현하기
struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

fn main() {
    /* [10.3] 트레이트로 공통된 동작 정의하기
       특정한 타입이 가지고 있으면서 다른 타입과 공유할 수 있는 기능을 정의 */

    // [10.3.1] 트레이트 정의하기
    // [10.3.2] 특정 타입에 트레이트 구현하기
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    // [10.3.3] 기본 구현
    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from("The Pittsburgh once again are the best hockey team in the NHL."),
    };

    println!("New article available! {}", article.summarize());

    // [10.3.4] 매개변수로서의 트레이트
    // notify(&article);
}
