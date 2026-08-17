use std::io::{self, BufRead, Read};

pub fn run(args: &[String]) {
    let args_config = ArgsConfig::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        std::process::exit(1);
    });

    let source = args_config.src_dir;
    let destination = args_config.tgt_dir;

    println!("source: {}", source);
    println!("destination: {}", destination);

    /*
    모든 콘솔 입력은, 콘솔 입력용 핸들러를 얻고 난 후 처리 가능하고,
    콘솔 입력용 핸들러는 std::io::stdin()으로 얻을 수 있다.
    즉, Rust에서 콘솔 입력 처리는 std::io::stdin()을 통해 핸들러를 얻은 후, 이 핸들러를 통해 처리된다.

    이후 이 핸들러를 그냥 사용할 것인지, lock을 걸어서 사용할 것인지를 결정해야 한다.
    여러 쓰레드가 경합하는 프로그램에서는 lock을 걸어서 사용하는 것이 좋다.
    그렇지 않으면 다른 쓰레드에 의해 인터럽트 당할 수 있다.
    */

    // 단일 쓰레드
    // 한 줄을 String으로 받기
    let stdin = std::io::stdin();
    let mut buf = String::new();
    stdin.read_line(&mut buf).expect("input error");

    println!("buf: {}", &buf);
    // 한 줄에 있는 숫자값 n, k를 읽는 경우
    // 알고리즘 문제에서 주어지는 수자를 읽어야 하는 경우가 있다.
    // 한 줄에 공백으로 분리된 정수값을 읽을 때는 아래와 같이 하면 된다.
    // 한 줄을 읽어서 String으로 저장하고, 이 String에서 공백을 기준으로 문자열을 분리한 후 숫자로 변환하는 것이다.

    let stdin = std::io::stdin();
    let mut buf = String::new();
    stdin.read_line(&mut buf).expect("input error");

    let input: Vec<String> = buf.split(' ').map(|s| s.to_string()).collect();
    let n = input[0].trim().parse::<usize>().unwrap();
    let k = input[1].trim().parse::<usize>().unwrap();

    println!("n = {}", n);
    println!("k = {}", k);

    // 멀티 쓰레드
    let mut buf = String::new();
    let len = std::io::stdin()
        .lock()
        .read_line(&mut buf)
        .expect("reading console input error");
    // carage return으로 길이는 5가 됨. buf.trim()하면 지워짐. &str 리턴된 걸 다시 String으로 변환하려면 .to_owned()
    println!("buf = {}, len = {}", buf, len);

    // 여러 줄 읽기
    // lines() 메서드는 콘솔 입력되는 문자열에 대한 반복자를 리턴한다. [io::Result]<String> (개행문자 포함 X)
    let lines = std::io::stdin().lock().lines();
    for line in lines {
        let s = line.unwrap();
        if s.eq("xxx") {
            break;
        }
        println!("input: {}", s);
    }

    // 중요! 빠르게 여러 줄을 읽어야 할 때
    // lines()가 리턴하는 것은 Result<String>을 함유하고 있는 반복자라는 것
    // String은 힙 메모리를 사용하기에 for 루프를 통해 값을 읽을 때마다 힙 영역에서 메모리 할당과 해제 반복
    // 알고리즘 사이트 등에서는 문제가 됨 lines -> read_line으로 한 줄 씩 읽는 것이 낫다

    let mut sum = 0;
    let mut cnt = 0;

    let mut handle = std::io::stdin().lock();
    let mut buf = String::new();
    let mut eof = false;

    while !eof {
        match handle.read_line(&mut buf) {
            Ok(0) => {
                // read_line은 eof 만나면 Ok(0)을 리턴 (Ctrl-z)
                eof = true;
            }
            Ok(_) => {
                let n = buf.trim().parse::<i32>().expect("parsing error");
                buf.clear();
                sum += n;
                cnt += 1;
            }
            Err(error) => {
                panic!("something wrong: {}", error);
            }
        }
    }
    println!(
        "(cnt, sum, avg) = ({}, {}, {})",
        cnt,
        sum,
        sum as f32 / cnt as f32
    );

    // 한 번에 모든 줄에 있는 데이터 읽기
    /*
    lines() 메서드는 콘솔로 입력되는 값들에 대한 반복자를 리턴하고, 실제 읽기 작업은 반복자에 의해서 실제 액션이 일어날 때 발생
    따라서, 반복자에 의한 루프에서 라인 데이터가 읽혀져서 어떤 String에 저장되고 해제되는 작업이 루프의 한 스텝마다 발생하기에,
    힙 메모리 할당 / 해제가 빈번해서, 많은 줄이 있는 데이터에는 적합하지 않다.
    그래서 buf라는 String 하나를 생성 후 재활용해서 힙 메모리 할당 / 해제가 일어나지 않게 한 것이다.

    그런데, 입력되는 데이터 줄 수가 수십 ~ 수백 줄 정도 일 때는, 해당 데이터를 한 번에 String으로 읽어내는 것이 훨씬 효과적일 수 있다.
    한 번에 모든 데이터를 읽어서 힙 메모리에 저장해두고 사용하는 것이다.
    */
    let mut buf = String::new();
    std::io::stdin()
        .lock()
        .read_to_string(&mut buf)
        .expect("input error");

    let mut grid: Vec<Vec<i32>> = Vec::new();
    for line in buf.lines() {
        let v: Vec<i32> = line
            .trim()
            .split(' ')
            .filter_map(|w| w.parse().ok())
            .collect();
        grid.push(v);
    }
    println!("{:?}", grid);

    // 출력 처리하기
}

struct ArgsConfig {
    src_dir: String,
    tgt_dir: String,
}

/*
(1) build의 파라미터에 self가 없다. 즉, 메서드가 아니라 함수다.
따라서 ArgsConfig::build(...)와 같은 형태로 실행될 수 있다.
그리고, build 함수의 파라미터 &[String] 타입의 값을 받게 되어 있다.
즉, String을 담고 있는 배열 혹은 벡터를 파라미터로 받을 수 있는 것이고,
&가 있기에 객체의 주소를 받는 참조형이어서 소유권이 이동하지는 않는다.

(2) 리턴 타입이 Result다. Result 열거형은 Ok와 Err을 베리언트로 갖는다.
Ok에 실제 리턴되는 값이 들어가게 되는데, 여기서는 ArgsConfig 구조체 형태의 값이 들어가게 되어 있다.
Err에 들어가는 타입은 문자열이다.

(3) args[0]은 프로그램 이름이 되고, args[1]이 src_dir 이름이, args[2]에는 tgt_dir 이름이 들어가게 된다.
따라서, args에는 최소한 3개의 값이 있어야 한다. 만약 3보다 작은 크기면 에러를 리턴하게 한다.

(4) src_dir을 나타내는 문자열에는 더블 따옴표가 들어갈 수 있다.
만약 그렇다면, 폴더 이름만을 사용해야 하기에 더블 따옴표를 없애야 한다.
replace 메서드를 이용해서 없앨 수 있다.

(5) Ok에 ArgsConfig 구조체를 담아서 리턴한다.
*/
impl ArgsConfig {
    // (1)
    fn build(args: &[String]) -> Result<ArgsConfig, &'static str> {
        // (2)
        if args.len() < 3 {
            // (3)
            return Err("not enough arguments. Use as follows: \n
            file_backup src_dir_name tgt_dir_name\n
            ex1) file_backup D:/tmp2 D:/tmp3\n
            ex2) file_backup \"D:/a folder\" \"D:/another folder\"\n");
        }
        let src_dir = args[2].replace("\"", ""); // (4)
        let tgt_dir = args[3].replace("\"", "");

        Ok(ArgsConfig { src_dir, tgt_dir }) // (5)
    }
}
