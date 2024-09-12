---
title: 2. Hello Tokio
---

# 2. Hello Tokio

Let's update the generated "Hello, World" application to use the `tokio` async runtime.

Replace the contents of `src/main.rs` with the following:

```rust
use std::io::stdout;
use std::io::Write as _;
use std::time::Duration;

use tokio::time::sleep;

#[tokio::main]
async fn main() {
    print!("Wait 3 seconds...");
    let _ = stdout().flush();

    sleep(Duration::from_secs(3)).await;
    println!("wait is over, goodbye!");
}
```

Run the application again using `cargo run`. Make sure the three seconds elapse between the two
print functions. The output should look something like this:

```sh
❯ cargo run
   Compiling api-server v0.1.0 (E:\Projects\rust-seminar\api-server)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.44s
     Running `E:\Projects\rust-seminar\target\debug\api-server.exe`
Wait 3 seconds...wait is over, goodbye!
```
