#!/usr/bin/env just --justfile

set windows-shell := ["powershell.exe", "-NoLogo", "-Command"]

release:
  cargo build --release    

lint:
  cargo clippy --all-targets --all-features

check-deadcode:
    rg '#\[allow\(dead_code\)\]'

check-todos:
    rg -i '// TODO:'