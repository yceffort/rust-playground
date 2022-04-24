fn main() {
    println!("Hello, Rust!");

    let name = "yceffort";
    // cannot assign twice to immutable variable
    // name = "power overwhelming";

    //  할당안하고 나중에 할당해서 가져다 쓰는 건 괜찮음.
    let name2;
    name2 = "hello!";
    println!("{}", name2);

    // mut으로 mutable한 값으로 사용하는 건됨
    let mut name3 = "yceffort";
    println!("{}", name3);
    name3 = "power overwhelming";
    println!("{}", name3);

    // const 는 자바스크립트와 다르게 컴파일 시점에 정해지는 값을 의미한다'.
    // 컴파일 시점에 값이 결정되어야 하므로, 타입이 제공되어야 한다.'
    //  provide a type for the constant: `hello_world: bool`
    // const hello_world = true;
    // println!("{}", hello_world);

    const hello_world: bool = true;
    println!("{}", hello_world);
}
