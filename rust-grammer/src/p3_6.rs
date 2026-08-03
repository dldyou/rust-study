pub fn run() {
    // ======================================== if ========================================

    // if-else 문
    let n = 12;
    if n > 10 {
        println!("larger than 10");
    } else if n > 5 {
        println!("larger than 5");
    } else {
        println!("less than 5");
    }

    // match
    let x = 1;
    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"), // default
    }

    // ======================================== match ========================================

    // range match
    let age: u32 = 30;
    let group = match age {
        0..=10 => "baby",
        11..=20 => "teen",
        21..=60 => "adult",
        _ => "old",
    }; // let group = ...; 형태의 statement라서 ; 있어야 함
    println!("age: {}, group: {}", age, group);

    let c = '5';
    let num = match c.to_digit(10) {
        Some(n) => n,
        None => 0,
    };
    println!("num: {}", num);

    /*
    // if let 표현식: match 패턴의 간략 표현식

    if let Some(num) = c.to_digit(10) {
        println!("num={}", num); // num=5
    }
    */

    // if let에서 else도 사용 가능
    if let Some(num) = c.to_digit(10) {
        println!("num={}", num);
    } else {
        println!("errno");
    }

    let a = 33;
    match (a % 3, a % 5) {
        (0, 0) => println!("3과 5의 배수"),
        (0, _) => println!("3의 배수"),
        (_, 0) => println!("5의 배수"),
        (_, _) => println!("3의 배수도 5의 배수도 아님"),
    }

    // ======================================== for ========================================

    let mut sum = 0;
    for i in 1..=100 {
        // = 1..101
        sum += i;
    }
    println!("sum = {}", sum);

    let b = [2, 4, 10, 60, 61];
    sum = 0;

    for i in b {
        if i > 50 {
            break;
        }
        sum += i;
    }
    println!("sum = {}", sum);

    let v = vec![1, 2, 3, 4, 6];
    for val in v.iter() {
        print!("{} ", val); // 1 2 3 4 6
    }
    println!("");

    for val in &v {
        print!("{} ", val); // 1 2 3 4 6
    }
    println!("");

    // ======================================== loop ========================================

    let mut i = 1;
    loop {
        if i > 10 {
            break;
        }
        println!("{}", i);
        i += 1;
    }

    let p = max_factor(10);
    println!("max_factor = {}", p);

    // ======================================== while ========================================

    sum = 0;
    i = 1;

    while i <= 100 {
        sum += i;
        i += i;
    }
    println!("sum: {}", sum);
}

fn max_factor(mut n: u64) -> u64 {
    let mut p = 2;
    loop {
        let (q, r) = (n / p, n % p);

        if q == 1 {
            break;
        }

        if r == 0 {
            n = q;
        } else {
            p += 1;
        }
    }
    return n;
}

#[test]
fn test7() {
    let mut v: Vec<(i32, i32)> = Vec::new();
    // label break
    'label_i: for i in 2..=10 {
        for j in 2..=10 {
            if j >= 5 {
                break 'label_i;
            }
            v.push((i, j));
        }
    }
    println!("{:?}", v); //[(2, 2), (2, 3), (2, 4)]
}
