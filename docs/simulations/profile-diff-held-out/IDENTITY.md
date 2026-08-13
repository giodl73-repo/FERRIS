# Profile Diff Public Identity Contract

Status: Frozen public reference
Contract revision: 2

This document freezes the identity algorithms implemented at the public
Pulse 17 cutoff. It is descriptive of the current Rust behavior. It does not
authorize a new identity format or CLI change.

## Common primitives

- Hash algorithm: SHA-256.
- Hex encoding: exactly 64 lowercase ASCII hexadecimal characters.
- Text encoding: UTF-8 with no Unicode normalization.
- `sha256:<hex>` identifies an unframed byte digest.
- `selection:<hex>`, `invocation:<hex>`, `profile-diff:<hex>`, and
  `result:<hex>` identify their respective domains.
- A NUL-framed identity hashes each UTF-8 part followed by one `0x00` byte,
  including the final part.

For `parts = [p0, p1, ...]`:

```text
SHA256(UTF8(p0) || NUL || UTF8(p1) || NUL || ...)
```

Selection and invocation identities use a hashed domain string as their first
part. `profile-diff` and `result` identities use the textual prefix only after
hashing their exact compact JSON payload; the prefix is not included in those
two hashes. Scorers MUST preserve that distinction.

## Compact `serde_json` behavior

Identity payloads use `serde_json` 1.0.143 `to_vec`, not the pretty CLI
formatter:

- no insignificant whitespace and no terminal newline;
- Rust struct members in declaration order;
- `serde_json::Value` object members in ascending key order because the
  `preserve_order` feature is not enabled;
- array order retained;
- JSON strings escaped by `serde_json` without Unicode normalization;
- booleans, null, signed and unsigned integers serialized as JSON literals;
- accepted finite floating-point values serialized by `serde_json`'s current
  shortest round-trippable representation; and
- duplicate object members rejected before canonical serialization.

The public vectors freeze exact bytes for every identity payload used by the
qualification suite.

## Canonical profile content and value digests

A successfully loaded input is deserialized into this exact field order:

```text
ProfileEvidence {
  schema,
  profile_id,
  revision,
  consumer,
  sections: {
    identity,
    closure,
    features,
    toolchain,
    targets,
    providers,
    native,
    stages,
    assurance,
    stewardship,
    support,
    lifecycle
  }
}
```

Only those fields participate. Source path, source bytes, source member order,
insignificant whitespace, and duplicate-bearing source text do not
participate. All twelve section values, including nulls and empty containers,
do participate.

```text
content_digest = "sha256:" + hex(SHA256(serde_json::to_vec(ProfileEvidence)))
value_digest   = "sha256:" + hex(SHA256(serde_json::to_vec(JSON value)))
```

Revision changes use the value digest of the JSON string value. Added values
have a null before digest; removed values have a null after digest; changed
values have both digests.

## Lexical request-path normalization

Path normalization is lexical and performs no filesystem lookup, case
folding, drive canonicalization, symlink resolution, or Unicode
normalization.

1. Strip one exact leading Windows extended-path prefix `\\?\`, if present.
2. Replace every `\` with `/`.
3. Select one prefix:
   - leading `//`: prefix `//`, rooted;
   - otherwise leading `/`: prefix `/`, rooted;
   - otherwise `<any-byte>:/`: preserve those first three bytes as the drive
     prefix and mark rooted;
   - otherwise no prefix and not rooted.
4. Split the remainder on `/`.
5. Discard empty components and `.`.
6. For `..`, pop the preceding component when it exists and is not `..`.
   Preserve an unpoppable `..` only for a relative path; discard it for a
   rooted path.
7. Join retained components with `/`.
8. Return `.` for an empty relative result. For a nonempty rooted result,
   join it to the preserved prefix, adding `/` only when the prefix does not
   already end in `/`.

Drive-letter case and all other component case are retained. A path such as
`C:relative` is relative because it does not match the `<byte>:/` branch.

```text
request_digest(path) =
  "sha256:" + hex(SHA256(UTF8(lexically_normalize(path))))
```

## Profile-diff selection branches

All selection hashes use NUL framing.

### Both inputs loaded

This branch covers success, difference, profile mismatch, consumer mismatch,
and a change-count overflow discovered after both inputs are canonicalized.

```text
parts = [
  "profile-diff-selection",
  before_content_digest,
  after_content_digest
]
selection_identity = "selection:" + hex(SHA256(frame(parts)))
```

### First input not completely loaded

This branch covers missing, unreadable, non-file, oversized, malformed,
duplicate-bearing, unsafe-key, invalid-shape, invalid-metadata, and unsupported
first inputs.

```text
request_material =
  "before-request=" + request_digest(before_path) +
  ";after-request=" + request_digest(after_path)

parts = ["profile-diff-selection", request_material]
selection_identity = "selection:" + hex(SHA256(frame(parts)))
```

### First input loaded; second input not completely loaded

This branch covers the same second-input failure classes after the first
content digest exists. The first request path is intentionally excluded.

```text
selection_material =
  "before=" + before_content_digest +
  ";after-request=" + request_digest(after_path)

parts = ["profile-diff-selection", selection_material]
selection_identity = "selection:" + hex(SHA256(frame(parts)))
```

### Error-envelope fallback

If a profile-diff `CoreError` has no bound selection material, the fallback is
the first-input-not-loaded request-material branch. Normal `create_profile_diff`
errors bind one of the three branches above.

### CLI parse and guarded-process identities

Arguments are processed in original order after skipping argv[0] and one
recognized subcommand. `--before` and `--after` values become
`value:<request_digest>`, `--format` is lowercased, unknown positional values
are replaced by their `sha256:` digest, and option spelling/order is retained.
Both `--x value` and the implemented `--x=value` branches are frozen by the
public vectors.

```text
normalized_text = normalized_parts joined with NUL
normalized_digest = sha256(UTF8(normalized_text))

selection parts = ["selection", semantic_command_id, normalized_digest]
invocation parts = [semantic_command_id] + normalized_parts
```

These identities are used for Clap-invalid and guarded internal envelopes.
They are not used by a successfully parsed profile-diff request.

## Profile-diff invocation identity

Every parsed profile-diff success or error uses:

```text
parts = [
  "profile-diff",
  selection_identity,
  "profile-schema=ferris.profile-evidence/v0",
  "input-max-bytes=1048576",
  "change-max=10000",
  "owner-tools=false",
  "network=false",
  "mutation=false"
]

invocation_identity = "invocation:" + hex(SHA256(frame(parts)))
```

The content or failure selection, schema version, both resource bounds, and
three authority prohibitions participate. Request paths, output format,
command package version, platform, current directory, and timestamps do not.

## Diff identity

The exact compact identity payload is the serialized
`ferris.profile-diff/v0` record in declaration order with `diff_id` set to the
empty string:

```text
{
  schema,
  diff_id: "",
  before,
  after,
  changed_sections,
  changes,
  unchanged_sections,
  unknowns,
  limitations,
  executable
}
```

The before and after references include `profile_id`, `revision`, `consumer`,
and `content_digest` in that order. Each change includes `path`,
`change_kind`, `before_value_digest`, and `after_value_digest` in that order.
The final populated `diff_id` is excluded by replacing it with the empty
string, not by omitting the member.

```text
diff_id = "profile-diff:" +
  hex(SHA256(serde_json::to_vec(record_with_empty_diff_id)))
```

The schema version, references, all sorted arrays, fixed unknowns and
limitations, digest nulls, and `executable:false` participate.

## Result identity

The result identity excludes only the envelope's `result_identity` member. Its
compact payload has this declaration order:

```text
{
  schema,
  command_version,
  semantic_command_id,
  selection_identity,
  invocation_identity,
  result_class,
  process_exit_code,
  diagnostics,
  record
}
```

`record` is the complete populated profile-diff record or JSON null.
`source_digest` is always present and may be null. `bounded_output` is omitted
when absent and is an object when present; it is never serialized as null.
Diagnostic array order and next-action order participate.

```text
result_identity = "result:" +
  hex(SHA256(serde_json::to_vec(CommandResultIdentityInput)))
```

The command package version participates here and nowhere earlier.

## Aggregate public-output digest

The scorer sorts rows by ascending UTF-8 byte order of `platform`, then
`declared_case_id`, then numeric `attempt`. It constructs this exact compact
payload member order:

```text
{
  schema: "ferris.aggregate-public-output/v1",
  contract_revision: 2,
  rows: [
    {
      platform,
      declared_case_id,
      attempt,
      stdout_digest,
      stderr_digest,
      process_exit_code
    }
  ]
}
```

The aggregate digest is:

```text
sha256(
  UTF8("ferris.aggregate-public-output/v1") || NUL ||
  serde_json::to_vec(payload)
)
```

It is rendered as `sha256:<lowercase-hex>`. No case-to-output mapping or row
payload is included in the public-safe result.

## Harness receipt and evidence digests

Public receipt vectors use these harness-owner algorithms:

```text
executable_digest = sha256(exact executable bytes)
stream_digest     = sha256(exact captured stream bytes)

command_digest =
  sha256(UTF8("ferris.command-argv/v1") || NUL ||
         compact JSON argv array)

environment_digest =
  sha256(UTF8("ferris.environment-allowlist/v1") || NUL ||
         compact JSON entries sorted by name)
```

For a receipt identity, parse the receipt into `serde_json::Value`, replace
its identity member (`row_identity`, `receipt_identity`,
`inventory_identity`, or `comparison_identity`) with the empty string, retain
array order, let the default `serde_json::Map` order every object key, and
serialize compactly:

```text
receipt_identity =
  sha256(UTF8(receipt schema value) || NUL ||
         serde_json::to_vec(receipt_with_empty_identity))
```

The identity member is replaced, not omitted. Environment entries and every
set-like receipt array MUST be sorted before identity construction. Empty
streams use the SHA-256 digest of zero bytes, never a null digest.

## Diff ordering

- Section traversal order is the fixed twelve-section order above.
- `changed_sections` and `unchanged_sections` are each sorted ascending.
- Object recursion uses the union of keys sorted ascending.
- Arrays compare by numeric position from zero through the larger length.
- Added and removed nonempty containers expand to leaf changes; empty
  containers remain one change at their own pointer.
- JSON Pointer tokens replace `~` with `~0`, then `/` with `~1`.
- Final changes are sorted ascending by the complete escaped pointer.
- The 10,000th change is allowed. Attempting to append the 10,001st returns
  `blocked`/7 and no partial record.

## Machine serialization and human grammar

Machine output uses `serde_json::to_vec_pretty`, followed by exactly one LF
byte. It is UTF-8, uses the current struct member order, emits success and
difference on stdout only, and emits every non-success envelope on stderr only.
The other stream is zero bytes.

Human success/difference output is the exact LF-terminated grammar below.
Items appear in typed-record order. Empty changed sections, changes, unchanged
sections, or unknowns use one `  - none` line. Digest nulls render as `none`.

```text
Ferris profile diff <diff_id>
Schema: <record.schema>
Result: <result_class>
Executable: <true|false>
Before: profile_id=<...>, revision=<...>, consumer=<...>, content_digest=<...>
After: profile_id=<...>, revision=<...>, consumer=<...>, content_digest=<...>
Changed sections:
  - <section-or-none>
Changes:
  - <path>: <kind> (before_digest=<digest-or-none>, after_digest=<digest-or-none>)
Unchanged sections:
  - <section-or-none>
Unknowns:
  - <unknown-or-none>
Limitations:
  - <limitation>
```

Parsed-command failures ignore the requested human format and emit the typed
JSON error envelope on stderr. Human output does not carry envelope identities,
diagnostics, or exit fields; those are established by the separately captured
process receipt.
