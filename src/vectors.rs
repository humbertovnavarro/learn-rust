// Arrays fixed list with elements of same type
use std::mem;
pub fn run() {
  let mut numbers: Vec<i32> = vec![1,2,3,4,5];
// Re-assign value
  numbers[3] = 20;
  numbers.push(1000);
  numbers.pop();
  for x in numbers.iter_mut() {
    *x *= 2
  }
  println!("{:?}", numbers);
  println!("single value: {}", numbers[0]);

  // Arrays are stack allocated
  println!("vector occupies {} bytes", mem::size_of_val(&numbers));

  let slice: &[i32] = &numbers[0..2];
  println!("Slice: {:?}", slice);
}
