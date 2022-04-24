fn main() {
    // 일반적인 if else 문. 괄호가 없다는 것을 제외하면 자바스크립트와 매우 유사
    let year = 2192;

    // 자바스크립트 처럼 truthy, falsy 라는 개념이 없으므로 항상 bool 한 값으로 해야함.
    if year >= 1946 && year < 1965 {
        println!("Hello, Boomers!");
    } else if year >= 1965 && year < 1981 {
        println!("Hello, Generation X!");
    } else if year >= 1981 && year < 1997 {
        println!("Hello, Millennials!");
    } else if year >= 1997 && year < 2012 {
        println!("Hello, Generation Z!");
    } else {
        println!("Hello, Unknown Generation!");
    }

    // 자바스크립트는 할 수 없는 이런 것도 가능.
    let year = 1991;

    // if - else 는 러스트에서 표현식으로 구별됨.
    let generation = if year >= 1946 && year < 1965 {
        "Boomer"
    } else if year >= 1965 && year < 1981 {
        "Generation X"
    } else if year >= 1981 && year < 1997 {
        "Millennial"
    } else if year >= 1997 && year < 2012 {
        "Generation Z"
    } else {
        "Unknown Generation"
    };

    println!("Hello, {}", generation);

    // 한무 루프 돔
    // loop는 while과 동일하다고 보면 됨
    // loop {
    //   println!("infinite!")
    // }

    let mut counter = 0;

    loop {
        counter = counter + 1;
        println!("loop {counter}");
        if counter == 10 {
            break;
        }
    }
}
