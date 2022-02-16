pub fn run() {
  // Print to console
  // Placeholders
  println!("Hello, my name is {} and I like {}", "Humberto", "Rust");
  println!("{0} is {1} and {0} is {2}", "Humberto", "awesome", "cool");
  println!("{name} likes {food}", name = "Humberto", food = "apples");
  println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);
  // Placeholder for debug trait
  println!("{:?}", (12, true, "hello"));
  println!("10 + 10 = {}", 10 + 10);
}
