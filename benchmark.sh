#!/bin/sh

RUSTFLAGS="-C target-cpu=native" cargo b --release

echo >> times.txt
for i in $(seq 10); do
    /bin/env time -o times.txt -a ./target/release/uniques-rs
done

