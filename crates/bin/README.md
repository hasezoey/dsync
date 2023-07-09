# libdsync-hasezoey

**This is a fork of [Wulf's dsync](https://github.com/Wulf/dsync) with some Pull Requests merged, see [Difference with original](https://github.com/hasezoey/dsync#difference-with-original)**

For a library, use [libdsync-hasezoey crates.io](https://crates.io/crates/libdsync-hasezoey) or [libdsync-hasezoey github](https://github.com/hasezoey/dsync#library)

Usage:

```sh
dsync -i src/schema.rs -o src/models -c "diesel::SqliteConnection"
```

All cli options can be found [here](https://github.com/hasezoey/dsync#binary) or via `dsync --help`.
