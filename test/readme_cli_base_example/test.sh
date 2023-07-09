#!/bin/bash

SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )"

cd $SCRIPT_DIR

cargo run -- -i src/schema.rs -o src/models -c "diesel::SqliteConnection"
