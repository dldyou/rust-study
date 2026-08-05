// 구조체의 정의
#[derive(Debug)]
struct Student {
    name: String,
    point: i32,
}

#[derive(Default)]
struct People {
    age: u32,
    name: String,
    is_male: bool,
}

struct Point(i32, i32);
struct Dot {
    x: i32,
    y: i32,
}

impl Dot {
    fn new(x: i32, y: i32) -> Dot {
        Dot { x: x, y: y }
    }

    fn distance(&self, p: &Dot) -> f64 {
        (((p.x - self.x).pow(2) + (p.y - self.y).pow(2)) as f64).sqrt()
    }
}

// 아무 멤버도 없는 비어있는 구조체
// - 타입을 구분하고자 할 때
// - 필드값은 없이 메서드만 구현하고 싶을 때
pub struct MyEmptyStruct;

impl MyEmptyStruct {
    fn print(&self) {
        println!("MyEmptyStruct 인스턴스가 생성되었습니다.");
    }
}

// unit-like struct: 데이터 없이 타입 구분만을 위해 사용
struct Marker;

// 일반 struct: 실제 데이터를 가집
struct Data(i32);

// 트레이트 정의
trait MyTrait {
    fn whoami(&self);
}

// Marker에 트레이트 구현
impl MyTrait for Marker {
    fn whoami(&self) {
        println!("I am a Marker (unit-like struct, no data)!");
    }
}

// Data에 트레이트 구현
impl MyTrait for Data {
    fn whoami(&self) {
        println!("I am Data, value = {}", self.0);
    }
}

fn do_something<T: MyTrait>(item: &T) {
    item.whoami();
}

pub fn run() {
    // 쓰기 가능 인스턴스 만들기: let mut s1
    let mut s1 = Student {
        name: "Jeff".to_owned(),
        point: 80,
    };
    s1.point = 100;

    println!("{:?}", s1);
    println!("name={}, point={}", s1.name, s1.point);

    let s1 = make_student("Jeff".to_owned(), 80);
    println!("name={}, point={}", s1.name, s1.point);

    let p1: People = Default::default(); // People::default() 로 써도 됨. 의미 전달은 여기가 더 잘 되는 느낌
    println!(
        "Default config: age={}, name='{}', is_male={}",
        p1.age, p1.name, p1.is_male
    );

    let p1 = Point(0, 0);
    let p2 = Point(3, 4);
    let dist = cal_distance(&p1, &p2);
    assert_eq!(5.0, dist);

    let p1 = Dot::new(0, 0); // new는 &self가 없으므로 연관 함수, :: 사용해서 호출
    let p2 = Dot::new(3, 4); // 메서드
    assert_eq!(5.0, p1.distance(&p2));

    let instance = MyEmptyStruct;
    instance.print();

    let m = Marker;
    let d = Data(42);
    do_something(&m);
    do_something(&d);
}

fn make_student(name: String, point: i32) -> Student {
    Student {
        name: name,
        point: point,
    }
    // Student { name, point }
}

fn cal_distance(p1: &Point, p2: &Point) -> f64 {
    (((p2.0 - p1.0).pow(2) + (p2.1 - p1.1).pow(2)) as f64).sqrt()
}
