# ANSI Escapes

[ANSI escape codes](http://www.termsys.demon.co.uk/vtansi.htm) for manipulating the terminal

## Usage

This example program will print "Hello, World!", then replace it with "Hello, Terminal!" after one second.

```rust
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
```

## API

See [documentation](https://docs.rs/ansi-escapes)
