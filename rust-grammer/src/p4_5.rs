use std::fs::File;

struct Point<T> {
    x: T,
    y: T,
}

// impl할 때, 제네릭이 아닌 특정 타입에 대해서만도 구현 가능
/*
impl Point<i32>
... 기존과 동일하게
*/
impl<T> Point<T> {
    fn add(&self, rhs: &Point<T>) -> Point<T>
    where
        T: std::ops::Add<Output = T> + Copy,
    {
        let x_val = self.x + rhs.x;
        let y_val = self.y + rhs.y;
        return Point { x: x_val, y: y_val };
    }
}

fn min<T: std::cmp::PartialOrd>(a: T, b: T) -> T {
    if a < b {
        return a;
    } else {
        return b;
    }
}

fn add_sub<T>(a: T, b: T) -> (T, T)
where
    // 구문이 길어지면 where을 쓰는 것이 가독성 좋음
    T: std::ops::Add<Output = T> + std::ops::Sub<Output = T> + Copy,
{
    return (a + b, a - b);
}

// struct Person<'a> {
//     name: &'a str,
//     num: i32,
// }

#[derive(Debug)]
struct Point2<'a> {
    _x: &'a i32,
    _y: i32,
}

#[derive(Debug)]
struct Borrowed<'a>(&'a i32); // 튜플 구조체에서도 해당 튜플 구성원에 참조자가 있으면 반드시 라이프타임 표기해야함 

/*
구조체를 구현할 때의 라이프타임 표기

라이프타임 표기가 사용된 구조체를 impl 할 때는 impl 키워드 뒤에도 해당 라이프타임을 표기해야 한다.
해당 구조체의 라이프타임 파라미터를 impl에서도 정의해야하기 때문이다.
*/
struct Person<'a> {
    name: &'a str,
    age: i32,
}

impl<'a> Person<'a> {
    fn get_age(&self) -> i32 {
        self.age
    }

    fn get_name(&self) -> &str {
        self.name
    }

    fn print_alias(&self, alias: &str) -> &str {
        println!("alias: {}", alias);
        self.name
    }

    fn longest_name<'b>(&'b self, s: &'b str) -> &'b str {
        if self.name.len() > s.len() {
            self.name
        } else {
            s
        }
    }
}
/*
1. 리턴 타입이 참조자 형태가 아니면 라이프타임 표기 필요 없다.
2. 리턴 타입이 self에 있는 구성원에 대한 참조자이면, 라이프타임 표기를 생략할 수 있다. 리턴값에 대한 라이프타임이 self의 라이프타임 매개변수와 같게 된다.
3. 리턴 타입이 참조자 형태이고 위의 2~3번 케이스가 아니면 라이프타임 표기를 해야한다.
*/

pub fn run() {
    let p1 = Point { x: 2, y: 3 };
    let p2 = Point { x: 2.5, y: 3.5 };

    println!("p1=({}, {})", p1.x, p1.y);
    println!("p2=({}, {})", p1.x, p2.y);

    let a: i32 = 2;
    let b: i32 = 3;
    println!("min={}", min(a, b));

    let a: f32 = 2.5;
    let b: f32 = 3.5;
    println!("min={}", min(a, b));

    let a = 3;
    let b = 4;
    let (c, d) = add_sub(a, b);
    println!("(add, sub)=({}, {})", c, d);

    let a = 4.5;
    let b = 3.1;
    let (c, d) = add_sub(a, b);
    println!("(add, sub)=({}, {})", c, d);

    let p1 = Point { x: 2, y: 3 };
    let p2 = Point { x: 2.5, y: 3.5 };
    let p3 = Point { x: 3, y: 4 };

    println!("p1=({}, {})", p1.x, p1.y);
    println!("p2=({}, {})", p2.x, p2.y);
    println!("p3=({}, {})", p3.x, p3.y);

    let p4 = p1.add(&p3);
    println!("p4=({}, {})", p4.x, p4.y);

    let f = File::open("README.md");

    // Result에는 <T, E>라는 두 개의 제네릭 타입 사용
    // T는 Ok<T>, E는 Err<E> -> <X, Y>처럼 다른 문자 써도 됨
    let _f = match f {
        Ok(file) => file,
        Err(error) => {
            panic!("There was a problem opening the file: {:?}", error)
        }
    };

    // 라이프타임 매개변수의 이름을 어퍼스트로피 (')로 시작한다
    // &'a i32
    // &'a mut i32
    // &'a str

    let x = String::from("123");
    let s;
    {
        let y = String::from("45678");
        s = first_str(x.as_ref(), y.as_ref());
    }
    println!("The first string is: {}", s);

    let x1 = 10;
    let p;
    {
        let y1 = 20;
        p = Point2 { _x: &x1, _y: y1 };
    }
    println!("point = {:?}", p);

    let x = 10;
    let b = Borrowed(&x);

    println!("borrowed: {}", b.0);

    let p = Person {
        name: "jeff",
        age: 10,
    };

    println!("age: {}", p.get_age());
    println!("name: {}", p.get_name());
    println!("name: {}", p.print_alias("park"));
    println!("longest: {}", p.longest_name("abcdefg"));
}

fn first_str<'a>(s1: &'a str, s2: &str) -> &'a str {
    println!("s2={}", s2);
    s1
}
