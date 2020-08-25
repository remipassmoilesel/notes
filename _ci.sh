#!/usr/bin/env sh

set -x
set -e

export RUST_BACKTRACE=1
export NOTES_STORAGE_DIRECTORY="$(pwd)/example-repo"

cd example-repo && git init && cd ..

# Print version info for debugging
rustc --version && cargo --version

cargo fmt
cargo fmt -- --check
cargo tarpaulin --verbose
