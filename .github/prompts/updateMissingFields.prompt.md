# Riven API Update Guide

This guide documents the process for updating the Riven library when Riot Games API documentation falls out of date and new undocumented fields appear in API responses.

## Overview

Riven's Rust data structures are generated from `riotapi-schema`, which generates an OpenAPI spec from online documentation. When the docs are outdated, we must add manual overrides to the schema.

## Prerequisites

- Both `Riven` and `riotapi-schema` repositories cloned locally
- Node.js installed for schema generation
- Rust toolchain for testing
- Valid Riot API key in `Riven/apikey.txt`

## Workflow

### 1. Identify Failing Tests

Run the test suite with **all features enabled** to see detailed deserialization errors:

```bash
cd /path/to/Riven
./test.bash
```

The test.bash script includes the critical features:
```bash
RGAPI_KEY="$(cat apikey.txt)" cargo test --no-fail-fast \
    --features riven/nightly,riven/tracing,riven/eserde,riven/deny-unknown
```

**Important**: The `deny-unknown` feature is essential - without it, unknown fields are silently ignored and you won't see what's missing!

Look for errors like:
- `unknown field 'fieldName' of type U64` - field exists but not in schema
- `missing field 'fieldName'` - field is in schema but always required, should be optional

### 2. Add Schema Overrides

There are THREE important files in `riotapi-schema/src/data/` for different purposes:

#### `dtoExtraFields.jsonc` - For adding new fields
Add fields that aren't in the official documentation:

```jsonc
"endpoint-version.DtoName": {
  "newFieldName": {
    "type": "integer",
    "format": "int32",
    "x-type": "int"
  }
}
```

#### `dtoOptional.jsonc` - For marking fields as optional
Add entries to make fields nullable:

```jsonc
"endpoint-version.DtoName.fieldName": true,
```

#### `schemaOverrides.jsonc` - For complete DTO overrides
This file is used when the entire DTO is missing from the docs and must be defined from scratch. Note this takes precedence over extraFields.

```jsonc
"endpoint-version.DtoName": {
  "type": "object",
  "title": "DtoName",
  "properties": {
    "field1": { "type": "string" },
    "field2": { "type": "integer", "format": "int64" }
  },
  "required": ["field1"]
}
```

**Best Practice**: When adding a field to `schemaOverrides.jsonc`, also add it to `dtoOptional.jsonc` as a safety measure in case the override is removed later.

### 3. Regenerate OpenAPI Spec

```bash
cd /path/to/riotapi-schema
node .
```

This generates the OpenAPI spec in `riotapi-schema/out/openapi-3.0.0.json`.

Verify your field was added:
```bash
grep -A 5 "yourFieldName" out/openapi-3.0.0.json
```

### 4. Regenerate Rust Code

To regenerate using the local riotapi-schema file:

```bash
cd /path/to/Riven
node riven/srcgen --spec=../../../riotapi-schema/out
```

The `--spec` argument should point to a schema source root (either a local `riotapi-schema/out` directory or a website base URL like `https://www.mingweisamuel.com/riotapi-schema`).

- Local source example: `--spec=../../../riotapi-schema/out`
- Website source example: `--spec=https://www.mingweisamuel.com/riotapi-schema`

Omit `--spec` to use the default published schema source.

This updates the Rust structs in `riven/src/models.rs`.

### 5. Test Changes

Run the specific failing test with features:

```bash
cd /path/to/Riven
RGAPI_KEY="$(cat apikey.txt)" cargo test --test tests_name \
    --features riven/nightly,riven/tracing,riven/eserde,riven/deny-unknown
```

### 5b. Optional: Use Local `eserde` Patch For Better Errors

When deserialization errors are hard to interpret, temporarily patch `eserde` to your local checkout for richer field-level diagnostics:

```toml
# Riven/Cargo.toml (temporary; do not commit unless intended)
[patch.crates-io]
eserde = { path = "../eserde/eserde" }
```

Then force lockfile resolution to the patched version:

```bash
cd /path/to/Riven
cargo update -p eserde --precise 0.1.7
```

Validation tip:
- Use `cargo tree -p eserde` to confirm Cargo is actually using the local patch.

Cleanup tip:
- Before final commits, restore `Cargo.toml` and `Cargo.lock` if the patch was only for debugging.

### 6. Commit Changes

**Important**: Create separate commits in riotapi-schema for each override!

```bash
cd /path/to/riotapi-schema
git add -A
git commit -m "Add specificFieldName to endpoint-version.DtoName"
```

This makes the history cleaner and easier to revert individual changes if needed.

## Common Field Types

```jsonc
// String
"fieldName": {
  "type": "string",
  "x-type": "string"
}

// Integer (32-bit)
"fieldName": {
  "type": "integer",
  "format": "int32",
  "x-type": "int"
}

// Long (64-bit)
"fieldName": {
  "type": "integer",
  "format": "int64",
  "x-type": "long"
}

// Float
"fieldName": {
  "type": "number",
  "format": "float",
  "x-type": "float"
}

// Double
"fieldName": {
  "type": "number",
  "format": "double",
  "x-type": "double"
}

// Boolean
"fieldName": {
  "type": "boolean",
  "x-type": "boolean"
}

// Array
"fieldName": {
  "type": "array",
  "items": {
    "type": "string",
    "x-type": "string"
  },
  "x-type": "List[string]"
}
```

## Troubleshooting

### Field not appearing in generated Rust code
- Check if the DTO is fully overridden in `schemaOverrides.jsonc` - if so, add the field there instead of `dtoExtraFields.jsonc`
- Verify the OpenAPI spec was regenerated and contains your field
- Ensure srcgen is using the local schema path, not downloading from online

### Tests still failing after adding field
- Make sure you added the field to `dtoOptional.jsonc` if it should be optional
- Check the error message - if it changed from "unknown field" to "missing field", the field needs to be optional
- Verify you're running tests with `--features riven/deny-unknown`

### Queue ID drift (e.g. queueId 710)
- Queue IDs can appear in live payloads before Riot's official `queues.json` is updated.
- If strict enum deserialization fails on a new ID, add it in `riotapi-schema/src/enums/queues.jsonc` with clear notes.
- Keep descriptions stable and explicit because they influence generated enum identifiers.
- Queue data sources:
  - CommunityDragon (often latest, but very large): https://raw.communitydragon.org/latest/plugins/rcp-be-lol-game-data/global/default/v1/queues.json
  - Riot official (usually smaller/easier to inspect): https://static.developer.riotgames.com/docs/lol/queues.json
- Practical usage:
  - Do not read the full CommunityDragon JSON directly in chat/output; use targeted filtering by queueId (or a small script) instead.
  - Reading Riot's official queues JSON directly is generally fine.
- Regenerate `riotapi-schema` (`node .`) and then regenerate Riven with local source root:

```bash
cd /path/to/Riven
node riven/srcgen --spec=../../../riotapi-schema/out
```

### Schema generation fails
- Check JSON syntax in the override files (trailing commas, etc.)
- Ensure DTO names match exactly (case-sensitive)
- Look at similar existing overrides for reference

## Example: Recent Updates (January 2026)

1. **premierRosterType** (val-ranked-v1.PlayerDto)
   - File: dtoExtraFields.jsonc + dtoOptional.jsonc
   - Type: string (optional)

2. **roleBoundItem** (match-v5.ParticipantDto)
   - File: dtoExtraFields.jsonc + dtoOptional.jsonc
   - Type: int (optional)

3. **position** (lol-challenges-v1.ChallengePointDto)
   - File: schemaOverrides.jsonc + dtoOptional.jsonc
   - Type: long (optional)
   - Note: Had to use schemaOverrides because ChallengePointDto was already fully overridden

## Quick Checklist

- [ ] Run `./test.bash` to identify failures
- [ ] Add field to appropriate schema file(s)
- [ ] Mark as optional in `dtoOptional.jsonc` if needed
- [ ] Run `node .` in riotapi-schema
- [ ] Run `node riven/srcgen` in Riven
- [ ] Test with full features enabled
- [ ] Commit to riotapi-schema with descriptive message
- [ ] Consider committing Riven changes as well
