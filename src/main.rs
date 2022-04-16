fn main() {
  fizz_buzz_to(30);
}

fn fizz_buzz_to(n: u32) {
  for i in 1..=n {
    fizz_buzz(i);
  }
}

fn fizz_buzz(n: u32) -> () {
  if is_dividable_by(n, 15) {
    println!("FizzBuzz");
  } else if is_dividable_by(n, 5) {
    println!("Buzz");
  } else if is_dividable_by(n, 3) {
    println!("Fizz");
  } else {
    println!("{}", n);
  }
}

fn is_dividable_by(lhs: u32, rhs: u32) -> bool {
  if rhs == 0 {
    return false;
  }

  lhs % rhs == 0
}
