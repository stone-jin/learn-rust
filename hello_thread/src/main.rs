use std::time::Duration;
use std::thread;

fn main() {
  let v1 = vec![1,2,3];
  let jh = std::thread::spawn(move || {
     for i in 1..10 {
            println!("hi number {} from the spawned thread! {:?}", i, v1);
            thread::sleep(Duration::from_millis(1));
        }
  });
  println!("before main thread.");
  jh.join().unwrap();
  println!("after main thread.");
}
