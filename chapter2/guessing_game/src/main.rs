// 라이브러리 불러오기
use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("숫자를 맞혀봅시다!");    

    let secret_number = rand::thread_rng().gen_range(0..=100);

    println!("사용자가 맞혀야할 숫자 {}", secret_number);

    loop {
        println!("정답이라고 생각하는 숫자를 적어주세요.");

        // 변수 선언. 
        // mut이 있다는 것은 가변 변수를 의미
        // utf-8 형식의 문자열 선언
        let mut guess = String::new();

        // io::stdin().read_line 는 문자열 전달
        // & 는 참조 타입을 전달한다는 것 (메모리에[ 복사할 필요 없이 접근])
        // 변경 가능한 참조라 &mut
        io::stdin().read_line(&mut guess)
            // Result를 반환함 (enum)
            // Err 발생시 expect를 실행하고 종료
            // 없어도 실행은 됨 경고만 날뿐
            .expect("입력한 값을 읽지 못했습니다.");

        // 변수 가리기
        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("입력한 값! {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("입력한 숫자가 작습니다."),
            Ordering::Greater => println!("입력한 숫자가 큽니다."),
            Ordering::Equal => {
                println!("정답!");
                break;
            }
        }
    }
}
