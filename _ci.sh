#!/usr/bin/env sh

set -x
set -e

export CI=true
export RUST_BACKTRACE=1
export NOTES_STORAGE_DIRECTORY="$(pwd)/example-repo"
export EDITOR="$(pwd)/tests/assets/fake_editor.sh"

# Print version info for debugging
rustc --version && cargo --version

cargo clean
cargo fmt                                     # Format code
cargo fmt -- --check                          # Fail if formatting is not correct
cargo clippy -- -D warnings                   # Lint
cargo clippy --tests -- -D warnings           # Lint tests code
cargo tarpaulin --verbose --out Html          # Test and coverage
