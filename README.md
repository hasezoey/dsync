# dsync

**This is a fork of [Wulf's dsync](https://github.com/Wulf/dsync) with some Pull Requests merged, see [Difference with original](#difference-with-original)**

<a href="https://crates.io/crates/dsync"><img src="https://img.shields.io/crates/v/dsync.svg?style=for-the-badge" height="20" alt="License: MIT OR Apache-2.0" /></a>

A utility to generate database structs and querying code from diesel schema files. Primarily built for [create-rust-app](https://github.com/Wulf/create-rust-app).

Currently, it's more advantageous to generate code over deriving code with macros because intellisense and autocompletion isn't quite there when it comes to macro expansion.

## Demo

Given the following schema:

```rust
// schema.rs
diesel::table! {
    todos (id) {
        id -> Int4,
        text -> Text,
        completed -> Bool,
    }
}
```

We run:

```sh
cargo dsync -i schema.rs -o models
```

Now we have everything we need!

```rust
use models::todos;

async fn demo(db: Connection) {
  let created_todo = todos::create(&mut db, todos::CreateTodo {
    text: "Create a demo",
    completed: false,
  }).await?;
  
  let todos_list = todos::paginate(&mut db, 1, 10).await?;
  
  let updated_todo = todos::update(&mut db, created_todo.id, UpdateTodo {
    text: created_todo.text,
    completed: true,
  }).await?;
}
```

For more examples, look into the [`test/`](test/) folder, where in `test.sh` the options used are listed, `schema.rs` is the diesel schema and all other files are output from dsync

## Library

1. Add this crate:

    ```sh
    cargo add libdsync-hasezoey
    ```

2. Create a new binary in your project which uses the crate (for example, `bin/dsync.rs`)

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

3. Create a `Cargo.toml` binary entry:

   ```toml
   [[bin]]
   name = "dsync"
   path = "bin/dsync.rs"
   ```

4. Execute!

  ```sh
  cargo run --bin dsync
  ```

  **Protip**: to use `cargo dsync`, create an alias in `.cargo/config`:
  
  ```toml
  [alias]
  dsync="run --bin dsync"
  ```

## Binary

Setting up a custom binary allows you to completely customize the generation; however, if complete customization isn't necessary, you can install the CLI directly
(you'll have to make sure you keep it up-to-date by running this periodically):

```sh
cargo install dsync-hasezoey
```

### CLI Usage

* `-i`: input argument: path to schema file
* `-o`: output argument: path to directory where generated code should be written
* `-c`: connection type (for example: `diesel::r2d2::PooledConnection<diesel::r2d2::ConnectionManager<diesel::PgConnection>>`)  
* `-g`: (optional, repeatable) list of columns that are automatically generated by create/update triggers (for example, `created_at`, `updated_at`)
* `--tsync`: (optional) adds `#[tsync]` attribute to generated structs (see <https://github.com/Wulf/tsync>)
* `--no-serde`: (optional) if set, does not output any serde related code
* `--model-path`: (optional) set a custom model import path, default `crate::models::`
* `--schema-path`: (optional) set a custom schema import path, default `crate::schema::`
* `--only-necessary-derives`: (optional) set to only generate the base necessary diesel derives for a struct
* `--once-common-structs`: (optional) if set, common structs like `PaginationResult` are only generated once
* `--single-model-file`: (optional) if set, only generate a single `table.rs` file instead of a `table/` directory with `mod.rs` and `generated.rs`
* `--file-mode`: (optional, default `overwrite`) set which file mode to use
  * `overwrite`: Overwrite original if exists and has dsync file signature
  * `newfile`: If changes to the original file would be done, create a `.dsyncnew.rs` file instead
  * `none`: Do not change the file and do not create a new file (still lists the file if changes would be done)
* `--read-only-prefix`: (optional, repeatable): table prefixes to treat as read-only tables (like `view_`)
* `--no-impls`: (optional) set to disable generating `impl Struct` (only generate the structs)

Notes:

* any other `file-mode` than `newfile` will check that the file is a dsync-managed file
* if `--once-common-structs` is used, then when a table named `common` is found, a error it thrown
* if `--no-impls` is used without `--once-common-structs`, no `PaginationResult` struct is generated
* if `--no-impls` and `--once-common-structs` are used, `PaginationResult` is generated into `common.rs`

`./test/readme_cli_base_example`:

```sh
$ dsync -i src/schema.rs -o src/models -c "diesel::SqliteConnection"
Modified models/todos/generated.rs
Modified models/todos/mod.rs
Modified models/mod.rs
Modified 3 files

$ find . -xdev -type f
./src/models/mod.rs
./src/models/todos/mod.rs
./src/models/todos/generated.rs
./src/schema.rs
```

`./test/readme_cli_advanced_example`:

```sh
$ dsync -i src/schema.rs -o src/models -c "diesel::SqliteConnection" --no-serde --only-necessary-derives --once-common-structs --single-model-file -g updated_at -g created_at
Modified models/todos.rs
Modified models/common.rs
Modified models/mod.rs
Modified 3 files

$ find . -xdev -type f
./src/models/mod.rs
./src/models/todos.rs
./src/models/common.rs
./src/schema.rs
```

## Docs

See `dsync --help` for more information.

Feel free to open tickets for support or feature requests.

## Development/Testing

Use `./test/test_all.sh` to run tests.
After running the test, there should be no unexpected changes to files in `./test` (use `git status` and `git diff` to see if there were any changes).

## License

This tool is distributed under the terms of both the MIT license and the Apache License (Version 2.0).

See LICENSE-APACHE, LICENSE-MIT, and COPYRIGHT for details.

## Difference with original

This Fork was made because the [original project](https://github.com/Wulf/dsync) was not updated anymore since March 2023 and had become incompatible with diesel 2.1.0.

See [CHANGELOG.md](./CHANGELOG.md) for all changes since the start of the fork.

## FAQ

### sqlite `get_result` `DoesNotSupportReturningClause`

If you get a error like the following, then the fix is to add feature `returning_clauses_for_sqlite_3_35` to the diesel dependency

```txt
75   |             .get_result(db)
     |              ---------- ^^ the trait `QueryFragment<Sqlite, DoesNotSupportReturningClause>` is not implemented for `ReturningClause<(sql_schema::media_types::columns::_id, sql_schema::media_types::columns::name)>`
     |              |
     |              required by a bound introduced by this call
```
