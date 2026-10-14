use std::fmt::Write;
use std::io::{self, BufWriter, Read, Write as IOWrite};

fn main() {
    // 한 줄 입력 받기
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    // 여러 줄 순차적으로 입력 받기
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let n = input.trim().parse::<usize>().unwrap();

    for _ in 0..n {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
    }

    // 숫자 여러 개 입력 받기
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<usize>);

    let n = input.next().unwrap();

    // 다음 숫자들 뽑을 때마다 input.next().unwrap();

    // 문자열 여러 개 입력 받기
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();

    let n = input.next().unwrap().parse::<usize>().unwrap();
    let line = input.next().unwrap();

    // BufWriter를 이용해 여러 줄 출력하기
    // 락을 걸고 생명 주기가 끝날 때까지 유지되어 속도가 빠름
    let stdout = io::stdout();
    let mut writer = BufWriter::new(stdout);

    for i in 0..100_000 {
        writeln!(writer, "{i}").unwrap();
    }

    // String 인스턴스를 버퍼로 사용
    let mut output = String::new();

    for i in 0..100_000 {
        writeln!(output, "{i}").unwrap();
    }

    println!("{output}");
}
