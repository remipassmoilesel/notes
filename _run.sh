#!/usr/bin/env sh

export RUST_BACKTRACE=1
export NOTES_STORAGE_DIRECTORY=$(pwd)/example-repo

cargo run "$@"
