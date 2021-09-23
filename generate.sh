#!/bin/sh -e
# Run the generator for the given OpenAPI spec files.
#
# Docs: https://openapi-generator.tech/docs/generators/rust

# Post-process script: Translate "1.*.*" → "0.*.*" for the Rust crate version.
sed_crate_version='
/version = / s/"1[.]/"0./
'
sed_readme_version='
/Package version: / s/: 1[.]/: 0./
'

# Post-process script: Trim the default Markdown documentation links from the generated README.
sed_readme_trim_docs="
/^## Documentation for API Endpoints$/,/^To get access to the crate's generated documentation, use:$/ c \
## Documentation\n\
\n\
To get access to the crate's generated documentation, use:
"

for spec_file in "$@"; do
    # Derive the crate name:
    # 1. Get the spec's base name. (Strip the file path and extension.)
    spec="$(basename "$spec_file" .yaml)"
    # 2. Replace `_` with `-`, and change the prefix to `twilio-oai-`
    name="$(echo "$spec" | sed 's/_/-/g; s/^twilio-/twilio-oai-/')"

    # Make sure the output directory is clean
    if test -d "${name}"; then rm -r "${name}"; fi

    ./openapi-generator-cli generate \
        --generator-name rust \
        --input-spec "$spec_file" \
        --ignore-file-override '.openapi-generator-ignore' \
        --library 'reqwest' \
        --additional-properties 'supportMultipleResponses=true' \
        --additional-properties 'useSingleRequestParameter=true' \
        --package-name "${name}" \
        --output "${name}"

    # Post-process:
    sed --in-place "$sed_crate_version" "${name}/Cargo.toml"
    sed --in-place "$sed_readme_version" "${name}/README.md"
    sed --in-place "$sed_readme_trim_docs" "${name}/README.md"
done
