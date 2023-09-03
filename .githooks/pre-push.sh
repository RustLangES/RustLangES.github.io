#!/bin/bash

cargo fmt --all -- --check
leptosfmt --check .
cargo clippy -- -D warnings
