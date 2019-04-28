extern crate enum_tag_lib;
use enum_tag_lib::*;
  
#[derive(Tag)]
enum Foo {
  ONE(String),
  TWO { i: i32, },
  THREE,
}


fn main() {
  let foo_0 = Foo::ONE(String::from("hi"));
  let foo_1 = Foo::ONE(String::from("hi"));
  let foo_2 = Foo::ONE(String::from("hi"));
  println!("foo_0.tag_count(): {}", foo_0.tag_count());
  println!("foo_0.tag(): {}", foo_0.tag());
  println!("foo_1.tag(): {}", foo_1.tag());
  println!("foo_2.tag(): {}", foo_2.tag());
}
