fn main() {
  println!("{} days", 31);
  println!("{:?} days", 31);
  // println!("{} is invalid UnPrintable", UnPrintable(1));
  // println!("{:?} is UnPrintable", UnPrintable(2));
  // println!("{} is invalid Printable", Printable(3));
  println!("{:?} is Printable", Printable(4));
  println!("{:#?} is Printable", Printable(4));
  // println!("{} is invalid Deep", Deep(Printable(5)));
  println!("{:?} is Deep", (Deep(Printable(6))));
  println!("{:#?} is Deep", (Deep(Printable(6))));
}

struct UnPrintable (i32);

#[derive(Debug)]
struct Printable (i32);

#[derive(Debug)]
struct Deep (Printable);
