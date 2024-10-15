const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3; // 상수

fn main() {
    let mut x = 5; // 가변성
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    let x = 5;
    let x = x + 1; // 쉐도잉
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }
    println!("The value of x is: {x}");

    let spaces = "    ";
    let spaces = spaces.len();

    // error
    // let mut spaces = "    ";
    // spaces = spaces.len();
}
