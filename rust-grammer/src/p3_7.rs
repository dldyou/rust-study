pub fn run() {
    // ======================================== Vec ========================================

    let mut vv: Vec<i32> = Vec::new();
    vv.push(1);
    vv.push(2);
    println!("{:?}", vv);

    vv = vec![5, 6, 7];
    println!("{:?}", vv);

    vv = vec![1; 5];
    println!("{:?}", vv);

    // arr.to_vec()
    let arr = [3, 4];
    let v = arr.to_vec(); // 이전 선언된 let v는 어떻게 되는가? -> shadowing에 따라 이전 v는 더 이상 이름으로 접근 불가능 (컴파일러가 정한 지점에서 drop됨)
    println!("{:?}", v);

    // 어떤 배열값을 추가로 push하는 것은 extend
    let mut v = vec![1, 2, 3];
    v.extend([4, 5, 6, 7]);
    println!("{:?}", v);

    // iterator adapter
    let mut v = vec![1, 2, 3];
    let arr = [4, 5, 6, 7];
    arr.into_iter().for_each(|x| v.push(x));
    println!("{:?}", v); // [1, 2, 3, 4, 5, 6, 7]

    // 벡터 원소 접근
    let v = vec![5, 6, 7];
    println!("v[0]={}", v[0]);
    println!("last element={}", v[v.len() - 1]);

    // get으로도 접근 가능
    let v = vec![1, 2, 3];
    // println!("{}", v[3]); // panic!!!
    println!("{:?}", v.get(3));

    if let Some(n) = v.get(1) {
        println!("v.get(1)={}", n);
    }

    // 반복자 이용
    // for a in v.into_iter() == for a in v
    // for a in v.iter() == for a in &v

    // a는 원소의 레퍼런스라 *a를 해야 원소의 내용에 접근
    for a in &v {
        print!("{} ", *a);
    }
    println!("");

    // but, println!을 하는 경우 a도 동일하게 처리
    for a in &v {
        print!("{} ", a);
    }
    println!("");

    // 소유권 이동
    for a in v {
        print!("{} ", a);
    }
    println!("");

    // println!("{}", v[0]); // error

    // 벡터의 원소 업데이트 하는 방법
    // 인덱스 접근
    let mut v = vec![1, 2, 3];
    for i in 0..v.len() {
        v[i] *= 2;
    }
    println!("{:?}", v);

    // mutable 반복자: &mut v
    let mut v = vec![1, 2, 3];
    for a in &mut v {
        *a *= 2;
    }
    println!("{:?}", v);

    // mutable 반복자: v.iter_mut()
    let mut v = vec![1, 2, 3];
    for a in v.iter_mut() {
        *a *= 2;
    }
    println!("{:?}", v);

    // iterator adapter
    let mut v = vec![1, 2, 3];
    v.iter_mut().for_each(|a| *a *= 2);
    println!("{:?}", v);

    // 벡터를 스택으로 사용하기
    let mut s: Vec<i32> = Vec::new();

    s.push(0);
    s.push(1);
    s.push(2);

    while s.len() > 0 {
        let i = s.pop().unwrap(); // ok 변형에서 값을 추출 but None인 경우 panic
        println!("pop: {}", i);
    }

    // Rust에서 Queue를 위해서는 VecDeque 혹은 LinkedList를 쓰면 된다

    // 벡터의 메서드
    let mut vec = vec![1, 2, 3];
    let mut vec2 = vec![4, 5, 6];
    vec.append(&mut vec2);

    let mut v = vec![1, 2, 3];
    v.clear(); // 벡터 데이터 제거. 크기가 줄어 들지는 않음
    assert!(v.is_empty());

    let a = vec![1, 2, 3];
    assert_eq!(a.len(), 3);

    let mut v = Vec::new();
    assert!(v.is_empty());

    v.push(1);
    assert!(!v.is_empty());

    // insert
    let mut vec = vec![1, 2, 3];
    vec.insert(1, 4);
    assert_eq!(vec, [1, 4, 2, 3]);
    vec.insert(4, 5);
    assert_eq!(vec, [1, 4, 2, 3, 5]);

    // pop
    let mut vec = vec![1, 2, 3];
    assert_eq!(vec.pop(), Some(3));
    assert_eq!(vec, [1, 2]);

    // push
    let mut vec = vec![1, 2];
    vec.push(3);
    assert_eq!(vec, [1, 2, 3]);

    // remove
    let mut v = vec![1, 2, 3];
    assert_eq!(v.remove(1), 2);
    assert_eq!(v, [1, 3]);

    // resize
    let mut vec = vec!["hello"];
    vec.resize(3, "world"); // resize해서 기존보다 크면 확장된 부분을 value로 채운다
    assert_eq!(vec, ["hello", "world", "world"]);

    let mut vec = vec![1, 2, 3, 4];
    vec.resize(2, 0); // resize해서 기존보다 작으면 짤려 없어짐. 단순하게 축소하는 거면 truncate 사용하는게 낫다
    assert_eq!(vec, [1, 2]);

    // truncate
    let mut vec = vec![1, 2, 3, 4, 5];
    vec.truncate(2);
    assert_eq!(vec, [1, 2]);

    let mut vec = vec![1, 2, 3];
    vec.truncate(8); // 기존 크기보다 크면, 아무 영향도 안 미침
    assert_eq!(vec, [1, 2, 3]);

    let mut vec = vec![1, 2, 3];
    vec.truncate(0);
    assert_eq!(vec, []);

    // fill
    let mut buf = vec![0; 10];
    buf.fill(1); // 모든 원소를 채움
    assert_eq!(buf, vec![1; 10]);

    // reverse
    let mut v = [1, 2, 3];
    v.reverse();
    assert!(v == [3, 2, 1]);

    // sort
    let mut v = vec![4, -5, 1, -3, 2];
    v.sort();
    assert_eq!(v, vec![-5, -3, 1, 2, 4]);
}
