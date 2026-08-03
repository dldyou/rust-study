use std::collections::HashMap;
use std::collections::HashSet;

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

    // ======================================== HashMap ========================================

    let mut map: HashMap<&str, i32> = HashMap::new();
    map.insert("Jeff", 100);
    map.insert("Tom", 90);
    println!("{:?}", map); // 순서 보장 X

    let map = HashMap::from([("Jeff", 100), ("Tom", 90)]);
    println!("{:?}", map); // 순서 보장 X

    // 벡터로부터 해시맵 생성
    let name = vec!["Jeff", "Tom", "Josh"];
    let score = vec![100, 90, 80];
    // zip: 반복자 2개에 대해 1:1 데이터 짝짓기 해줌 -> 튜플 3개 생김
    // collect에 의해 HashMap으로 생성
    let map: HashMap<_, _> = name.into_iter().zip(score.into_iter()).collect(); // _, _로 자동 추론
    println!("{:?}", map);

    // 데이터 접근
    let map = HashMap::from([("Jeff", 100), ("Tom", 90), ("Josh", 80)]);
    // key를 지정해서 get
    println!("Jeff's score: {}", map.get("Jeff").unwrap());

    for (k, val) in &map {
        println!("{}: {}", k, val);
    }

    for k in map.keys() {
        if k.starts_with("J") {
            print!("{} ", map.get(k).unwrap());
        }
    }
    println!("");

    // 데이터 갱신
    let mut map = HashMap::from([("Jeff", 100), ("Tom", 90), ("Josh", 80)]);

    // 덮어쓰기
    map.insert("Jeff", 50);
    println!("{:?}", map);

    // 만약 key가 있다면 갱신하지 않고 key가 없는 것에 대해서만 값을 쓰자고 하면 entry(key).or_insert(val) 메서드가 있음
    let mut map = HashMap::from([("Jeff", 100), ("Tom", 90), ("Josh", 80)]);

    let new_data = [("Jeff", 50), ("Alice", 10)];
    for (k, v) in &new_data {
        map.entry(k).or_insert(*v);
    }

    println!("{:?}", map);

    // entry(key).or_insert(val)은 &val을 return
    let text = "stay foolish stay hungry";
    let mut map = HashMap::new();

    for c in text.chars() {
        let cnt = map.entry(c).or_insert(0);
        *cnt += 1;
    }
    println!("{:?}", map);

    let text = "stay foolish stay hungry";
    let mut map = HashMap::new();

    for w in text.split_whitespace() {
        let cnt = map.entry(w).or_insert(0);
        *cnt += 1;
    }
    println!("{:?}", map);

    // 해시맵의 메서드
    // clear
    let mut a = HashMap::new();
    a.insert(1, "a");
    a.clear();
    assert!(a.is_empty());

    // contains_key
    let mut map = HashMap::new();
    map.insert(1, "a");
    assert_eq!(map.contains_key(&1), true);
    assert_eq!(map.contains_key(&2), false);

    // get
    let mut map = HashMap::new();
    map.insert(1, "a");
    assert_eq!(map.get(&1), Some(&"a"));
    assert_eq!(map.get(&2), None);

    // insert
    let mut map = HashMap::new();
    assert_eq!(map.insert(37, "a"), None);
    assert_eq!(map.is_empty(), false);

    map.insert(37, "b");
    assert_eq!(map.insert(37, "c"), Some("b"));
    assert_eq!(map[&37], "c");

    // keys
    let map = HashMap::from([("a", 1), ("b", 2), ("c", 3)]);

    for key in map.keys() {
        println!("{key}");
    }

    // remove
    let mut map = HashMap::new();
    map.insert(1, "a");
    assert_eq!(map.remove(&1), Some("a"));
    assert_eq!(map.remove(&1), None);

    // values
    let map = HashMap::from([("a", 1), ("b", 2), ("c", 3)]);

    for val in map.values() {
        println!("{val}");
    }

    // ======================================== HashSet ========================================

    // 해시셋 선언과 생성
    let mut set = HashSet::new();

    set.insert(1);
    set.insert(2);
    set.insert(3);
    set.insert(3);
    set.insert(4);
    set.insert(5);
    println!("{:?}", set);

    // 배열 / 벡터로부터 해시셋 생성
    let set = HashSet::from([1, 2, 3, 3, 4, 5]);
    println!("{:?}", set);

    // sorting된 상태로 출력하고 싶다면
    let mut set = HashSet::from([1, 2, 3]);
    set.extend([3, 4, 5].iter());
    println!("{:?}", set);

    let mut v: Vec<_> = set.into_iter().collect();
    v.sort();
    println!("{:?}", v);

    // 해시셋 데이터로의 접근
    let set = HashSet::from([1, 2, 3, 3, 4, 5]);
    let mut v: Vec<i32> = Vec::new();
    for x in set.iter() {
        if *x % 2 == 0 {
            v.push(*x);
        }
    }
    println!("{:?}", v);

    let set = HashSet::from([1, 2, 3, 3, 4, 5]);
    let v: Vec<_> = set.iter().filter(|x| *x % 2 == 0).collect();
    println!("{:?}", v);

    // 집합
    let a = HashSet::from([1, 2, 3, 4]);
    let b = HashSet::from([3, 4, 5]);

    let u: Vec<_> = a.union(&b).collect();
    let i: Vec<_> = a.intersection(&b).collect();
    let d: Vec<_> = a.difference(&b).collect();

    println!("union={:?}", u);
    println!("intersection={:?}", i);
    println!("difference={:?}", d);
}
