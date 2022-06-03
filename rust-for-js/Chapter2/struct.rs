// 디버그
#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

// 튜플 구조도 가능
#[derive(Debug)]
struct Celsius(i16);

fn main() {
    let me = Person {
        name: String::from("yceffort"),
        age: 34,
    };

    /*
    자사스크립트로 치면 이런 느낌
    let name = "yceffort"
    let age =30

    let me = {
      name,
      age,
    }
    */

    let name = String::from("yceffort");
    let age = 34;
    let me2 = Person { name, age };

    println!("{:?}", me);
    println!("{:?}", me2);

    // 이런 것도 가능
    let me_next_year = Person { age: 35, ..me };
    println!("{:?}", me_next_year);

    // 숫자의 용도를 알기에 편함.
    let boiling_point_of_water = Celsius(100);
    let melting_point_of_ice = Celsius(0);

    // 값에 접근할 때
    println!("{:?}", boiling_point_of_water.0);
    println!("{:?}", boiling_point_of_water);
}
