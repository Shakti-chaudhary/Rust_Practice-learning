// copy all the code to main.rs in src/  to use crate in programme

use humantime::format_duration;
use std::time::Duration;

fn main() {
    let d = Duration::from_secs(9876);
    println!("{}", format_duration(d));
}
