use std::io;

fn main() {
  let mut input = String::new();
  io::stdin()
    .read_line(&mut input)
    .expect("Failed to read line.");

  let mut sum = 45;

  for ch in input.iter() {
    let num = ch as usize - '0' as usize;
    usd[num] = true;
  }
}
