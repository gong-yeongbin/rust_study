fn main() {
    // 슬라이스
    // let mut s = String::from("hello world");
    // let word = first_word(&s);
    // s.clear();

    let mut s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];

    let mut s = String::from("hello");
    let slice = &s[0..2];
    let slice = &s[..2];

    let mut s = String::from("hello");
    let len = s.len();
    let slice = &s[3..len];
    let slice = &s[3..];

    let mut s = String::from("hello");
    let len = s.len();
    let slice = &s[0..len];
    let slice = &s[..];

    let mut s = String::from("hello world");
    let first = first_world(&s);
    let second = second_world(&s);
    println!("{}", first);
    println!("{}", second);

    let my_string = String::from("hello, world!");
    let world = first_world(&my_string[0..6]);
    let world = first_world(&my_string[..]);
    let world = first_world(&my_string);

    let my_string_literal = "hello world";
    let world = first_world(&my_string_literal[0..6]);
    let world = first_world(&my_string_literal[..]);
    let world = first_world(my_string_literal);

    // 그 외 슬라이스
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    let eq = assert_eq!(slice, &[2, 3]);
}

// fn first_word(s: &String) -> usize {
//     let bytes = s.as_bytes();
//
//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return i;
//         }
//     }
//
//     return bytes.len();
// }

fn first_world(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    return &s[..];
}

fn second_world(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[i + 1..];
        }
    }

    return &s[..];
}
