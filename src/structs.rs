// Tuple struct
// struct Color(u8,u8,u8);
struct Person {
  first_name: String,
  last_name: String
}
impl Person {
  // Constructor
  fn new(first: &str, last: &str) -> Person {
    Person {
      first_name: first.to_string(),
      last_name: last.to_string()
    }
  }
  fn get_full_name(&self) -> String {
    format!("{} {}", self.first_name, self.last_name)
  }
  // Set last name
  fn set_last_name(&mut self, last: &str) {
    self.last_name = last.to_string();
  }
  fn name_to_tuple(self) -> (String, String) {
    (self.first_name, self.last_name)
  }
}
// struct Color {
//   red: u8,
//   green: u8,
//   blue: u8
// }
pub fn run() {
  // let mut red = Color {
  //   red: 255,
  //   green: 0,
  //   blue: 0
  // };
  // red.blue = 4;
  // println!("Color: {} {} {}", red.red, red.green, red.blue);
  // let mut c = Color(255,255,255);
  // c.0 = 1;
  // println!("Color: {} {} {}", c.0, c.1, c.2);
  let mut p = Person::new("John", "Doe");
  p.set_last_name("Johnson");
  // println!("Person {} {}", p.first_name, p.last_name);
  println!("Person: {}", p.get_full_name());
  println!("Person: {:?}", p.name_to_tuple());
}
