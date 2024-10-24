fn main() {
    let mut s = String::new();

    let data = "inital contents";
    let s = data.to_string();
    let s = "inital contents".to_string();
    let s = String::from("inital contents");

    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");

    let mut s = String::from("foo");
    s.push_str("bar");
    println!("{s}");

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {s2}");

    let mut s = String::from("lo");
    s.push('l');
    println!("{s}");

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;
    println!("{s3}");

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    // let s = s1 + "-" + &s2 + "-" + &s3;
    let s = format!("{s1}-{s2}-{s3}"); // 소유권 가져가지 않음
    println!("{s}");

    let s1 = String::from("hello");
    // let h = s1[0]; // error!

    let hello = String::from("Hola");
    let hello = String::from("Здравствуйте");

    let hello = "Здравствуйте";
    // let answer = &hello[0];
    let s = &hello[0..4];
    println!("{s}");

    for c in hello.chars() {
        println!("{c}");
    }

    for c in hello.bytes() {
        println!("{c}");
    }
}
