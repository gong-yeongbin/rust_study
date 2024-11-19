use std::collections::HashMap;
use std::io;


const TOTAL_INPUT_COUNT: usize = 10;

fn main() {
    /* 정수 리스트가 주어졌을 때, 벡터를 이용하여 이 리스트의 중간값(median, 정렬했을 때 가운데 위치한 값),
       그리고 최빈값(mode, 가장 많이 발생한 값. 해시 맵이 여기서 도움이 될 것입니다)을 반환해보세요.*/
    let mut number_list: Vec<i64> = Vec::new();

    println!("숫자 {}개를 입력하세요.", TOTAL_INPUT_COUNT);

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        number_list.push(input.trim().parse().unwrap());

        if number_list.len() == TOTAL_INPUT_COUNT {
            break;
        }
    }

    print_input(&number_list);
    let median = median(&number_list);
    println!("중간값 : {median}");
    let mode = mode(&number_list);
    println!("최빈값 : {}, {}번", mode.0, mode.1);
}

fn print_input(vec_list: &Vec<i64>) {
    print!("입력 값: ");
    for vec in vec_list {
        print!("{} ", vec);
    }
    println!();
}

// 중간 값 -> 길이가 짝수: len / 2 -1, 홀수: len / 2
fn median(number_list: &Vec<i64>) -> i64 {
    let mut list = number_list.clone();
    list.sort();

    if list.len() % 2 == 0 {
        list[list.len() / 2 - 1]
    } else {
        list[list.len() / 2]
    }
}

// 최빈 값
fn mode(number_list: &Vec<i64>) -> (i64, i64) {
    let list = number_list.clone();
    let mut map: HashMap<i64, i64> = HashMap::new();

    for number in list {
        map.entry(number).and_modify(|count| *count += 1).or_insert(1);
    }

    let mut max: (i64, i64) = (0, 0);

    for key in map.iter() {
        if &max.1 < key.1 {
            max.0 = *key.0;
            max.1 = *key.1;
        }
    }

    max
}
