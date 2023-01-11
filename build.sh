#!/bin/sh -e

cargo build
cargo build --target aarch64-unknown-uefi
