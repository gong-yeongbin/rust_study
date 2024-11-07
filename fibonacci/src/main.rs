use std::io;

fn main() {
    // 피보나치 수
    // 1 1 2 3 5 8 13 21 34 55.....
    println!("몇 번째 피보나치 수?");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("throw input exception.");

    let n: u32 = input.trim().parse().expect("throw type conversion exception.");

    let mut result = [1; 2];

    for i in 1..n + 1 {
        if i >= 3 {
            let new: i32 = result[0] + result[1];
            result[0] = result[1];
            result[1] = new;
        }
    }

    println!("{}번째 피보나치 수: {}", n, result[1]);
}