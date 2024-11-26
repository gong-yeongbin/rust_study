// fn largest<PartialOrd>(list: &[PartialOrd]) -> &PartialOrd {
//     let mut largest = &list[0];
//
//     for item in list {
//         if item > largest {
//             largest = item;
//         }
//     }
//
//     largest
// }

// fn largest_i32(list: &[i32]) -> &i32 {
//     let mut largest = &list[0];
//
//     for item in list {
//         if item > largest {
//             largest = item;
//         }
//     }
//
//     largest
// }
//
// fn largest_char(list: &[char]) -> &char {
//     let mut largest = &list[0];
//
//     for item in list {
//         if item > largest {
//             largest = item;
//         }
//     }
//
//     largest
// }

// struct Point<T> {
//     x: T,
//     y: T,
// }
//
// impl<T> Point<T> {
//     fn x(&self) -> &T {
//         &self.x
//     }
// }
//
// impl Point<f32> {
//     fn distance_from_origin(&self) -> f32 {
//         (self.x.powi(2) + self.y.powi(2)).sqrt()
//     }
// }

// struct Point<T, U> {
//     x: T,
//     y: U,
// }

struct Point<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> Point<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}


fn main() {
    // [10.1] 함수로 추출하여 중복 없애기
    // let number_list = vec![34, 50, 25, 100, 65];
    // let result = largest(&number_list);
    //
    //
    // println!("The largest number is {}", result);
    //
    // let char_list = vec!['y', 'm', 'a', 'q'];
    // let result = largest(&char_list);
    //
    // println!("The largest char is {}", result);


    // [10.2.2] 제네릭 구조체 정의
    // let both_integer = Point { x: 5, y: 10 };
    // let both_float = Point { x: 1.0, y: 4.0 };
    // let integer_and_float = Point { x: 5, y: 4.0 };

    // [10.2.3] 제네릭 열거형 정의 Option<T>, Result<T, E>
    // [10.2.4] 제네릭 메서드 정의
    // let p = Point { x: 5, y: 10 };
    // println!("p.x = {}", p.x());

    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

    // [10.2.5] 제네릭 코드의 성능
    let integer = Some(5);
    let float = Some(5.0);

    /* 단정형화::러스트 제네릭을 런타임에 극도로 효율적으로 만들어줌 */
    enum Option_i32 {
        Some(i32),
        None,
    }

    enum Option_f64 {
        Some(f64),
        None,
    }

    let integer = Option_i32::Some(5);
    let float = Option_f64::Some(5.0);
}

