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
