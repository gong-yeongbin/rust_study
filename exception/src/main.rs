use std::fs::File;
use std::error::Error as stdError;
use std::{fs, io};
use std::io::{Error, ErrorKind, Read};


// [9.3.4] 유효성을 위한 커스텀 타입 생성하기
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }
        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}


fn main() -> Result<(), Box<dyn stdError>> {
    // panic!("crash and burn");

    // [9.1.1] panic! 백트레이스 이용하기
    // let v = vec![1, 2, 3];
    // v[99];

    // [9.2] Result로 복구 가능한 에러 처리하기
    let greeting_file_result = File::open("hello.txt");

    // let greeting_file = match greeting_file_result {
    //     Ok(file) => file,
    //     Err(error) => panic!("Problem opening the file: {:?}", error)
    // };

    // [9.2.1] 서로 다른 에러에 대해 매칭하기
    // let greeting_file = match greeting_file_result {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => panic!("Problem creating the file: {:?}", e)
    //         },
    //         other_error => {
    //             panic!("Problem opening the file: {:?}", other_error)
    //         }
    //     }
    // };

    // Result<T,E>와 match 사용에 대한 대안
    // let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
    //     if error.kind() == ErrorKind::NotFound {
    //         File::create("hello.txt").unwrap_or_else(|error| {
    //             panic!("Problem creating the file: {:?}", error);
    //         })
    //     } else {
    //         panic!("Problem opening the file: {:?}", error);
    //     }
    // });

    // 에러 발생 시 패닉을 위한 숏컷: unwrap과 expect
    // let greeting_file = File::open("hello.txt").unwrap();
    // let greeting_file = File::open("hello.txt").expect("hello.txt should be included in this project");

    // ? 연산자가 사용될 수 있는 곳
    // let greeting_file = File::open("hello.txt")?;
    // Ok(())

    Ok(())
}

// [9.2.2] 에러 전파하기
/* 함수에서 에러를 처리하는 대신 이 함수를 호출하는 코드 쪽으로 에러를 반환하여 그쪽에서 수행할 작업을 결정하도록 하기 */
// fn read_username_from_file() -> Result<String, io::Error> {
//     let username_file_result = File::open("hello.txt");
//
//     let mut username_file = match username_file_result {
//         Ok(file) => file,
//         Err(e) => return Err(e),
//     };
//
//     let mut username = String::new();
//
//     match username_file.read_to_string(&mut username) {
//         Ok(_) => Ok(username),
//         Err(e) => Err(e)
//     }
// }

// 에러를 전파하기 위한 숏컷: ? 연산자
// fn read_username_from_file() -> Result<String, io::Error> {
//     let mut username_file = File::open("hello.txt")?;
//     let mut username = String::new();
//     username_file.read_to_string(&mut username)?;
//     Ok(username)
// }

// ? 연산자 뒤에 메서드 호출을 연결하기
// fn read_username_from_file() -> Result<String, io::Error> {
//     let mut username = String::new();
//     File::open("hello.txt")?.read_to_string(&mut username)?;
//
//     Ok(username)
// }

// 파일을 열고, 읽는 대신 fs::read_to_string을 사용하기
fn read_username_from_file() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}


// Option<T>값에 대한 ? 연산자의 사용
fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}





