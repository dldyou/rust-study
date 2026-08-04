use std::collections::HashMap;

pub fn run() {
    /* 해당 코드 수정하기
    let a: Vec<_> = (1..=100).collect();
    let b: Vec<_> = a.into_iter().filter(|i| i % 2 == 0).collect();

    println!("a's length={}", a.len());
    println!("b's length={}", b.len());
    */

    let a: Vec<_> = (1..=100).collect();
    let b: Vec<_> = a.iter().filter(|i| *i % 2 == 0).collect(); // a.clone().into_iter()로 써도 됨

    println!("a's length={}", a.len());
    println!("b's length={}", b.len());

    /* 해당 코드 수정하기
    let mut v = vec![3,4,5,6,7];

    for i in v {
        i += 10;
    }
    println!("{:?}",v);
    */
    let mut v = vec![3, 4, 5, 6, 7];

    for i in &mut v {
        *i += 10;
    } // v.iter_mut().for_each(|i| *i += 10)
    println!("{:?}", v);

    /* 해당 코드 수정하기
    let map = HashMap::from([
        ("Alice", 30), ("Bob", 40), ("Dave", 60), ("Jeff", 80)
    ]);
    let over_70 = over_limit(map, 70);
    let over_50 = over_limit(map,50); //에러
    println!("over 70: {:?}",over_70);
    println!("over 50: {:?}",over_50);
    */
    let map = HashMap::from([("Alice", 30), ("Bob", 40), ("Dave", 60), ("Jeff", 80)]);
    let over_70 = over_limit(&map, 70);
    let over_50 = over_limit(&map, 50);
    println!("over 70: {:?}", over_70);
    println!("over 50: {:?}", over_50);

    // move
    let s1 = String::from("hello"); // heap memory
    let _s2 = s1;

    // println!("s1 = {}", s1); //error

    // copy
    let a1 = 10; // primitive type 이기에 stack memory
    let a2 = a1;

    println!("a1={}", a1);
    println!("a2={}", a2);

    #[derive(Debug)] // 구조체 객체를 {:?}을 이용해서 출력하려면 #[derive(Debug)] 속성을 부여해줘야 한다.
    struct Foo; // 구조체는 Copy 트레잇이 자동 구현되어 있지 않기에 'move'가 발생한다. 즉, 소유권이 이동된다.

    let st1 = Foo;
    let st2 = st1; // moved

    // println!("st1={:?}", st1);
    println!("st2={:?}", st2);

    copy_trait();
    copy_trait1();

    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1={}", s1);
    println!("s2={}", s2);

    // 각 구성원들은 Copy 트레잇이 구현되어 있어야 함
    #[derive(Copy, Clone)]
    struct Point {
        x: i32,
        y: i32,
    }

    // Vec는 힙 메모리에 데이터가 저장되는 타입으로 Copy 트레잇이 구현되어 있지 않다.
    // #[derive(Copy, Clone)] //error
    // struct PointList {
    //     points: Vec<Point>,
    // }
}

fn over_limit(map: &HashMap<&str, i32>, limit: i32) -> Vec<String> {
    let mut name: Vec<String> = vec![];
    for (k, v) in map {
        if *v > limit {
            name.push(k.to_string());
        }
    }
    return name;
}

/*
fn over_limit(map:&HashMap<&str,i32>, limit:i32) -> Vec<String> {
    return map.iter()
        .filter(|(_,v)| *v > &limit)
        .map(|(k,_)| k.to_string())
        .collect::<Vec<_>>();
}
*/

fn copy_trait() {
    #[derive(Debug, Copy, Clone)] // Debug속성은 {:?}에 의해서 출력이 가능하도록하는 것이고, Copy, Clone은 Copy가 일어나도록 하기 위함이다.
    struct Foo;

    let st1 = Foo;
    let st2 = st1;

    println!("st1={:?}", st1);
    println!("st2={:?}", st2);
}

fn copy_trait1() {
    #[derive(Debug)]
    struct Foo;

    impl Copy for Foo {} // (1)

    impl Clone for Foo {
        // (2)
        fn clone(&self) -> Foo {
            // (3)
            *self // (4)
        }
    }

    let st1 = Foo;
    let st2 = st1; // (5)

    println!("st1={:?}", st1);
    println!("st2={:?}", st2);
}
