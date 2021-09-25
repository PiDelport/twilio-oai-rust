# Twilio OpenAPI clients for Rust

Rust client libraries generated from Twilio's OpenAPI specs at [twilio-oai], using [openapi-generator].

## Usage

Reference the crates for the APIs you need from `Cargo.toml` using `git`:

```toml
[dependencies]
twilio-oai-api-v2010 = { git = "https://github.com/PiDelport/twilio-oai-rust" }
```

Optionally, specify `branch`, or `tag` for a specific release.

## Version policy

The crate versions follow the upstream [twilio-oai] versions, but with a major version of `0` (instead of `1`).

## Spec updates

To update this repository for new upstream spec releases:

1. Fetch the latest `twilio-oai` specs:

   ```shell
   ./fetch-specs-latest.sh
   ```

2. Regenerate the client crates:

   ```shell
   ./generate-all.sh
   ```

[twilio-oai]: https://github.com/twilio/twilio-oai
[openapi-generator]: https://github.com/OpenAPITools/openapi-generator
