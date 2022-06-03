// enum 키워드로 enum을 만들 수 있음
// 타입스크립트와 다르게, 값이 키와 똑같음
#[derive(Debug)]
enum Colour {
    Red,
    Green,
    Blue,
    // 이런 짓도 가능함.
    // 즉 enum 내부에서 값들이 서로 다른 타입을 가지고 이써도 된다는 뜻
    Rgb(u8, u8, u8),
}

fn main() {
    // :: 을 사용해서 접금함
    let red = Colour::Red;
    // 앞서 설명던 것 처럼, {:?} 을 사용해서 접근함.
    // 원래 enum은 debg가 없는데, 상단에 트레잇을 선언해서 사용 가능해졌음
    println!("Apples are {:?}", red);

    let Purple = Colour::Rgb(150, 20, 250);
    println!("My favorite colour {:?}", Purple);
}
