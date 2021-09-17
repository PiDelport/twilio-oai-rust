#!/bin/sh -ex

rm -rf rust-fixed
mkdir -p rust-fixed/reqwest

for base in README reqwest/api; do
    curl \
        --fail \
        --location \
        --output "rust-fixed/${base}.mustache" \
        --remote-time \
        "https://github.com/PiDelport/openapi-generator/raw/fix-rust-client/modules/openapi-generator/src/main/resources/rust/${base}.mustache"
done
