pub fn run() {
    // ======================================== &str ========================================

    // Rust에서의 문자열은 항상 utf-8 인코딩 사용
    let s = "Hello, World!"; // s는 &str 타입 변수
    println!("len={}", s.len());

    // 문자 개수
    println!("cnt={}", s.chars().count());

    let s = "대한민국";
    println!("len={}", s.len());
    println!("cnt={}", s.chars().count());

    // 슬라이스
    let s = "hello";
    // 아래 두 경우 모두 에러 (문자열 인덱스에 의한 접근 허용하지 않음)
    // println!("{}", s[0]);
    // println!("{}", &s[0]);
    println!("{}", &s[0..1]); // h

    // 한글은 3바이트를 사용해 한 단어를 표현한다. 이 경우 1바이트 범위만 읽은 경우 에러 발생
    let s = "대한민국";
    // println!("{}", &s[0..1]);
    println!("{}", &s[0..3]);

    // 문자열을 문자로 변환
    let s = "010-9999-1234";
    println!("{:?}", get_number(s));
    println!("{}", s);

    // &str의 메서드
    // bytes
    let mut bytes = "bors".bytes();
    assert_eq!(Some(b'b'), bytes.next());
    assert_eq!(Some(b'o'), bytes.next());
    assert_eq!(Some(b'r'), bytes.next());
    assert_eq!(Some(b's'), bytes.next());

    assert_eq!(None, bytes.next());

    // contains
    let bananas = "bananas";
    assert!(bananas.contains("nana"));
    assert!(!bananas.contains("apples"));

    // find
    let s = "012345ABC";
    assert_eq!(s.find('0'), Some(0));
    assert_eq!(s.find("ABC"), Some(6));

    // lines
    let text = "foo\r\nbar\n\nbaz\r";
    let mut lines = text.lines();

    assert_eq!(Some("foo"), lines.next());
    assert_eq!(Some("bar"), lines.next());
    assert_eq!(Some(""), lines.next());

    assert_eq!(Some("baz\r"), lines.next());
    assert_eq!(None, lines.next());

    // parse
    let four: u32 = "4".parse().unwrap();
    assert_eq!(4, four);

    let four = "4".parse::<u32>();
    assert_eq!(Ok(4), four);

    // split_whitespace
    let mut iter = "A few words".split_whitespace();

    assert_eq!(Some("A"), iter.next());
    assert_eq!(Some("few"), iter.next());
    assert_eq!(Some("words"), iter.next());

    assert_eq!(None, iter.next());

    // trim
    let s = "\n Hello\tworld\t\n";
    assert_eq!("Hello\tworld", s.trim());

    // to_lowercase
    let s = "HELLO";
    assert_eq!("hello", s.to_lowercase());

    // ======================================== String ========================================

    // hello 변수는 stack에 저장되지만 실제 문자값들은 힙 메모리에 저장
    let hello = String::from("Hello, World!");
    println!("{}", hello);

    // &str 에서 String 생성 (to_owned, to_string)
    let s1 = "Hello, ".to_owned();
    let s2 = "World!".to_string();
    println!("{}{}", s1, s2);

    // 문자열 합치기 String이 주체가 되어 &str을 합침. 앞쪽에는 String 타입이어야 함.
    // push_str
    let mut s = String::from("foo");
    s.push_str("bar");
    assert_eq!("foobar", s);

    // +
    let a: String = String::from("foo");
    let b: String = String::from("bar");
    let c = a + &b; // a의 소유권이 c로 이동되어 더 이상 변수 a로의 접근 불가

    println!("c={}", c);

    // format
    let a: String = String::from("foo");
    let b: String = String::from("bar");
    let c = format!("{}{}", &a, &b);
    println!("c={}", c);

    let c = format!("{}{}", "foo", "bar");
    println!("c={}", c);

    let c = format!("{}{}", a, b);
    println!("c={}", c);
    println!("a={}", a);

    // String 메서드
    // as_bytes
    let s = String::from("hello");
    assert_eq!(&[104, 101, 108, 108, 111], s.as_bytes());

    // as_str
    let s = String::from("foo");
    assert_eq!("foo", s.as_str());

    // clear
    let mut s = String::from("foo");
    s.clear();

    assert!(s.is_empty());
    assert_eq!(0, s.len());
    assert_eq!(3, s.capacity());

    // from_utf8
    let sparkle_heart = vec![240, 159, 146, 150];
    let sparkle_heart = String::from_utf8(sparkle_heart).unwrap();
    println!("{}", sparkle_heart);
    assert_eq!("💖", sparkle_heart);

    // insert
    let mut s = String::with_capacity(3);
    s.insert(0, 'f');
    s.insert(1, 'o');
    s.insert(2, 'o');

    assert_eq!("foo", s);

    // insert_str
    let mut s = String::from("bar");
    s.insert_str(0, "foo");
    assert_eq!("foobar", s);

    // into_bytes
    let s = String::from("hello");
    let bytes = s.into_bytes();

    println!("{:?} {:?}", bytes, &bytes[..]);
    assert_eq!(&[104, 101, 108, 108, 111][..], &bytes[..]);

    // push
    let mut s = String::from("abc");
    s.push('1');
    s.push('2');
    s.push('3');

    assert_eq!("abc123", s);

    // push_str
    let mut s = String::from("foo");
    s.push_str("bar");
    assert_eq!("foobar", s);

    // remove
    let mut s = String::from("abc");
    assert_eq!(s.remove(0), 'a');
    assert_eq!(s.remove(1), 'c');
    assert_eq!(s.remove(0), 'b');

    // truncate
    let mut s = String::from("hello");
    s.truncate(2);
    assert_eq!("he", s);

    // ======================================== &srt과 String ========================================

    /*
    Rust에서 어떤 변수를 선언하면, 그 변수의 정보는 모두 스택 메모리에 저장된다.

    &str과 String 타입 변수도 그 정보는 스택에 저장된다.
    그런데, &str에서의 실제 값인 문자들 값은 프로그램이 실행될 때 미리 확보된 특수한 읽기 전용 메모리에 적재된다. &str 변수의 스택 정보는 그 문자들 값이 저장된 메모리 위치만을 가지고 있는 것이다.
    그리고 String은, 문자들 값이 힙 공간에 위치하는 것이고, 힙 공간의 특성이 메모리 사용공간을 더 확대/축소 할 수 있는 것이기에, 문자들을 위한 공간을 더 늘리거나 축소 시킬 수 있다.
    */

    // ======================================== 문자열 관련 예제 ========================================

    // 예제 1. 문자열에 있는 숫자문자를 숫자로 변환하기
    const INPUT1: &str = "
        a3167176531330624919225119674426574742355349194934
        b6983520312774506326239578318016984801869478851843
        c5861560789112949495459501737958331952853208805511
    ";
    let v = get_num_vector(INPUT1);
    assert_eq!(vec![3, 1, 6, 7, 1], &v[0..5]);

    // 예제 2. 라인단위 읽기
    const INPUT2: &str = r"37107287533902102798797998220837590246510135740250
    46376937677490009712648124896970078050417018260538
    74324986199524741059474233309513058123726617309629
    91942213363574161572522430563301811072406154908250
    23067588207539346171171980310421047513778063246676";

    let sum = cal_sum(INPUT2);
    assert_eq!(271, sum);

    // 예제 3. 숫자 변환 후 짝수만 더하기
    let input: &str = "73167176531330624919225119674426574742355349194934";
    let sum: u32 = input
        .chars()
        .filter_map(|c| c.to_digit(10))
        .filter(|d| *d % 2 == 0)
        .sum();
    assert_eq!(72, sum);
}

fn get_number(s: &str) -> Vec<u32> {
    s.chars()
        .into_iter()
        .filter_map(|c| c.to_digit(10))
        .collect::<Vec<u32>>()
}

fn get_num_vector(s: &str) -> Vec<u64> {
    // case 1
    // let mut v: Vec<u64> = Vec::<u64>::new();

    // for c in s.chars().into_iter() {
    //     let d: Option<u32> = c.to_digit(10);
    //     if d.is_none() {
    //         continue;
    //     }
    //     v.push(d.unwrap() as u64);
    // }
    // v

    // case 2
    s.chars()
        .filter_map(|c| c.to_digit(10).map(u64::from))
        .collect()
}

fn cal_sum(input: &str) -> u64 {
    input
        .lines()
        .map(|line| line.trim())
        .map(|line| &line[0..2])
        .filter_map(|s| s.parse::<u64>().ok())
        .sum()
}
