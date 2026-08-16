/*
?Sized의 사용 예

struct Foo<T>(T);
struct Bar<T: ?Sized>(T);

?Sized의 의미는 '꼭 Sized 트레잇을 구현한 타입이 아니어도 된다'의 의미다.

?Sized 사용이유

Rust는 모든 변수에 대해서 컴파일 타임에 그 크기를 한정할 수 있어야 한다.
정확하게는 스택에 할당되는 메모리 크기를 알 수 있어야 한다. 이게 어떤 의미인지를 문자열을 가지고 설명을 해 보겠다.

Rust에서 더블 따옴표로 되어 있는 문자열은 &str 타입이다. str 타입이 아니다.
예를 들어 아래와 같은 변수 선언은 불가능하다.

let s1: str = "Hello there!";
let s2: str = "How's it going?";

Rust 컴파일러가 위 코드에 대해 컴파일 에러를 내기에 사용할 수 없다.
그런데, 왜 Rust는 문자열에 대해 str로 타입지정을 하지 않은 것일까?

이유는, 위와 같은 코드를 허용하게 되면 변수 s1과 s2에 할당된 크기가 달라지게 되고, Rust가 이러한 것을 용납하지 않기로 했기 때문이다.
Rust는 같은 타입의 변수에 대해서는 스택에 할당된 크기가 동일하길 원한다.

그래서, 더블 따옴표로 쓰는 문자열의 경우 실제 문자열 값은 프로그램 내 읽기 전용 영역에 저장되고, 그 영역을 가리키는 주소만 일정한 크기로 스택에 저장되는 것이다.
즉, s1과 s2는 스택에 동일한 크기를 차지하며 존재하기 된다. 그래서 str이 아니고 &str 타입이 된 것이다.

이처럼 Rust에서는 선언되는 변수의 크기를 컴파일 타임에 알 수 있어야 한다.
그리고, 각 타입마다 그 크기를 알 수 있다면 Sized 트레잇을 구현하게끔 해놨다.
따라서, 해당 객체의 타입이 Sized를 구현한 객체이면 컴파일 타임에 사이즈를 알 수 있는 객체다.

Rust에서는 제네릭 타입이 선언될 때, 디폴트로 Sized를 트레잇 바운드를 한다.
즉, Sized를 구현한 타입만 제네릭 타입이 되게 하는 것이다. 예를 들어 어떤 함수 test를 아래와 같이 선언한다면,

fn test<T>(t: T) {
    ...
}

실제로는 아래와 같이 작성된 것으로 Rust 컴파일러가 취급한다.

fn test<T: Sized>(t: T) {
    ...
}

이렇게 하는 이유는, 제네릭 함수에 대해서 컴파일 타임에 스택에 할당할 공간을 알아내기 위해서다.
위의 경우는 test함수의 매개변수 t가 제네릭 타입 T이고, 이 T타입이 스택에 할당될 공간을 알아낼 수 있음을 보장받기 위해서,
Sized 트레잇이 구현된 타입만 매개변수로 받겠다는 것이다.

그러나, 이러한 조건을 회피해야할 경우가 있다. 이때 사용하는 키워드가 ?Sized다. Sized 앞에 ?를 붙인 것인데, 트레잇 바운드 Sized 조건을 해제하는 키워드다.
이처럼 트레잇 앞에 ?를 붙여서 트레잇 바운드 조건을 회피하는 키워드는 Sized 트레잇에만 유일하게 쓸 수 있다.
즉, ?Sized가 유일한 트레잇 바운드 회피 키워드다.

?Sized 사용 방법

아래는 test 함수에 대해서 ?Sized를 사용한 예다. 여기서 매개변수 t의 타입이 &T가 되었음에 유의한다.
fn test<T: ?Sized>(t: &T) {
    // --snip--
}

다시 얘기하면, Rust는 컴파일 타임에 모든 변수가 차지할 스택 공간을 알아내야 한다.
그런데, 위와 같이 제네릭 타입에 대해 ?Sized를 사용하면, 크기를 한정할 수 없는 타입이 매개변수로 사용될 수 있음을 얘기한다.
그러면 컴파일러가 매개변수가 차지할 스택 공간을 알아낼 수 없다. 그래서 참조자 형태로 받으라고 강제하는 것이다.
참조자 형태가 되면, 참조하는 주소에 대한 스택 공간만 이 함수에 배정하기 때문이다.
따라서, 매개변수 t의 타입을 &T로 바꿔야 하는 것이다.

?Sized를 사용해야 하는 케이스

그러면, 어떤 경우에 ?Sized 키워드를 사용할 필요가 있을까?

아래 예를 보자.
제네릭 타입 T를 사용하는 Foo라는 구조체를 선언했고, 이 Foo 구조체를 원소로 갖는 또 다른 구조체 FooUse1과 FooUse2가 선언되었다.

pub fn main() {
    struct Foo<T>(T);
    struct FooUse1(Foo<i32>); // ok
    struct FooUse2(Foo<[i32]>); // error
}

여기서 FooUse2에서 에러가 발생한다.

the size for values of type `[i32]` cannot be known at compilation time

i32 타입을 원소로 갖는 배열 [i32]는 Sized를 구현하지 않는 객체다.
배열이기에 당연히 그 크기를 한정할 수 없기에 Sized 구현을 하지 않는 것이 맞다.
그리고 Rust의 제네릭 사용에서의 규칙에 의해 struct Foo<T>(T);는 실제로는 struct Foo<T: Sized>(T);로
트레잇 바운드가 되어 있기에, 이 T에 대한 타입으로 Sized가 구현되지 않은 [i32]가 사용되었기에 에러가 발생하는 것이다.

pub fn main() {
    struct Foo<T: ?Sized>(T);
    struct FooUse1(Foo<i32>); // ok
    struct FooUse2(Foo<[i32]>); // ok
}

Deref에 대한 이해

Deref는 레퍼런스가 가리키는 곳의 값을 가져오는, 레퍼런스화 되어 있는 것을 원래의 값으로 되돌리는 그러한 트레잇이다.
이 트레잇을 구현한 구조체 등이 그렇게 된다는 것이다.

예를 들어, let x = 5; let y = &x; 라고 되어 있을 때 *y를 하는 것이 역참조(De-reference)이고,
이렇게 하는 트레잇이 Deref다. 여기서 *y를 하게 되면 *y=*(&x)가 되서, 끝내는 &y=5가 된다.

이처럼 Deref는 De-referencing(역참조)될 때의 행동을 정의하는 것으로만 생각하면 어려울게 없다.
그런데 Rust에서는 어떤 경우에 '강제 역참조'가 되는 경우가 있고, 이 Deref 트레잇에 있는 deref 함수가 자동 수행되서 이해하기 어렵게 만든다.

"역참조"의 일반적인 동작

어떤 변수 x에 대해 *x에 대한 연산은 다음과 같이 정의 된다.

*Dref::deref(&x)

이 연산을 두 개로 분리해서 차례대로 수행해보면
- (1) Dref::deref(&x)
Dref 트레잇에 정의되는 dref 메서드가 호출된다. Deref트레잇의 정의는 다음과 같다.

pub trait Deref {
    type Target: ?Sized;

    fn deref(&self) -> &Self::Target;
}

즉, &Self::Target 형태가 리턴된다.

만약 x = String::from("abc")와 같이 String 형태라고 해보자. String에 대한 Deref에 대한 구현은 아래와 같이 되어 있다.
따라서, Deref::deref(&x)는 &str 타입을 리턴한다.

impl ops::Deref for String {
    type Target = str;

    #[inline]
    fn deref(&self) -> &str {
        self.as_str()
    }
}

- (2) *연산

이제 위에서 수행한 Dref::deref(&x)에 대해 *연산을 수행한다.
위에 예를 들었던 String 타입의 x에 대해 예를 들면, Dref::deref(&x)에 의해 &str 타입이 리턴되었고, 이것에 *연산을 가하면 str 형태가 리턴되게 된다.
즉, x = String::from("abc")에 대해서 *x를 하게 되면 str타입이 리턴되게 된다.

따라서, 실제 코딩을 함에 있어서, y = *x와 같이 String 타입 변수 x를 역참조해서 어떤 변수에 할당할 수가 없다.
Rust에서 str은 &str에 대한 역참조 형태로서 이론적으로 존재할 뿐이지, 어떤 변수를 str 타입으로 지정할 순 없다.

따라서 아래 코드는 에러가 뜬다.
let x: String = String::from("abc");
let y = *y; // error

스마트 포인터에 대한 Deref

스마트 포인터는 '실제 데이터는 힙에 있고, 그것을 가리키는 포인터는 스택에 존재하는' 그러한 포인터를 의미하고,
이러한 것으로 String, Vec, Box 같은 것들이 있다.
이러한 것들에 대한 객체를 생성한 후, 해당 객체를 가리키는 변수에 대해 *을 붙이면, 이 객체의 데이터가 나와야 한다.
그것이 우리가 원하는 것이다. 그런데, 스마트 포인터라는 것이 실제 데이터는 힙에 있고, 변수 정보만 스택에 있기에,
변수에다가 *을 붙였다고 해서 해당 변수가 위치한 스택 메모리의 값을 리턴해 봐야 원하는 데이터가 나오지 않는다.
힙 데이터가 나와야 한다. 그래서 스마트 포인터에 대해서는 Deref 트레잇을 구현하게 해서,
*이 붙여 졌을 때 실제 힙 데이터가 나오도록 별도 조치를 하는 것이다.

암묵적 "강제 역참조" : 함수/메서드의 파라미터

Rust에서는 함수나 메서드의 파라미터로 어떤 객체가 전달될 때,
메서드가 정의한 파라미터 타입과 맞지 않는 경우 그대로 에러를 내지 않고,
넘겨진 객체에 대해 "강제 역참조"를 한 번 시도해서 파라미터 타입과 맞는지 확인해 본다.

fn hello(name: &str) {
    println!("Hello, {}!", name);
}

fn main(){
    hello("Jeff");

    let s = String::from("Tom");
    hello(&s);  //success
}

Rust 컴파일러는 어떤 객체의 메서드 호출에 대해, 호출된 메서드가 해당 객체에 없는 경우 두 가지를 시도해 본다.
1) 해당 객체에 대해 참조자를 붙인 후 해당 메서드가 있는지 찾아본다.
2) 참조자에다가 다시 역참조자를 붙여서 메서드가 있는지 찾아본다.
struct MyStruct {
    value: i32,
}

impl MyStruct {
    // This method takes a shared reference to MyStruct
    fn print_value(&self) {
        println!("Value: {}", self.value);
    }

    // This method takes a mutable reference to MyStruct
    fn increment_value(&mut self) {
        self.value += 1;
    }
}

fn main() {
    let mut my_instance = MyStruct { value: 10 };

    (&my_instance).print_value();

    // Calling print_value: Rust implicitly takes a shared reference (&my_instance)
    my_instance.print_value();

    // Calling increment_value: Rust implicitly takes a mutable reference (&mut my_instance)
    my_instance.increment_value();
    my_instance.print_value();
}

위 코드를 수행하면 잘 수행된다.


`MyStruct`에는 `print_value`와 `increment_value` 메서드가 정의되어 있는데, 모두 참조자 &를 사용하도록 정의되어 있다.
즉, `(&my_instance).print_value()`와 같이 호출하도록 되어 있다.

그러나, `my_instance.print_value()`와 같이 호출해도 문제없이 동작한다.
MyStruct 구조체에는 `fn print_value(self)`와 같은 메서드가 정의되어 있지 않아서, 이와 같은 호출이 문제가 될 것 같지만,
Rust는 `&my_instance.print_value()`와 같이 `&`를 붙여서 시도해보기에, 문제없이 동작한다.

tipRust에서는 왜 이렇게 만든 것일까?
어떤 객체에 대한 메서드 호출이라는 것은, 그 객체에 대해 호출하는 것이나, 그것의 참조자 형태로 호출하는 것이나 똑 같은 결과가 나와야하기 때문일 것이다.
그렇기에, Rust에서 구조체내의 메서드를 호출할 때는, 메서드의 파라미터가 self인지 &self인지 고민할 필요없다.
메서드의 파라미터에 &self로 되어 있어도 my_instance.amethod()처럼 호출이 가능하고,
메서드의 파라미터로 self가 되어 있어도 my_instance.amehod() 및 &my_instance.amethod() 둘 다 사용 가능하기 때문이다.

두 번째 강제 조건은 조금 헷갈릴 것이다.

2)참조자에다가 다시 역참조자를 붙여서 메서드가 있는지 찾아본다.

이것은 `역참조자`를 붙였기에 `Deref` 트레잇을 어떻게 구현했는지에 따라 어디서 메서드를 찾을지가 결정되기에,
코드를 유연하게 짤 수 있다. (이걸 다른 말로 하면 코드 이해가 어렵게 된다는 의미도 된다.)

아래 예를 가지고 설명해 보자.

먼저 Session이라는 구조체를 만들고, 이 구조체용으로 `find`라는 메서드를 만들자.

struct Session { }

impl Session {
    fn find(&self) -> String{
        "this is find method".to_owned()
    }
}

이제 이 Session을 변수로 가지는 Driver라는 구조체를 만들자.
Driver용 메서드는 없고, 이 Driver 구조체에 대해 Deref 트레잇만 구현하도록 하자.
다만, Deref 트레잇을 구현할 때 type Target=Session이 되도록 하고 deref의 리턴값으로 Session이 리턴되도록 하자.
이 부분이 핵심 코드다. 역참조를 하게되면 Session 객체가 리턴되도록 하는 것이다.

struct Driver{
    handle: Session,
}

impl std::ops::Deref for Driver {
    type Target = Session;
    fn deref(&self) -> &Self::Target{
        &self.handle
    }
}

이제 main 함수에서 테스트를 해보겠다. Driver 객체를 만들고,
이 객체에서 Session에서만 정의되어 있는 메서드를 호출해 보는 것이다.

fn main() {
    let driver = Driver {
        handle: Session {},
    };
    let txt = driver.find();
    println!("{}",txt);
}
위 코드에서 Session에만 정의되어 있는 find()메서드를 Driver 객체가 호출했는데도 문제없이 수행된다.

이유는 무엇일까? 위 쪽에서 얘기했던 '참조-역참조 강제'가 이루어졌기 때문이다.

driver.find()에 대해 Rust 컴파일러가 어떻게 동작하는지 단계별로 추측해보자.

1. driver의 타입인 Driver에 find() 메서드가 있는지 찾아본다. 없다.
2. driver에 대해 '참조자'를 붙여본다. &driver가되고 &Driver 타입이다.
이 &Driver 타입에 find() 메서드가 있는지 찾아본다. 없다.
3, driver에 대해 참조-역참조 강제를 해본다. *&driver가 된다.
이것은 Deref::deref(&driver)가 되고 이것의 리턴값은 driver.handle이 된다.
즉, driver가 가지고 있는 Session 타입인 handle 객체에 대한 find를 찾게된다. 존재한다. 따라서, 이 메서드가 호출된다.

쉽게 얘기하면,

어떤 구조체 A에 대해서, 그 구조체 A가 가지고 있는 어떤 타입 B가 가지고 있는 메서드도 호출할 수 있게 하려면,
구조체 A에 대해 Deref를 구현하면서 deref 메서드에서 B의 객체가 리턴되도록 하면 된다.

타입 별칭 (Type Alias)

타입 별칭은 "어떤 타입에 대해 별도의 이름을 부여하는 것"을 말한다. type 키워드를 사용해서 이루어진다.
type <새로운 타입 이름> =

타입 별칭을 사용하는 것은, 복잡한 타입 이름을 간단한 타입명으로 재정의해서 코드를 간단히 하고자 할 때이다.

use std::collections::HashMap;
fn process_data(data: HashMap<String, Vec<i32>>) {
    // ...
}

fn analyze_data(data: HashMap<String, Vec<i32>>) {
    // ...
}

위에서 HashMap<String, Vec<i32>> 이란 타입이 여러 군데 사용된다.
이것을 별도의 타입으로 명명하면 코드가 좀 더 간단해질 수 있다.

use std::collections::HashMap;

type DataMap = HashMap<String, Vec<i32>>;

fn process_data(data: DataMap) {
    // ...
}

fn analyze_data(data: DataMap) {
    // ...
}

좀 더 복잡한 예시는 아래와 같이
pub type AppCreator<'app> = Box<dyn FnOnce(&CreationContext<'_>) -> Result<Box<dyn App + 'app>,
Box<dyn Error + Send + Sync>> + 'app>;

pub fn run_native(
    app_name: &str,
    native_options: NativeOptions,
    app_creator: AppCreator<'_>,
) -> Result {
...
}

*/

pub fn run() {}
