// Rust has two kinds of strings
// primitive string: Immutable, fixed length string.

// String: Growable, heap allocated - Use when you need to modify string


pub fn run() {
  let mut hello = String::from("Hello");
  let primitive_string = "ooga booga";
  // Get length (either string)
  println!("Length of hello: {}", hello.len());
  println!("Length of primitive_string: {}", primitive_string.len());
  println!("{}", hello);
  // Cap of String
  hello.push_str(" Humberto");
  println!("Capacity: {}", hello.capacity());
  println!("Contains hello: {}", hello.contains("Hello"));
  for word in hello.split_whitespace() {
    println!("word: {}", word);
  }
  let mut s = String::with_capacity(10);
  s.push('a');
  s.push('b');
  assert_eq!(2, s.len());
  assert_eq!(10, s.capacity());
  println!("{}", s);
}
