use std::collections::HashMap;
use std::io::{self, Read, Write};
/* 해시 맵과 벡터를 이용하여 사용자가 회사 부서의 직원 이름을 추가할 수 있도록 하는 텍스트
       인터페이스를 만들어보세요. 예를 들어 'Add Sally to Engineering'이나 'Add amir to Sales'
       같은 방식으로요. 그 후 사용자가 모든 사람에 대해 알파벳 순으로 정렬된 목록이나 부서별
       모든 사람에 대한 목록을 조회할 수 있도록 해보세요. */


#[derive(Debug)]
struct User {
    department: String,
    employee: String,
}

impl User {
    pub fn new(department: String, employee: String) -> User {
        Self {
            department,
            employee,
        }
    }
}

fn main() {
    let mut company: HashMap<String, Vec<String>> = HashMap::new();

    loop {
        println!("입력 : ");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let input = input.trim();

        if input.starts_with("exit") {
            return break;
        } else if input.starts_with("all") {
            println!("{:?}", company);
        } else if input.starts_with("add") {
            let collect: Vec<&str> = input.split_whitespace().collect();
            let user = User::new(collect[3].to_string(), collect[1].to_string());

            if let Some(&dep) = company.get(collect[1]) {
                *dep.push(collect[1].to_string());
            }
        }
    }
}
























