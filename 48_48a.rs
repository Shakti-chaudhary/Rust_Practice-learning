use std::thread;

fn msg_hello() -> &'static str {
    use std::time::Duration;
    std::thread::sleep(Duration::from_millis(1000));
    "Hello, "
}

fn msg_thread() -> &'static str {
    use std::time::Duration;
    std::thread::sleep(Duration::from_millis(1000));
    "threads"
}

fn msg_excited() -> &'static str {
    use std::time::Duration;
    std::thread::sleep(Duration::from_millis(1000));
    "!"
}

fn main() {
    let hello = thread::spawn(move || msg_hello());
    let thread = thread::spawn(move || msg_thread());
    let excit = thread::spawn(move || msg_excited());

    let msg_one = hello.join().expect("failed to join msg one");
    let msg_two = thread.join().expect("failed to join msg one");
    let msg_three = excit.join().expect("failed to join msg one");

    println!("{}{}{}", msg_one, msg_two, msg_three);
}
