#!/bin/bash

cargo fmt --all
leptosfmt .
cargo clippy -- -D warnings
