fn main() {
    {
        let s = "hello";
        println!("{s}")
    }

    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{}", s);

    {
        let s = String::from("hello"); // 힙 할당
    } // 해제

    let x = 5;
    let y = x;

    let s1 = String::from("hello");
    // let s2 = s1;
    let s2 = s1.clone();

    println!("{}, world!", s1);
    println!("{}, world!", s2);

    let s = String::from("hello");
    takes_ownership(s);

    let x = 5;
    makes_copy(x);

    let s1 = gives_ownership();
    let s2 = String::from("hello");
    println!("{s2}");
    let s3 = takes_and_gives_back(s2);
    println!("{s3}");

    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}

fn test(s: String) -> usize {
    s.len()
}
