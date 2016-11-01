extern crate ansi_escapes;

use std::thread::sleep;
use std::time::Duration;

fn main() {
    // Hides the cursor
    print!("{}", ansi_escapes::CursorHide);

    // Prints first message
    println!("Hello, World!");

    // Waits one seconds
    sleep(Duration::from_secs(1));

    // Erases the two lines
    print!("{}", ansi_escapes::EraseLines(2));

    // Print final message
    println!("Hello, Terminal!");

    // Shows the cursor
    print!("{}", ansi_escapes::CursorShow);
}
