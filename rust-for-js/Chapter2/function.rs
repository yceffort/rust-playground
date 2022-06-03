fn main() {
    println!("Hello, World!");
    // 리턴이 없는 함수는 tuple () 암묵적으로 리턴한다.

    let result = sum(10, 5);
    println!("Result: {}", result);

    // 러스트에도 익명함수가 존재한다. 람다함수 또는 클로져라고도 불리운다.
    // |x| {x}
    // 위 함수는 자바스크립트의 x => x 다.

    // 과거에는 타입 annotaion이 없어도 되었는데, 요즘은 필요한듯
    let anonymous_sum = |x: u64, y: u64| x + y;
    let result2 = anonymous_sum(10, 5);
    println!("Result: {}", result2);
}

// 아래와 같은 함수는 이제 더이상 컴파일 되지 않음
// 러스트에서 모든 함수의 인수와 리턴 값에는 타입이 달려 있어야 함
// fn sum(x, y) {
//   return x + y;
// }

// 이런 방식으로 선언해야 함.
fn sum(x: u64, y: u64) -> u64 {
    return x + y;
}

fn implicit_sum(x: u64, y: u64) -> u64 {
    // return 문이 없다면 맨 마지막 표현식을 리턴으로 간주한다.
    // ; 가 없는 이유는 표현식에 한해서 필요 없기 때문이다.
    // 있으면 에러남
    x + y
}
