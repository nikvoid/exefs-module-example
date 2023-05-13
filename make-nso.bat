#!/bin/bash
cargo build --release
linkle nso target/aarch64-nintendo-switch-freestanding/release/exefs-module-example target/aarch64-nintendo-switch-freestanding/release/subsdk6