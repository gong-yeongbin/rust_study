#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    // [5.1] 구조체 정의 및 인스턴스화
    /* 일부 필드만 가변으로 만들 수 없음 */
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");

    // [5.1.2] 기존 인스턴스를 이용해 새 인스턴스를 만들 때 구조체 업데이트 문법 사용하기
    /* 이 구문은 데이터를 이동 시킴, user2를 생성 후 user1를 더 이상 사용할 수 없음
    user1의 username 필드의 String이 user2로 이동하기 때문. active, sign_in_count는 Copy 트레이트를 구현한 타입 */
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };

    // [5.1.3] 명명된 필드 없는 튜플 구조체를 사용하여 다른 타입 만들기
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    // [5.1.4] 필드가 없는 유사 유닛 구조체
    struct AlwaysEqual;

    let subject = AlwaysEqual;

    // [5.2] 구조체를 사용한 예제 프로그램
    // let width1 = 30;
    // let height1 = 50;
    //
    // println!("The area of the rectangle is {} square pixels.", area(width1, height1));

    // [5.2.1] 튜플로 리팩터링하기
    // let rect1 = (30, 50);
    // println!("The area of the rectangle is {} square pixels.", area(rect1));

    // [5.2.2] 구조체로 리팩터링하여 코드에 더 많은 의미를 담기
    let rect1 = Rectangle { width: 30, height: 50 };
    println!("The area of the rectangle is {} square pixels.", area(&rect1));

    // [5.2.3] 트레이트 파생으로 유용한 기능 추가하기
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!("rect1 is {:#?}", rect1);

    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect1);
}

// [5.1.1] 필드 초기화 축약법 사용하기
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

// fn area(width: u32, height: u32) -> u32 {
//     width * height
// }

// fn area(dimensions: (u32, u32)) -> u32 {
//     dimensions.0 * dimensions.1
// }

fn area(recttangle: &Rectangle) -> u32 {
    recttangle.width * recttangle.height
}