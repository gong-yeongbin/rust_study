#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
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