
fn main() {
  let jh = std::thread::spawn(move || {
    println!("in thead ");
  });
  println!("before main thread.");
  jh.join().unwrap();
  println!("after main thread.");
}
