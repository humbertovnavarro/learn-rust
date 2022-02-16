pub fn run() {
  let mut count = 0;
  loop {
    count += 1;
    println!("Number: {}", count);
    if count == 20 {
      break;
    }
  }
  count = 0;
  // While Loop (FizzBuzz)
  while count <= 100 {
    let div_a = count % 3 == 0;
    let div_b = count % 5 == 0;
    if div_a && div_b {
      println!("fizzbuzz");
    } else if div_a {
      println!("fizz");
    } else if div_b {
      println!("buzz");
    } else {
      println!("{}", count);
    }
    count += 1;
  }
}
