#!/bin/bash

cargo clean
cargo build --release

mkdir -p ~/bin/udata
cp udata/*                     ~/bin/udata
cp target/release/iana_info    ~/bin/ii
cp target/release/unicode_info ~/bin/ui

cargo clean

ii --subtag =mo -t language
ui 0312
