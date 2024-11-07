use std::io;

fn main() {
    // 화씨 온도와 섭씨 온도 간 변환하기
    // 화씨 -> 섭씨 (F - 32) / 1.8
    // 섭씨 -> 화씨  (C * 1.8) + 32

    println!("현재 온도(ex: 18C, 18F) : ");
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("unit exception");
    input = input.trim().parse().expect("");

    let temperature: f64 = input[0..input.len() - 1].parse().unwrap();
    let unit = input.chars().last().unwrap().to_string();


    let result = if unit == "C" { to_fahrenheit(temperature) } else { to_celsius(temperature) };
    println!("{result}");
}

fn to_fahrenheit(c: f64) -> f64 {
    (c * 1.8) + 32.0
}

fn to_celsius(f: f64) -> f64 {
    (f - 32.0) / 1.8
}