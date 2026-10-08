#!/bin/sh
set -eu

exec cargo +stable \
    --config 'unstable.build-std=[]' \
    run \
    --target x86_64-unknown-linux-gnu \
    --no-default-features \
    --features simulator \
    --bin fish_pond_sim \
    "$@"
