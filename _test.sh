#!/usr/bin/env sh

set -x
set -e

export CI=true
export RUST_BACKTRACE=1
export NOTES_STORAGE_DIRECTORY="$(pwd)/example-repo"
export EDITOR="$(pwd)/tests/assets/fake_editor.sh"

cargo fmt                               # Format code
cargo fmt -- --check                    # Fail if formatting is not correct
cargo test $@ -- --nocapture            # Test
