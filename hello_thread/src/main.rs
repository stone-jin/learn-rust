use std::time::Duration;
use std::thread;
use std::sync::{mpsc, Mutex, Arc};

fn demo1(){
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

fn demo_mpsc(){
  let (tx, rx) = mpsc::channel();
  std::thread::spawn(move || {
    tx.send("hello mpsc").unwrap();
  });
  let result = rx.recv().unwrap();
  println!("result: {}", result);
}

fn demo_mpsc_mul_msg(){
  let (tx, rx) = mpsc::channel();
  std::thread::spawn(move || {
    let msgs = vec![
      String::from("msg1"),
      String::from("msg2"),
      String::from("msg3"),
    ];

    for msg in msgs{
      tx.send(msg).unwrap();
    }
  });
  for msg in rx{
    println!("rec: {}", msg);
  }
}

fn demo_mpsc_mul_tx(){
  let (tx, rx) = mpsc::channel();
  let tx1 = tx.clone();
  std::thread::spawn(move || {
    let msgs = vec![
      String::from("tx_msg1"),
      String::from("tx_msg2"),
      String::from("tx_msg3"),
    ];

    for msg in msgs{
      tx.send(msg).unwrap();
      thread::sleep(Duration::from_secs(1));
    }
  });

  std::thread::spawn(move || {
    let msgs = vec![
      String::from("tx1_msg1"),
      String::from("tx1_msg2"),
      String::from("tx1_msg3"),
    ];

    for msg in msgs{
      tx1.send(msg).unwrap();
      thread::sleep(Duration::from_secs(1));
    }
  });

  for msg in rx{
    println!("rec: {}", msg);
  }
} 

fn demo_arc_mux(){
  let counter = Arc::new(Mutex::new(0));
  let mut handlers = vec![];
  for _ in 0..10 {
    let counter = Arc::clone(&counter);
    let handler = std::thread::spawn(move || {
      let mut m = counter.lock().unwrap();
      *m += 1;
    });
    handlers.push(handler);
  }

  for handler in handlers{
    handler.join().unwrap();
  }
  println!("counter: {}", *counter.lock().unwrap());
}

fn main() {
  demo1();
  demo_mpsc();
  demo_mpsc_mul_msg();
  demo_mpsc_mul_tx();
  demo_arc_mux();
}
