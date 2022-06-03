fn main() {
    // 자바스크립트 배열과 매우 비슷함.__rust_force_expr!
    // 한가지 차이점은 배열 내 요소의 모든 타입이 같아야 한다는 것
    let sample_array = ["hello", "my name", "is", "yceffort"];

    // ^ expected `&str`, found integer
    // let invalid_array = ["hello", "my name", "is", "yceffort", 5];
    // 자바스크립트에 있는 빈 공간도 존재해서는 안됨.

    // 한가지 특이한 사실은 배열을 한번 선언하고 나면 요소를 더하거나 빼는 것이 불가능 하다는 것.
    // 필요하다면 Vec을 사용해야 함.

    // 그러므로 배열 선언시 미리 타입과 크기를 타입으로 선언하여 집어넣을 수 있음.
    let top_scores: [u32; 3] = [100, 200, 300];

    // mut을 사용하면 내부 값을 바꾸는 정도는 가능함.
    // 0으로 가득찬 3개짜리 배열이라는 뜻
    let mut mut_top_scores: [u32; 3] = [0; 3];
    mut_top_scores[0] = 100;
    mut_top_scores[1] = 200;
    mut_top_scores[2] = 300;
    mut_top_scores[2] = 400;

    println!("{}", mut_top_scores.len());

    // 배열을 프린트 하려면 어떻게 해야할까?
    // cannot be formatted with the default formatter
    // println!("{}", mut_top_scores);

    // :? 는 Debug 트레잇으로, 기본 Display 대신 사용한다는 뜻이다.
    // 이를 사용하면 프린트 할 수는 있는데, 나중에 자세하게 확인.
    println!("{:?}", mut_top_scores);

    // Tuple
    // 고정된 길이를 가졌으며, array와 다르게 여러 타입을 가질 수 있음.
    let mut product = ("iphone", 150, true);

    // 이건 가능
    product = ("ps5", 200, false);

    // 이건 마지막에 타입이 달라지므로 불가능.
    // product = ("ps5", 200, 100);

    // 튜플과 배열모두 자바스크립트와 유사한 형식으로 구조분해 할당이 가능
    let [score, _, _] = mut_top_scores;
    let (_, _, available) = product;

    // 그리고 rest operator도 가능. . 개수가 다른게 함정

    let [scores, ..] = mut_top_scores;
}
