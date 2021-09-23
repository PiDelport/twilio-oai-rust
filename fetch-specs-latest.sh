#!/bin/sh -e

version="$(curl -sS https://api.github.com/repos/twilio/twilio-oai/releases/latest | jq -r .tag_name)" ./fetch-specs.sh
