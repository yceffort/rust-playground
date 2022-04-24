fn main() {
  // 불리언
  let boolean: bool = true;
  // character
  let character: char = 'S';
  // 부호있는 숫자 (8, 16, 32, 64, 128)
  let int: i32 = -30;
  // 부호없는 숫자 (8, 16, 32, 64, 128)
  let number: u64 = 30;
  // float, (32, 64)
  let double: f32 = 1.5;

  // 대부분 머신이 64비트를 지원하니까, 64를 쓰면 별일은 없을듯 
  let price = 129;
  let tax = 23.22;
  // :: 은 자바스크립트의 . 과 비슷하다고 보면됨.
  let total = f64::from(price) + tax;
  println!("{}", total);
  println!("Total: {} + {} = {}", price, tax, total);

  // 자바스크립트랑 비슷하게 이런 것도 됨.
  let person = "Charlie";
  println!("Hello, {person}!");
}