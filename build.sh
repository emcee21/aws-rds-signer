#!/bin/bash

set -e

cargo fmt --all -- --check
cargo clippy -- -D warnings
cargo build --verbose
