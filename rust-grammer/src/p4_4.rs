/*
Rust의 트레잇은 타입, 상수, 함수를 항목으로 가질 수 있다.

- 타입 (type): 공통으로 사용될 수 있는 타입 정의 가능 (디폴트 할당 불가능)
- 상수 (const): 트레잇에 정의되어 해당 트레잇을 impl하는 개체들에서 공통으로 사용 가능
- 함수 (function): 공통으로 사용되는 함수
*/

trait ExTrait {
    type TypeNoDefault;
    const CONST_DEFAULT: i32 = 100;
    const CONST_NO_DEFAULT: i32;

    fn method_default(&self) {
        println!("default method");
    } // 함수의 본체까지 정의. 구현 측에서 다시 구현 할 수 있음
    fn method_without_default(&self); // 공통 메서드 정의. 구현하는 측에서 함수 바디 구현해야 함
}

struct ExStruct {}

impl ExTrait for ExStruct {
    type TypeNoDefault = f64;

    const CONST_DEFAULT: i32 = 101;
    const CONST_NO_DEFAULT: i32 = 102;

    fn method_default(&self) {
        println!("method is re-implemented.");
    }

    fn method_without_default(&self) {
        println!("method_without_default is implemented.");
    }
}

trait Car {
    fn drive(&self);
}

struct Truck {}

impl Car for Truck {
    fn drive(&self) {
        println!("Truck is driving.");
    }
}

struct SUV {}
impl Car for SUV {
    fn drive(&self) {
        println!("SUV is driving.");
    }
}

struct Sedan {}
impl Car for Sedan {
    fn drive(&self) {
        println!("Sedan is driving.");
    }
}

#[derive(Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl std::ops::Add for Point {
    type Output = Self; // Point를 의미

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl std::ops::Sub for Point {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.y,
            y: self.y - rhs.y,
        }
    }
}

// impl Clone for Point {
//     fn clone(&self) -> Self {
//         Self {
//             x: self.x.clone(),
//             y: self.y.clone(),
//         }
//     }
// }

trait MyOp {
    type Output;
    fn add(&self, rhs: &Self) -> Self::Output;
    fn sub(&self, rhs: &Self) -> Self::Output;
}

impl MyOp for Point {
    type Output = Self;

    fn add(&self, rhs: &Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }

    fn sub(&self, rhs: &Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.x - rhs.y,
        }
    }
}

trait PointOp {
    fn len(&self) -> f64;
}

impl PointOp for Point {
    fn len(&self) -> f64 {
        ((self.x.pow(2) + self.y.pow(2)) as f64).sqrt()
    }
}

fn print_distance<T: PointOp>(p: &T) {
    println!("Distance={}", p.len());
}

// 트레잇 바운드는 제네릭 타입을 정의하는 부분에 <T: PointOp>라고 할 수도 있고 where를 써서 표현할 수도 있다.
fn min<T>(a: T, b: T) -> T
where
    T: std::cmp::PartialOrd,
{
    if a > b { b } else { a }
}

trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}

struct Counter {
    cnt: u32,
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cnt > 100 {
            return None;
        } else {
            self.cnt += 1;
            return Some(self.cnt);
        }
    }
}

// 연관 타입이 여러 개인 경우
struct Node {
    id: i32,
    name: String,
    value: i32,
}

struct Edge {
    start: Node,
    end: Node,
}

struct MyGraph {}

impl Graph for MyGraph {
    type N = Node;
    type E = Edge;

    fn get_value(&self, n: &Self::N) -> i32 {
        return n.value;
    }

    fn edges(&self, n: &Self::N) -> Vec<Self::E> {
        todo!()
    }
}

trait Graph {
    type N;
    type E;

    fn get_value(&self, n: &Self::N) -> i32;
    fn edges(&self, n: &Self::N) -> Vec<Self::E>;
}

fn distance<G: Graph>(g: &G, s: &G::N, e: &G::N) -> i32 {
    g.get_value(e) - g.get_value(s)
}

struct Human;
struct Dog;

trait Moving {
    fn run(&self);
}

impl Moving for Human {
    fn run(&self) {
        println!("A human is running.");
    }
}

impl Moving for Dog {
    fn run(&self) {
        println!("A dog is running.");
    }
}

fn runm(x: impl Moving) {
    x.run();
}

// impl trait은 함수의 리턴 타입에도 사용 가능
// Moving 트레잇을 구현한 객체를 리턴한다는 의미
// 하지만 리턴 위치에 이렇게 사용해도 타입이 한 가지로 고정되어 한정적임. (컴파일 시간에 타입이 정해져야하기에, 정적 바인딩)
// 함수 내부에서 여러 타입이 리턴될 수 있으려면 리턴을 impl Trait처럼 하는 것이 아니고 Box<dyn Moving>과 같이 해야함
fn who_moved_there() -> impl Moving {
    let jeff = Human;
    return jeff;
}

// 런타임에 리턴되는 타입을 바인딩해서 수행하는 동적 디스패치
// Box가 스마트 포인터로 해당 객체를 힙에 위치시키고 그 주소를 가리키는 포인터 역할을 함
// dyn Moving의 참조자는 인스턴스 객체를 위한 포인터와 virtual table을 가리키는 포인터의 총 두 개의 포인터를 갖는다.
fn find_runner(is_human: bool) -> Box<dyn Moving> {
    if is_human {
        Box::new(Human)
    } else {
        Box::new(Dog)
    }
}

pub fn run() {
    let s = ExStruct {};
    let a: <ExStruct as ExTrait>::TypeNoDefault = 1.0;
    println!("a={}", a);

    let a = String::from("aa");
    let b = a;

    println!("CONST_DEFAULT={}", ExStruct::CONST_DEFAULT);
    println!("CONST_NO_DEFAULT={}", ExStruct::CONST_NO_DEFAULT);

    s.method_default();
    s.method_without_default();

    let car1 = Truck {};
    car1.drive();

    let car2 = SUV {};
    car2.drive();

    let car3 = Sedan {};
    car3.drive();

    let p1 = Point { x: 2, y: 3 };
    let p2 = Point { x: 4, y: 5 };
    let p3 = p1.clone() + p2.clone();
    let p4 = p1.clone() - p2.clone();
    println!("addition=({}, {})", p3.x, p3.y);
    println!("subtract=({}, {})", p4.x, p4.y);

    let p1 = Point { x: 2, y: 3 };
    let p2 = Point { x: 4, y: 5 };
    let p3 = p1.add(&p2);
    let p4 = p1.sub(&p2);
    println!("addition=({}, {})", p3.x, p3.y);
    println!("subtract=({}, {})", p4.x, p4.y);

    let p1 = Point { x: 2, y: 3 };
    print_distance(&p1);

    let mut counter = Counter { cnt: 1 };
    println!("{}", counter.next().unwrap());

    let tom = Human;
    let hodu = Dog;

    runm(tom);
    runm(hodu);

    let w = who_moved_there();
    runm(w);

    let x = find_runner(false);
    // find_runner로 얻어낸 x를 가지고 runm(x)를 할 수는 없음
    // runm 함수는 fn runm(x: impl Moving)으로 선언되어 파라미터로 Moving을 받을 수는 있으나, 현재 x의 타입인 Box<dyn Moving>을 받을 수는 없음
    x.run();
}
