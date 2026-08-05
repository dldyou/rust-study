use std::collections::HashMap;
use std::{fs::File, io::Write};

enum Gender {
    Male { name: String, is_military: bool },
    Female { name: String },
}

// Rust에서 ::는 스태틱한 필드나 함수에 접근할 때, .은 인스턴스의 메서드나 필드에 접근할 때 사용

fn get_customer(id: i32) -> Gender {
    if id % 2 == 0 {
        return Gender::Male {
            name: "Jeff".to_owned(),
            is_military: true,
        };
    }
    return Gender::Female {
        name: "Alice".to_owned(),
    };
}

// Rust 에서 가장 많이 사용되는 열거형은 Option과 Result
// Option: 어떤 값을 리턴할 때 "값이 없을 수도 있을 때"를 위해 None이라는 타입이 있음
// Result: 어떤 값을 리턴할 때 "에러가 있는 경우"를 위해 Err이라는 타입이 있음

pub fn run() {
    let gender = get_customer(10);
    match gender {
        Gender::Male {
            name: n,
            is_military: b,
        } => println!("name={}, is_military={}", n, b),
        Gender::Female { name: n } => {
            println!("name={}", n);
        }
    }

    let map = HashMap::from([("Jeff", 80), ("Alice", 100)]);
    let name = "Jeff";
    let point = map.get(name).unwrap();
    println!("{}'s point = {}", name, point);

    // None 처리해준 코드
    let name = "Bob";
    match map.get(name) {
        Some(point) => println!("{}'s point = {}", name, point),
        None => println!("There is no name of {}", name),
    }

    // if let
    let name = "Jeff";
    if let Some(point) = map.get(name) {
        println!("{}'s point = {}", name, point);
    } else {
        println!("There is no name of {}", name);
    }

    /*
    let num_str = "010-1234-5678";으로 선언된 num_str에서
    숫자 부분만 뽑아내서 Vec:<u32>형태의 벡터에 저장하고, 전체 숫자를 프린트하는 프로그램을 짜세요.
    */
    let num_str = "010-1234-5678";
    let mut v: Vec<u32> = Vec::new();
    for c in num_str.chars() {
        match c.to_digit(10) {
            Some(c) => v.push(c),
            None => (),
        }
    }
    println!("{:?}", v);

    let num_str = "010-1234-5678";
    let mut v: Vec<u32> = Vec::new();
    for c in num_str.chars() {
        if let Some(d) = c.to_digit(10) {
            v.push(d);
        } else {
        }
    }
    println!("{:?}", v);

    let num_str = "010-1234-5678";
    let v: Vec<u32> = num_str.chars().filter_map(|c| c.to_digit(10)).collect();
    println!("{:?}", v);

    match divmod(10, 0) {
        Ok((q, r)) => println!("(quotient, remainder)={:?}", (q, r)),
        Err(e) => println!("Error: {}", e),
    }

    match write_file("*.txt") {
        Ok(()) => {
            println!("File Writing Success");
        }
        Err(error) => {
            println!("Error: {}", error);
        }
    }

    match write_file_short("*.txt") {
        Ok(()) => {
            println!("File Writing Success");
        }
        Err(error) => {
            println!("Error: {}", error);
        }
    }

    // expect(err_msg)를 통한 panic 처리
    write_file_short("*.txt").expect("File Writing Error");

    // unwrap으로 간략화
    write_file_short("*.txt").unwrap();

    // result 처리 원칙: https://wikidocs.net/268478
}

fn divmod(n: i32, d: i32) -> Result<(i32, i32), String> {
    if d == 0 {
        Err("can't divide by zero".to_owned())
    } else {
        Ok((n / d, n % d))
    }
}

fn write_file(f_name: &str) -> Result<(), String> {
    let mut f = match File::create(f_name) {
        Ok(file) => file,
        Err(error) => {
            return Err(format!("File Creation Error: {}", error));
        }
    };
    let _ = f.write_all(b"hello");
    return Ok(());
}

fn write_file_short(f_name: &str) -> Result<(), std::io::Error> {
    let mut f = File::create(f_name)?;
    let _ = f.write_all(b"hello");
    return Ok(());
}
