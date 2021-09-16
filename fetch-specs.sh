#!/bin/sh -ex
# Fetch specs for twilio-oai $version
#
# Usage: version=… ./fetch-specs.sh
#
# Specs: https://github.com/twilio/twilio-oai

: "${version:?}"

if test -d specs; then rm -r specs; fi
mkdir specs

curl \
    --fail \
    --location \
    "https://github.com/twilio/twilio-oai/archive/refs/tags/${version}.tar.gz" \
    |
tar -xz \
    --directory 'specs' \
    --strip-components=3 \
    "twilio-oai-${version}/spec/yaml"
