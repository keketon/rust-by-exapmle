#![allow(unused)]
/*
cannot compile

enum List {
  Cons(i32, List),
  Nil,
}
*/

#[derive(Debug)]
enum List {
  Cons(Rc<RefCell<i32>>, Rc<List>),
  Nil,
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
  fn new(x: T) -> MyBox<T> {
    MyBox(x)
  }
}

impl<T> Deref for MyBox<T> {
  type Target = T;
  fn deref(&self) -> &T {
    &self.0
  }
}

use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;
use List::{Cons, Nil};

fn main() {
  let value = Rc::new(RefCell::new(5));

  let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

  let b = Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));
  let c = Cons(Rc::new(RefCell::new(10)), Rc::clone(&a));

  let q = *value.borrow_mut() += 10;
  *value.borrow_mut() += 10;

  println!("{:?}", a);
  println!("{:?}", b);
  println!("{:?}", c);
}
