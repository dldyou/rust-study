// 스마트 포인터
/*
'스마트 포인터(smart Pointers)'는 어떤 데이터에 대한 주솟값과 함께, 이 주소값을 관리하는 메타 데이터도 같이 가지고 있는 데이터 구조다.
Rust는 몇 개의 '스마트 포인터' 특성을 가지는 데이터 구조를 제공한다. 이를 통해, 데이터가 힙 메모리에 저장되도록 강제화 할 수 있게 하거나(Box), 스택에 있는 다른 두 변수가 같은 힙 공간을 가리킬 수 있다(Rc, Arc). 즉, 일반적인 Rust 기본 규칙으로는 할 수 없는 기능을 수행할 수 있게 한다.

이러한 '스마트 포인터'는 String, Vec, Box, Rc, Arc 등이 있다.

String과 Vec는 이미 앞에서 다룬 데이터 구조다.
String 변수의 경우 힙 공간에 실제 문자열이 저장되고 스택에는 해당 힙 공간을 가리키는 주소와 해당 크기에 해당하는 메타 정보를 가지고 있다.
*/

// 스마트 포인터의 구조체는 Deref와 Drop 트레잇을 구현한다.
// Deref *x와 같이 스마트 포인터 객체에 *가 붙여 졌을 때의 동작을 정의하는 것
// Drop 은 스마트 포인터 객체가 스코프를 벗어날 때 자동으로 수행. 힙 메모리를 해제하기 위함. 프로그래머가 따로 신경 쓸 것은 별로 없다.

// enum List {
//     Cons(i32, Box<List>),
//     Nil, // 제일 마지막에 위치하는 노드의 이름인 Nil은 라틴어의 nil에서 유래
// }
// use List::{Cons, Nil};

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
        println!("A dog is running")
    }
}

fn find_runner(is_human: bool) -> Box<dyn Moving> {
    if is_human {
        Box::new(Human)
    } else {
        Box::new(Dog)
    }
}

pub fn run() {
    // Box의 기본 사용법
    let b = Box::new(5);
    println!("value = {}", b); // 5

    // 큰 데이터는 그냥 벡터를 쓰면 됨.
    /*
    Box를 써서 유용한 경우는 아래와 같은 경우
    1. 일반적인 방법으로는 컴파일 타임에 데이터의 크기를 한정할 수 없어 에러가 날 때
    2. 특정 트레잇을 구현한 타입을 소유하고자 할 때
    3. 큰 크기의 데이터를 실제 복제하지 않고 소유권을 이동시키고자 할 때
    */

    // struct Node {
    //     // next_node: Option<Node>, // error
    //     next_node: Option<Box<Node>>,
    // }

    // let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    let x = find_runner(true);
    x.run();

    // 힙 영역 데이터 그대로, 소유권이 b2로 이동해서 복사가 바로 이루어짐
    let b1 = Box::new([1, 2, 3, 4]);
    let _b2 = b1;
}
