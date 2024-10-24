fn main() {
    let mut v: Vec<i32> = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    let v = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    let v = vec![1, 2, 3, 4, 5];
    // let does_not_exist = &v[100]; // 패닉!
    let does_not_exist = v.get(100);

    let mut v = vec![1, 2, 3, 4, 5];
    let first = &v[0]; // 소유권 first

    v.push(6); // 기존 힙메모리 해지, 새로운 메모리 할당(넉넉한 메모리)
    // println!("The first element is: {first}"); // first 기존 메모리 주소

    let mut v = vec![100, 32, 57];
    for i in &v {
        println!("{i}");
    }

    for i in &mut v {
        *i += 50;
    }

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(10.12),
        SpreadsheetCell::Text(String::from("blue")),
    ];

    {
        let v = vec![1, 2, 3, 4];
    } // 스코프 밖으로 벗어나면 벡터 해제
}
