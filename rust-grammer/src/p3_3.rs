pub fn run() {
    let a: u32 = 1; // 타입을 u32로 명시
    let b = 1; // 타입을 명시하진 않았지만, i32로 추론
    let c = "abc"; // &str 타입으로 추론
    // 타입 결정 불가로 에러 발생, 타입을 명시해야 함
    // let d = "2".parse().expect("not a number");
    let d_1: u32 = "2".parse().expect("not a number");
    let d_2 = "2".parse::<u32>().expect("not a number");

    println!("a: {}, b: {}, c: {}, d_1: {}, d_2: {}", a, b, c, d_1, d_2);

    // isize, usize 타입은 32bit OS에서는 32bit, 64bit OS에서는 64bit로 결정됨
    let e: isize = 1;
    let f: usize = 1;

    println!(
        "e size: {}, f size: {}",
        std::mem::size_of_val(&e),
        std::mem::size_of_val(&f)
    );

    // 배열이나 벡터의 크기가 usize 타입

    // 숫자 사이에 _ 사용 가능
    let a1 = 100_000;
    let a2 = 100000;
    println!("{} {}", a1, a2);

    // 16진수
    let b1 = 0xff;
    let b2 = 15 * 16 + 15;
    println!("{} {}", b1, b2);

    // 8진수
    let c1 = 0o77;
    let c2 = 7 * 8 + 7;
    println!("{} {}", c1, c2);

    // 2진수
    let d1 = 0b1111_0000;
    let d2 = 128 + 64 + 32 + 16;
    println!("{} {}", d1, d2);

    // 문자 아스키 값
    let e1 = b'A'; // e1: u8
    let e2 = 'A'; // e2: char
    println!("{} {}", e1, e2); // 65 A

    // 부동 소숫점
    let x = 2.0;
    let y: f32 = 3.0;
    println!("x: {}, y: {}", x, y); // x: 2, y: 3

    let u: u32 = 40_000;
    let sqrt_u = (u as f64).sqrt();
    println!("sqrt(u): {}", sqrt_u); // sqrt(u): 200

    // 부울
    let t1 = true;
    let t2: bool = false;

    if t1 {
        println!("t1 is true");
    }
    println!("t2 is {}", t2); // t2 is false

    // 문자
    let ac = 'a';
    let zc = 'z';
    println!("{} {}", ac, zc); // a z

    // 튜플
    let p: (&str, u32) = ("Lee", 20);
    println!("name: {}, age: {}", p.0, p.1); // name: Lee, age: 20
    println!("{:?}", p); // ("Lee", 20)

    let info = get_info();
    println!("age: {}, height: {}", info.0, info.1); // age: 20, height: 180

    // 배열
    let arr = [1, 2, 3, 4, 5];
    println!("{}", arr[0]); // 1
    println!("{:?}", arr); // [1, 2, 3, 4, 5]
    println!("{:?}", &arr[0..2]); // [1, 2]
    println!("{:?}", &arr[3..]); // [4, 5]

    let arr2 = [1; 5];
    println!("{:?}", arr2); // [1, 1, 1, 1, 1]

    let arr3: [i32; 5];
    arr3 = [1; 5];
    println!("{:?}", arr3); // [1, 1, 1, 1, 1]

    // 에러
    // let arr4 = [1, 2.0];

    // let arr5 = [1, 2, 3];
    // println!("{}", arr5[3]); // Out of bounds

    // 벡터
    let mut v = vec![1, 2, 3];
    v.insert(3, 4);
    println!("{:?}", v); // [1, 2, 3, 4]

    // 디폴트 값
    let num: i32 = Default::default(); // default is 0
    println!("num: {}", num); // num: 0

    /*
    // Rust 표준 라이브러리 코드
    impl Default for i32 {
        #[inline]
        fn default() -> i32 {
            0
        }
    }
    */

    #[derive(Default)]
    struct Person {
        name: String,    // ""
        age: u32,        // 0
        is_active: bool, // false
    }

    struct Point {
        x: f64,
        y: f64,
    }

    impl Default for Point {
        fn default() -> Self {
            Point { x: 0.0, y: 0.0 }
        }
    }

    let p: Person = Default::default();
    println!(
        "p: name: {}, age: {}, is_active: {}",
        p.name, p.age, p.is_active
    ); // p: name: , age: 0, is_active: false
    let p1: Point = Default::default();
    println!("p1: ({}, {})", p1.x, p1.y); // p1: (0, 0)
}

fn get_info() -> (i32, f64) {
    let age = 20;
    let height = 180.5;

    return (age, height);
}
