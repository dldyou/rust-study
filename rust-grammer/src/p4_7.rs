use std::fs::File;
use std::io::{Error, Read};
pub fn run() {
    // 파일 오픈 함수인 File::open 함수가 리턴하는 Result를 처리하는 예
    // Err일 때 프로그램 종료하는 방식
    let file = File::open("src\\p4_7.rs"); // 파일 없는 것으로 변경 후 테스트

    let file = match file {
        Ok(f) => f,
        Err(e) => {
            panic!("file open error: {}", e);
        }
    };
    println!("{:?}", file);

    // Err일 때 메시지만 출력
    let file = File::open("src\\p4_7.rs");

    let file = match file {
        Ok(f) => f,
        Err(e) => {
            println!("Error: {}", e);
            return;
        }
    };
    println!("{:?}", file);

    // Option에 대한 처리
    let num_str = "010-1234-5678";

    let mut v: Vec<u32> = Vec::new();
    for c in num_str.chars() {
        match c.to_digit(10) {
            Some(n) => v.push(n),
            None => (),
        }
    }
    println!("v={:?}", v);

    /*
    대규모의 프로그램을 짤 때는, 에러를 어떻게 처리할 것인지에 대한 전략을 세우고 짜야한다.

    대부분의 경우 다음의 전략이 유효하다.
    - 작성되는 모든 함수 혹은 메서드는 에러가 발생할 가능성이 있거나, 값이 없을 수 있다면 무조건 Result 혹은 option을 리턴 타입으로 해서 작성한다.
    - 하위 함수의 리턴값이 Result일 때, Result::Err에 대한 처리는 상위 함수로 위힘한다.
    즉, 현재의 함수가 실제 화면 출력 혹은 GUI를 담당하는 함수가 아니라면, 더 상위의 함수로 에러 처리를 위임한다.
    - 최상위 함수 혹은 화면 출력을 하는 함수에서 밑에서부터 올라온 에러를 처리한다.
    즉, 에러에 대해서 출력만 하던지 패닉 처리를 하던지 하는 것을 결정한다.

    위와 같이 에러의 처리를 최상위 함수로 하는 것은, 에러 처리에 대한 전략 관리를 수월하게 하기 위해서다.
    이렇게 하지 않고, 개별 함수에서 에러가 발생할 때마다 에러 메시지를 출력하거나 패닉을 일으키거나 하게되면,
    나중에 에러 처리에 대한 정책이 바뀌거나 하면 그 부분을 찾아서 수정하기가 어렵기 때문이다.

    에러를 상위로 전파하는 키워드는 ?다. 아래 예제는 ?를 이용해 에러를 상위로 위임하는 예제다.
    */

    let result = match read_name() {
        Ok(n) => n,
        Err(e) => {
            println!("{:?}", e);
            String::from(" ")
        }
    };
    println!("result = {}", result);

    /*
    일반적으로 어떤 함수를 만들 때 입력값이 얼마일 때 overflow가 발생하는지를 예측하기 힘든 경우가 많기에,
    이처럼 입력값을 제한하는 것이 어려울 수 있다.

    이처럼 overflow가 발생할 때, 이 overflow가 발생하지 않게하는 것이 가장 좋겠지만, 만약 발생한다면 어떻게 처리할 지를 지정할 수 있으면 좋겠다.
    Rust에서는 이러한 방법을 제공한다. 크게 2가지인데, 위 예제에서와 같이 Panic을 발생시키는 것과, Wrapping을 하는 것이다.

    1. Panic: overflow가 발생했을 때 panic에 빠지게한다. 즉, 프로그램을 강제 종료 시킨다.
    2. Wrapping: 최댓값을 넘어가면, 최댓값을 뺀 나머지 값이 되게 한다.

    overflow-check

    Rust는 dev 모드에서는 overflow가 발생하면 panic이 발생하는 것이 디폴트로 되어 있다. 그러나, 만든 프로그램을 배포하기 위해
    Release 모드로 컴파일 할 때는 overflow가 발생하면 wrapping이 되게 되어 있다.

    Release 버전에 대해서도 overflow일 때 wrapping이 아닌 panic이 발생하게하려면, Cargo.toml 파일에서 아래와 같은 설정을 추가해줘야한다.

    [profile.release]
    overflow-checks = true

    -> 이렇게 하면 기존 2의 보수값 오버플로우 처리로 바뀜
    */
    println!("3! = {}", factorial(3));
    println!("12! = {}", factorial(12));
    println!("13! = {}", factorial(13)); // panic
}

/*
(1) read_name 함수의 리턴 타입을 Result로 했다.
(2) File::open()?은 성공하면 파일 핸들러가 Ok(f)로 리턴되고, 실패해서 Err가 나오면 이 Err값을 리턴값으로 해서 read_name 함수를 종료시킨다.
또한, 붙여서 .read_to_string(&mut name)? 메서드를 호출할 수 있는데, 여기도 ?가 붙어 있어서, 만약 read_to_string 메서드에서 Err가 리턴되면
이것을 리턴값으로 해서 read_name 함수를 종료시킨다.
*/
fn read_name() -> Result<String, Error> {
    // (1)
    let mut name = String::new();
    // (2)
    File::open("src/p4_6.rs")?.read_to_string(&mut name)?;
    Ok(name)
}

fn factorial(n: i32) -> i32 {
    let mut ans = 1;
    for i in 2..=n {
        ans *= i;
    }
    return ans;
}
