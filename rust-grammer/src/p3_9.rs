pub fn run() {
    // iter(): 반복자에 의해 접근되는 원소들의 레펀런스가 넘어온다. 소유권이 이동되는 것이 아니다.
    // into_iter(): 컬렉션 자체가 넘겨져서, 소유권이 넘어가 버리기에, into_iter()를 수행하고 난 후에는, 해당 컬렉션 변수로의 접근이 안된다.
    // iter_mut(): 컬렉션의 값을 수정해야할 때 사용한다. 레퍼런스로 받은 다음에 수정하는 것이다. 소유권이 넘어가지는 않는다.

    // take(n) : iterator에서 n개의 원소를 취해서 iterator를 만든다.
    // windows(n) : iterator에서 n개씩의 원소를 취해서 iterator를 만든다.
    // step_by(n) : iterator에서 n개씩 건너 뛰면서 원소를 취해서 iterator를 만든다.
    // for_each(|x| ...) : iterator의 각 원소에 대해 클로저를 호출한다.
}

#[test]
fn iter_test() {
    let v = vec![1, 2, 3, 4, 5];
    for val in v.iter() {
        print!("{} ", val);
    }
    println!("");

    for val in &v {
        print!("{} ", val);
    }
}

#[test]
fn into_iter_test() {
    let v = vec![1, 2, 3, 4, 5];
    for val in v.into_iter() {
        print!("{} ", val);
    }
    println!("");

    // error occured
    // for val in &v {
    //     print!("{} ", val);
    // }
}

#[test]
fn iter_mut_test() {
    let mut v = vec![1, 2, 3, 4, 5];
    for x in v.iter_mut() {
        *x += 1;
    }
    println!("{:?}", v);

    for val in &v {
        print!("{} ", val);
    }
}

#[test]
fn iter_mut_for_each_test() {
    let mut v = vec![1, 2, 3, 4, 5];
    v.iter_mut().for_each(|x| *x += 1);
    println!("{:?}", v);

    for val in &v {
        print!("{} ", val);
    }
}

#[test]
fn map_test() {
    let v = vec![1, 2, 3, 4, 5];
    let v1: Vec<_> = v.iter().map(|&x| x + 1).collect();

    println!("v={:?}", v);
    println!("v1={:?}", v1);

    // collect() : iterator의 내용을 collection으로 만든다. 어떤 컬렉션으로 만들지는 지정해줘야한다.
    // sum() : iterator의 내용을 합한 결과를 리턴. 결과가 어떤 타입(u32 등)일 지는 지정해줘야 한다.
    // max() : 최댓값을 리턴. 타입 지정 필요
    // min() : 최솟값을 리턴. 타입 지정 필요
    // count() : iterator의 원소 개수 리턴
    // product() : iterator의 각 원소를 곱한 결과를 리턴. 타입 지정 필요
}

#[test]
fn map_test2() {
    let ans: Vec<_> = (1..=100).map(|x| x * x).collect();
    println!("{:?}", ans);

    let ans = (1..=100).map(|x| x * x).collect::<Vec<_>>();
    println!("{:?}", ans);
}

#[test]
fn filter_test() {
    let v = vec![1, 2, 3, 4, 5];
    let v1: Vec<_> = v.iter().filter(|&&x| x % 2 == 0).collect(); // iter()에 의해 리턴되는 것은 &i32 타입. 따라서 &&x로 받음

    println!("v={:?}", v);
    println!("v1={:?}", v1);
}

#[test]
fn filter_test1() {
    let v = vec![1, 2, 3, 4, 5];
    let v1: Vec<_> = v.iter().filter(|x| *x % 2 == 0).collect();

    println!("v={:?}", v);
    println!("v1={:?}", v1);
}

#[test]
fn filter_test2() {
    let v = vec![1, 2, 3, 4, 5];
    let v1: Vec<_> = v.into_iter().filter(|x| x % 2 == 0).collect();

    // println!("v={:?}", v); // error occured
    println!("v1={:?}", v1);
}

#[test]
fn filter_map_test() {
    let a = ["1", "two", "NaN", "four", "5"];
    let v: Vec<_> = a
        .iter()
        .map(|s| s.parse::<i32>())
        .filter(|s| s.is_ok())
        .map(|s| s.unwrap())
        .collect();
    println!("v={:?}", v);
}

#[test]
fn filter_map_test1() {
    let a = ["1", "two", "NaN", "four", "5"];
    let v: Vec<_> = a.iter().filter_map(|s| s.parse::<i32>().ok()).collect();

    println!("v={:?}", v);
}

#[test]
fn for_each_test() {
    let mut v = vec![1, 2, 3, 4, 5];
    v.iter_mut().for_each(|x| *x += 1);
    println!("{:?}", v);

    let mut v = vec![1; 10];
    v.iter_mut()
        .enumerate()
        .filter(|(i, _)| *i % 2 == 0)
        .for_each(|(_, val)| *val = 0);
    println!("{:?}", v);
}

#[test]
fn take_while_test() {
    let v = vec![1, 3, 5, 7, 9, 10, 11, 13, 15];
    let v1: Vec<_> = v.iter().filter(|x| *x % 2 == 1).collect();
    let v2: Vec<_> = v.iter().take_while(|x| *x % 2 == 1).collect();

    println!("v1={:?}", v1);
    println!("v2={:?}", v2);
}
