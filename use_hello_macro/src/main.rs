#[macro_use]
extern crate hello_macro_derive;

trait HelloMacro {
    fn hello_macro();
}

#[derive(HelloMacro)]
struct Pancakes;

#[napi]
fn hello(){
  println!("hello world");
}

fn main() {
    Pancakes::hello_macro();
    hello();
}