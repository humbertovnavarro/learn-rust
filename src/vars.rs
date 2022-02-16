/*
  Vars in rust are immutable by default. (cannot be reassigned)
  Rust is block scoped.
*/

pub fn run() {
  // name is immutable
  let name = "Humberto";
  // mut makes the var mutable
  let mut age = 23;
  println!("My name is {} and I am {}", name, age);
  age = 24;
  println!("My name is {} and I am {}", name, age);
  // Constants are a thing
  const ID: i32 = 1;

  // i32 = 32 bit signed integer 😎
  println!("ID: {}", ID);
  // Assign multiple vars
}
