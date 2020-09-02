#!/usr/bin/env bash
set -x
set -e

./scripts/mksvd.py system/iobitmasks/*.h system/iodefines/*.h

rm -rf src
svd2rust --target none -i rza1.svd
form -i lib.rs -o src/ && rm lib.rs
cargo fmt
