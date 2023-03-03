// use parking_lot::mutex;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

type SharedSignData = Arc<Mutex<String>>;

struct DigitalSignBoard {
    display: SharedSignData,
}
impl DigitalSignBoard {
    fn update(&self) {
        let data = self.display.lock().unwrap();
        println!("sign data='{}'", data);
    }
}

fn spawn_display_thread(display_data: SharedSignData) {
    thread::spawn(|| {
        let board = DigitalSignBoard {
            display: display_data,
        };
        loop {
            board.update();
            thread::sleep(Duration::from_millis(200));
        }
    });
}

fn change_data(display_data: SharedSignData, new_data: &str) {
    let mut data = display_data.lock().unwrap();
    *data = new_data.to_owned();
    println!("------updated: {}", new_data);
}

fn main() {
    let display_data = Arc::new(Mutex::new("initial".to_owned()));
    thread::sleep(Duration::from_millis(100));
    change_data(Arc::clone(&display_data), "message 1");

    thread::sleep(Duration::from_millis(600));
    change_data(Arc::clone(&display_data), "another message");
    thread::sleep(Duration::from_millis(600));
    change_data(Arc::clone(&display_data), "goodbay message");
}
