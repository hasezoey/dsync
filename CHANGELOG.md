# Changelog

This Changelog only lists the changes done to this fork and since 0.0.13

## 0.1.0

The First version of the fork

- Support diesel `2.1.0` ([PR #51](https://github.com/Wulf/dsync/pull/51))
- Option to disable `serde` output ([PR #54](https://github.com/Wulf/dsync/pull/54))
- Option to set custom model and schema path ([PR #55](https://github.com/Wulf/dsync/pull/55))
- Some CI updates ([PR #50](https://github.com/Wulf/dsync/pull/50))
- Separate library and binary into their own crates
- Change from `structop` to use `clap`
  - Add cli completions thanks to `clap_completions`
- Enable `anyhow` backtrace support
- Add `Error` type for library (via `thiserror`)
  - Replace many `panic` and `expect` with a Error
  - Enable `backtrace` for Error
