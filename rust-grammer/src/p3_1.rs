pub fn run() {
    println!("hello"); // hello

    let sum = 100;
    println!("a + b = {}", sum); // a + b = 100
    println!("a + b = {sum}"); // a + b = 100

    let v = vec![1, 2, 3]; // vec!도 매크로
    println!("{:?}", v); // [1, 2, 3]
}
