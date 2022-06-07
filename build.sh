#!/bin/bash

export RUSTUP_HOME=/usr/local/rust
export CARGO_HOME=/usr/local/rust

curl -fsSL https://sh.rustup.rs | sh -s -- -y

source /usr/local/rust/env
rustup default stable

for FILE in "$RUSTUP_HOME/bin/"*; do
    if [[ -f "/usr/local/bin/$(basename "$FILE")" ]]; then
        rm "/usr/local/bin/$(basename "$FILE")"
    fi
    if [[ -f "$FILE" ]]; then
        ln -sf "$FILE" "/usr/local/bin/$(basename "$FILE")"
    fi
done

rustup default stable
