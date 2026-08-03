/*
함수명은 snake_case로 작성하는 것이 관례이다.

return 키워드 생략 가능, 세미콜론 없이 값이나 변수명 적어놓으면 그 값이 리턴된다.
*/
pub fn run() {
    let c = add(3, 5);
    println!("c = {}", c);

    let p1 = Point::new(0, 0);
    let p2 = Point::new(3, 4);

    println!("distance = {}", p1.distance(&p2));
    assert_eq!(5.0, p1.distance(&p2));

    // 클로저 "|x| x + 1"를 변수 add_one에 할당해서 함수처럼 사용 가능
    let add_one = |x: i32| x + 1;
    println!("{}", add_one(2)); // 3

    // 함수와 달리 파라미터의 타입 지정 안 해도 됨
    let add_two = |x| x + 2;
    println!("{}", add_two(2)); // 4

    // 파라미터가 없어도 됨
    let print_hello = || println!("hello");
    print_hello(); // hello

    // 바디는 {}로 감쌀 수 있다. 파라미터 여러 개 사용 가능
    let divmod = |x: i32, y: i32| {
        let q = x / y;
        let r = x % y;
        return (q, r);
    };
    println!("{:?}", divmod(10, 3)); // (3, 1)

    // 클로저를 꼭 써야만 하는 경우는 외부 변수를 함수의 내부에서 써야하는 경우
    let num = 100;
    // println!("{}", add_num(5));

    // fn add_num(x: i32) -> i32 {
    //     x + num // 에러
    // }
    let add_num = |x| x + num;
    println!("{}", add_num(5)); // 105

    // 실 사용 예
    // Rust에서 제공하는 스탠다드 함수가 클로저 형태의 파라미터를 요구해서 그것을 구현해야 할 때
    let v = vec![1, 2, 3];
    assert_eq!(1, get_val(&v, 3));
    assert_eq!(2, get_val(&v, 4));

    let v: Vec<i32> = Vec::new();
    assert_eq!(0, get_val(&v, 1));
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

/*
Rust에서 method는 function과 구분해서 사용
method: 인스턴스에 의해서만 호출됨 (instance.method())
function: 인스턴스와 상관없이 호출됨
*/

struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }

    fn distance(&self, p: &Point) -> f64 {
        (((p.x - self.x).pow(2) + (p.y - self.y).pow(2)) as f64).sqrt()
    }
}

/*
Rust에서 함수와 유사하게 사용되는 것으로 macro가 있다.

- 가장 큰 형태상 특징은 매크로는 이름 뒤에 느낌표 !가 있다.
- 함수는 작성된 텍스트 형태의 코드에 의해 바이너리 코드가 만들어지는 것이고,
매크로는 작성된 텍스트 형태의 코드에 의해 또 다른 Rust 코드가 만들어지는 것
*/

// 클로서 실 사용 예
fn get_val(v: &Vec<i32>, idx: usize) -> i32 {
    let val = v
        .get(idx)
        .unwrap_or_else(|| if v.get(0).is_some() { &v[0] } else { &0 });

    // unwarp_or_else()를 사용하지 않고 match로 구현한 경우
    // let val = match v.get(idx) {
    //     Some(x) => x,
    //     None => {
    //         if v.get(0).is_some() {
    //             &v[0]
    //         } else {
    //             &0
    //         }
    //     }
    // };
    return *val;
}
// filter 메서드에서도 클로저를 사용
