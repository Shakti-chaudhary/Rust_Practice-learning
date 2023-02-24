use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;

// ========================= 1.rs ==========================
// fn main() {
//     let iterations = 10;
//     let a = thread::spawn(move || {
//         for i in 1..=iterations {
//             println!("A: {}", i);
//         }
//     });
//     let b = thread::spawn(move || {
//         for i in 1..=iterations {
//             println!("B: {}", i);
//         }
//     });
//     a.join();
//     b.join();
// }
// ====================== 2.rs ===========================

// fn main() {
//     let value: JoinHandle<usize> = thread::spawn(move || {
//         thread::sleep(Duration::from_secs(2));
//         42
//     });
//     println!("Waiting on thread");
//     match value.join() {
//         Ok(n) => println!("value: {:?}", n),
//         Err(e) => println!("error joining thread: {:?}", e),
//     }
// }
// ====================== 2.rs ===========================

fn main() {
    let data = vec!['a', 'b', 'c'];
    let caps = thread::spawn(move || {
        let data: Vec<char> = data.iter().map(|c| c.to_ascii_uppercase()).collect();
        return data;
    });
    println!("Waiting on value..");
    match caps.join() {
        Ok(n) => println!("value: {:?}", n),
        Err(e) => println!("error joining thread: {:?}", e),
    }
}
