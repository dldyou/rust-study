pub fn run() {
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
}

fn get_number(s: &str) -> Vec<u32> {
    s.chars()
        .into_iter()
        .filter_map(|c| c.to_digit(10))
        .collect::<Vec<u32>>()
}
