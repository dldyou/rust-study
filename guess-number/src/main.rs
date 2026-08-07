use std::io::Write;

use rand::RngExt;

fn game() {
    let mut is_gameover = false;
    let mut rng = rand::rng();
    let guess_number = rng.random_range(1..101);

    let stdin = std::io::stdin();
    let mut stdout = std::io::stdout();
    let mut buffer = String::new();

    println!("1-100 사이의 숫자를 맞춰보세요!");
    while !is_gameover {
        print!("사용자 입력: ");
        stdout.flush().expect("flush error");
        stdin.read_line(&mut buffer).expect("input error");

        let input_number: i32 = buffer.trim().parse().expect("invalid number");
        if input_number == guess_number {
            is_gameover = true;
            println!("{} 입니다!", guess_number);
        } else if input_number < guess_number {
            println!("{} 보다 큽니다.", input_number);
        } else {
            println!("{} 보다 작습니다.", input_number);
        }
        buffer.clear();
    }
}

fn main() {
    game();
}
