/*
 * https://wikidocs.net/book/16747
 * Just Do Rust - 러스트 기초부터 고급까지
 */
use std::env; // Rust에서 기본으로 제공하는 std::env 라이브러리 사용하겠다는 의미

mod p3_1; // program1.rs 파일을 모듈로 사용하겠다는 의미
mod p3_2;
mod p3_3;
mod p3_4;
mod p3_5;
mod p3_6;

/**
 * Rust의 main 함수
 * - 일반적으로 main 함수는 리턴 타입 없이 짜도 무방하나, Result 타입으로 리턴 가능
 */
fn main() -> Result<(), i32> {
    // 실행 인자를 아래와 같이 받아올 수 있다. &args[0]에는 프로그램 이름이 들어간다.
    let args: Vec<String> = env::args().collect();

    let program = &args[1];

    match program.as_str() {
        "3.1" => p3_1::run(),
        "3.2" => p3_2::run(),
        "3.3" => p3_3::run(),
        "3.4" => p3_4::run(),
        "3.5" => p3_5::run(),
        "3.6" => p3_6::run(),
        _ => println!("Invalid program number"),
    }

    Ok(())
}
