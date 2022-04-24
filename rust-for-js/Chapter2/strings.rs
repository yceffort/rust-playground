// 러스트의 string은 자바스크립트와 많이 다르다.
// 자바스크립트는 모두 힙에 존재하고 있어 문자열이건 배열이건 자유롭게 길이를 조절할 수 있지만, 러스트는 그렇지 못하다.

// str: 문자열 슬라이스, 배열과 유사하게 고정된 길이로 불변임
// String: u8의 Vec으로 볼 수 있으며, UTF-8을 가지고 있고, 고정된 길이가 아니다.

fn main() {
    // 이것이 string slice다.
    // &은 일단 무시
    let greeting: &str = "Hello, World!";
    println!("{}", greeting);

    // 이것이 String이다.
    let mut name = String::new();
    name.push_str("yceffort");
    println!("{}", name);

    // String::from 도 가능하다.
    let mut name = String::from("yceffort");
    println!("{}", name);

    let first_name = String::from("yceffort");
    let last_name = String::from("kim");

    // 여기서 주목해야 할 것은, string 두개를 바로 합칠수는 없다는 것이다.
    // String에 string slice를 더했다.
    // string을 더하기 위해서는 최초의 string에 나머지는 string slice를 활용해야 한다.
    // & 을 제대로 공부하기 위해서는 소유권에 대해 이해해야 한다ㅣ.
    let name = first_name + " " + &last_name;
    println!("{}", name);

    // push, push_str, split, split_whitespace 등 다양한 것이 있다.
    let parts: Vec<&str> = name.split_whitespace().collect();

    //["yceffort", "kim"]
    println!("{:?}", parts)
}
