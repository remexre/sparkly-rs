#!/bin/bash

set -eu

cargo doc --all
cargo test --all

if [[ "$TRAVIS_RUST_VERSION" = "nightly" ]]; then
    for bench in `basename -s .rs benches/*`; do
        cargo test --bench $bench
	done
    cargo bench
fi
