use std::{thread, time::Duration};
use ctrlc;

fn main() {
    ctrlc::set_handler(move || {
        println!("receive Ctrl+C!");
    })
    .expect("Error setting Ctrl-C handler");

    // Following code does the actual work, and can be interrupted by pressing
    // Ctrl-C. As an exaple: Let's wait a few seconds.
    thread::sleep(Duration::from_secs(20));
    println!("Hello, world!");
}
