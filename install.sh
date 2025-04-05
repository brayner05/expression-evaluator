#!/bin/env bash

set -e

error_message() {
    echo -e "\033[1;31mfailed\033[0m: $1"
}

if ! command -v "cargo" &> /dev/null; then
    error_message "This software requires an installation of Cargo."
    exit 1
fi

cargo build --release
cargo install --path .

echo -e "\033[1;32m PXPR [Expression Parser] has successfully been installed on your system! \033[0m"