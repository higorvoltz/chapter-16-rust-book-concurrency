use std::thread;
use std::time::Duration;

fn main() {
  // Creating a New Thread with spawn
  thread::spawn(move || {
    for i in 1..10 {
      println!("hi number {} from the spawned thread!", i);
      thread::sleep(Duration::from_millis(1));
    }
  });

  for i in 1..5 {
    println!("hi number {} from the main thread!", i);
    thread::sleep(Duration::from_millis(1));
  }

  // hi number 1 from the main thread!
  // hi number 1 from the spawned thread!
  // hi number 2 from the main thread!
  // hi number 2 from the spawned thread!
  // hi number 3 from the main thread!
  // hi number 3 from the spawned thread!
  // hi number 4 from the main thread!
  // hi number 4 from the spawned thread!
  let handle = thread::spawn(|| {
    for i in 10..15 {
        println!("hi number {} from the spawned thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
});

for i in 10..15 {
    println!("hi number {} from the main thread!", i);
    thread::sleep(Duration::from_millis(1));
}

handle.join().unwrap();
// hi number 1 from the main thread!
// hi number 1 from the spawned thread!
// hi number 2 from the main thread!
// hi number 2 from the spawned thread!
// hi number 3 from the main thread!
// hi number 3 from the spawned thread!
// hi number 4 from the main thread!
// hi number 4 from the spawned thread!
// hi number 5 from the spawned thread!
// hi number 10 from the main thread!
// hi number 10 from the spawned thread!
// hi number 6 from the spawned thread!
// hi number 11 from the main thread!
// hi number 11 from the spawned thread!
// hi number 7 from the spawned thread!
// hi number 12 from the main thread!
// hi number 12 from the spawned thread!
// hi number 8 from the spawned thread!
// hi number 13 from the main thread!
// hi number 13 from the spawned thread!
// hi number 9 from the spawned thread!
// hi number 14 from the main thread!
// hi number 14 from the spawned thread!

println!();

let other_handle = thread::spawn(|| {
  for i in 1..10 {
      println!("hi number {} from the spawned thread! case 3", i);
      thread::sleep(Duration::from_millis(1));
  }
});

other_handle.join().unwrap();

for i in 1..5 {
  println!("hi number {} from the main thread! case 3", i);
  thread::sleep(Duration::from_millis(1));
}

// hi number 1 from the spawned thread! case 3
// hi number 2 from the spawned thread! case 3
// hi number 3 from the spawned thread! case 3
// hi number 4 from the spawned thread! case 3
// hi number 5 from the spawned thread! case 3
// hi number 6 from the spawned thread! case 3
// hi number 7 from the spawned thread! case 3
// hi number 8 from the spawned thread! case 3
// hi number 9 from the spawned thread! case 3
// hi number 1 from the main thread! case 3
// hi number 2 from the main thread! case 3
// hi number 3 from the main thread! case 3
// hi number 4 from the main thread! case 3

}