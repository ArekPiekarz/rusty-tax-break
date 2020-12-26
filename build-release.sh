#!/usr/bin/env bash
# Script for building Rusty Tax Break with options tuned for better runtime performance on stable Rust.
# It can increase the compilation time in comparison to default release profile.

RUSTFLAGS="-Clink-arg=-fuse-ld=lld -Ctarget-cpu=native" cargo build --release
