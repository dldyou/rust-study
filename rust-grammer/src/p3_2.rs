// 변수는 함수 밖에서 선언 불가, 상수는 가능
const MAX: u32 = 1000;
// let mut sum: i32 = 0;

pub fn run() {
    /*
     * Rust에서 변수 선언은 let 키워드 사용
     * 변수 이름은 snake_case를 사용 (한글 사용도 가능은 함)
     * 기본적으로 Read Only, 변경 가능하게 하려면 mut 키워드 사용 ex) let mut
     */
    let sum: i32 = 0;
    println!("sum = {}", sum);

    // 변수 타입은 생략 가능, but 명시적으로 타입을 지정하는 것이 좋음
    let mut x = 5;
    println!("x: {}", x); // 5
    x = 6;
    println!("x: {}", x); // 6

    // 상수 이름은 대문자와 언더스코어를 사용, const 키워드 사용
    // 상수는 mutable할 가능성 조차 없으며 타입과 값이 반드시 명시되어야 함
    const MIN: u32 = 1;
    println!("{}, {}", MIN, MAX);
    sub_fn();

    const MAX_FLOAT: f64 = 1000.123;
    const MAX_STR: &str = "Max String";
    const MAX_ARR: [i32; 4] = [1, 2, 3, 4];
    println!("{}, {}, {:?}", MAX_FLOAT, MAX_STR, MAX_ARR);
}

fn sub_fn() {
    println!("{}", MAX);
}
