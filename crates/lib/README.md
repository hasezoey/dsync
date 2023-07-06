# libdsync-hasezoey

**This is a fork of [Wulf's dsync](https://github.com/Wulf/dsync) with some Pull Requests merged, see [Difference with original](https://github.com/hasezoey/dsync#difference-with-original)**

For a binary, use [dsync-hasezoey crates.io](https://crates.io/crates/dsync-hasezoey) or [dsync-hasezoey github](https://github.com/hasezoey/dsync)

Usage:

```rust
use std::{collections::HashMap, path::PathBuf};
use dsync_hasezoey::{GenerationConfig, TableOptions};

pub fn main() {
    let dir = env!("CARGO_MANIFEST_DIR");

    dsync_hasezoey::generate_files(
        PathBuf::from_iter([dir, "src/schema.rs"]), 
        PathBuf::from_iter([dir, "src/models"]), 
        GenerationConfig { /* ... your generation options ... */ }
    );
}
```
