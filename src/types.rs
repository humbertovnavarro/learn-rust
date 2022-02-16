//mod print;
/*
  Rust has tuples
*/
pub fn run() {
  // Default is i32
  let x = 1;
  // Default is f64
  let y = 2.5;
  // Explicit type
  let z: i64 = 23199000000000;
  println!("Max i32: {}", std::i32::MAX);
  println!("Max i64: {}", std::i64::MAX);

  // boolean
  let is_active = true;

  let is_greater  = 10 > 5;
  let a1 = 'a';
  let face = '😀';
  println!("{:?}", (x,y,z, is_active, is_greater, a1, face));
}
