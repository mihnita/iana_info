#!/bin/bash

rm language-subtag-registry
wget https://www.iana.org/assignments/language-subtag-registry/language-subtag-registry

cargo clean
cargo build --release

mkdir ~/bin
mkdir ~/bin/udata
cp language-subtag-registry ~/bin/udata
cp target/release/iana_info ~/bin/ii

rm language-subtag-registry
cargo clean

ii --subtag =mo -t language
