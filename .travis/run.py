#!/usr/bin/env python3

import os
import subprocess

def cargo(*args):
    code = subprocess.call(["cargo"] + list(args))
    assert code == 0

cargo("doc", "--all")
cargo("test", "--all")

if os.getenv("TRAVIS_RUST_VERSION") == "nightly":
    for bench in os.listdir("benches"):
        cargo("test", "--bench", bench.replace(".rs", ""))
    cargo("bench")
