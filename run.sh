#!/usr/bin/env sh

export NOTES_STORAGE_DIRECTORY=$(pwd)/example-repo

cargo run "$@"
