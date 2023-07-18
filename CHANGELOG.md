# Changelog

This Changelog only lists the changes done to this fork and since 0.0.13

## next

- fix that a file is created if the file does not exist on `MarkedFile::new`
- fix so that all entries in `common.rs` are properly new-lined
- add option `read-only-prefix` to set prefixes for tables to treat as read-only (like `view_`)
- add option `no-impls` to not generate any `impl Struct`s (like `read`, `create`, `paginate`)
- add option `once-connection` to only output the connection type once in `common.rs` instead of in all files
- add option `lessen-conflicts` to lessen conflict with diesel types (like having to import `Connection`)
- add option `create-str` to have `Create*` structs be generated with `&'a str` instead of `String`

## 0.2.0

- add option `only-necessary-derives` to only generate the necessary diesel derives for a struct
- add option `file-mode` to set a different mode than direct `overwrite`
- add option `once-common-structs` to only generate common structs once
- replace dependency `inflector` with `heck` (`heck` is used by `diesel` and `clap`)
  - remove converting struct names with `to_singular`
- simplify implementation for how struct derives are generated (order of derives has changed)
- always end model files with a empty new line
- have less unnecessary new-lines (order of imports has changed)
- add rust doc-comments to all generated functions
- add rust doc-comments to all generated structs (not their fields)
- add rust doc-comments to all generated fields listing their column name
- add display on what happened to files (unchanged, modified, overwritten, deleted)

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
