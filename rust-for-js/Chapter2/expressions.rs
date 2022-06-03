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
    // loop는 while (true) 와 동일하다고 보면 됨
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

    // 자세한 설명은 생략한다.
    let mut counter2 = 0;
    while counter2 <= 10 {
        counter2 = counter2 + 1;
        println!("while {counter}");
    }

    // 자바스크립트의 for in과 유사함.
    for planet in [
        "Mercury", "Venus", "Earth", "Mars", "Jupiter", "Saturn", "Uranus", "Neptune",
    ] {
        println!("{}", planet)
    }

    // 자바스크립트에 switch가 있다면, 러스트에는 패턴 매칭이 있다.
    // let status = Status::Connected;
    let status = Status::Fail(String::from("접속에 실패하였습니다."));

    // 반드시 모든 enum을 커버해야 한다.
    match status {
        Status::Connected => {
            println!("Connected!")
        }
        Status::DisConnected => {
            println!("Disconnected!")
        }
        Status::Fail(error) => {
            println!("Error: {}", error)
        }
    }

    // 만약에 하나에 대해서만 구현하고싶다면 이렇게 하면 된다.
    // match status {
    //   Status::Fail(error) => {
    //     println!("Error: {}", error);
    //   }
    //   _ => {}
    // }
    // 위와 동일
    // if let Status::Fail(error) = status {
    //   println!("Error: {}", error)
    // }
}

enum Status {
    Connected,
    DisConnected,
    // 만약 여기에 새로운 값이 추가되면 match에서 에러가 발생한다.
    Fail(String),
}
