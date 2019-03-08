# ms

A Rust version of [Tiny milisecond conversion utility](https://github.com/zeit/ms)

# Install

```
Cargo.toml

[dependencies]
ms = "0.1.0"
```

# Usage

```rust
extern crate ms;
use ms::*;

fn main() {
    ms!("2 days");  // 172800000
    ms!("1d");      // 86400000
    ms!("10h");     // 36000000
    ms!("2.5 hrs"); // 9000000
    ms!("2h");      // 7200000
    ms!("1m");      // 60000
    ms!("5s");      // 5000
    ms!("1y");      // 31557600000
    ms!("100");     // 100

    // convert in milliseconds to string in short type
    ms!(60000, false);        // "1m"
    ms!(2*60000, false);      // "2m"
    ms!(ms!("10 hours"), false); // "10h"

    // convert in milliseconds to string in long type
    ms!(60000, true);         // "1 minute"
    ms!(2*60000, true);       // "2 minutes"
    ms!(ms!("10 hours"), true);  // "10 hours"
}

```

# Test
```bash
$ cargo test
```