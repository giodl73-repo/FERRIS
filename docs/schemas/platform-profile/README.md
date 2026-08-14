# Ferris Platform Profile Schema

Status: Frozen controlled-fixture contract
Schema: `ferris.platform-profile/v1`
Implementation authority: Schema documents and controls only

## Boundary

This directory defines the canonical profile record used by the
PLATFORM-001 controlled conformance program. It is not a support catalog,
distribution, resolver, approval, generated profile, or promise that any
package, target, provider, runtime, platform, or deployment is suitable.

The normative Draft requirements remain in
[PLATFORM-001](../../specs/FERRIS_PLATFORM_PROFILE_CONTRACT.md). The JSON
Schema freezes their controlled-fixture projection so the nine family
programs can produce comparable records.

## Schema identity

- schema ID: `ferris.platform-profile/v1`;
- JSON Schema dialect: 2020-12;
- schema document:
  [`ferris.platform-profile.v1.schema.json`](ferris.platform-profile.v1.schema.json);
- canonical digest algorithm: SHA-256; and
- maximum canonical record size for the controlled program: 4 MiB.

Schema identity, profile revision, package version, lock identity, compiler
identity, contract version, evidence revision, support revision, and lifecycle
revision are distinct.

## Canonical serialization

Canonical bytes are UTF-8 JSON with:

1. object members sorted by Unicode code point;
2. arrays retained in declared order;
3. no insignificant whitespace;
4. JSON escaping applied without Unicode normalization;
5. no floating-point values in canonical fields; and
6. one domain-separated digest frame:

```text
ferris.platform-profile/v1 NUL <canonical-byte-length> NUL <canonical-bytes>
```

The harness must reject duplicate members before ordinary JSON
deserialization. Canonicalization never makes two duplicate-bearing inputs
equivalent.

## Output-visible metadata

Identifiers, names, source locations, owner names, command arguments,
diagnostics, limitation text, and object member names may be visible in human
or machine projections. They must not contain credentials, reusable tokens,
private keys, secret values, or unrestricted environment data.

Evidence payloads remain in owner records. The profile stores bounded
references, digests, typed states, attribution, dates, expiry, diagnostics,
and limitations.

## RUNE fixture dependency and v1 reconciliation

The semantic-contract fixture boundary is frozen to:

| Field | Value |
|---|---|
| Repository | `https://github.com/giodl73-repo/RUNE.git` |
| Revision | `194449444624fb10add4137cb0da8d0327164fa7` |
| Crate version | `0.1.0` |
| Descriptor collection | `v0` |
| Neutral profile | `rune.neutral_descriptor_json` |
| Neutral profile version | `v0` |

Pulse 21 recognizes this exact already-bound revision as satisfying
CONTRACT-001's Typebook/RUNE v1 **contract-baseline dependency**. Public RUNE
evidence at the revision records v1 release readiness as closed, describes
RUNE v1 as ready publishable contract infrastructure, and retains eight
accepted specification rows.

The decision does not claim Cargo SemVer `1.0.0` publication or a Git
`v1.0.0` tag. The workspace remains `0.1.0`; the descriptor collection and
neutral profile remain `v0`. No fixture bytes, identities, digests, or FERRIS
production behavior change.

The closed
[`ferris.rune-v1-dependency-receipt/v1` receipt](../../plans/validation/PULSE-21-RUNE-V1-DEPENDENCY-RECEIPT.json),
its
[Draft 2020-12 schema](../../plans/validation/ferris.rune-v1-dependency-receipt.v1.schema.json),
and the
[nine-role review](../../plans/reviews/PULSE-21-RUNE-V1-DEPENDENCY-ROLE-REVIEW.md)
retain these fact boundaries. The RUNE dependency is no longer a
PLATFORM-001 blocker; the valid Pulse 17 `process-exit-agreement` failure
remains the sole blocker.

## Experimental diff projection

`ferris.profile-evidence/v0` remains the experimental input accepted by the
existing `profile-diff` command. A v1 profile may be projected into its twelve
sections, but the projection is lossy:

| v0 section | v1 sources |
|---|---|
| `identity` | schema, profile, consumer, owner, operation, status, dates |
| `closure` | selection and closures |
| `features` | requested and effective features |
| `toolchain` | Cargo, rustc, toolchain, host, components |
| `targets` | targets and target-specific closure coordinates |
| `providers` | providers and runtimes |
| `native` | native tools plus native contract and stage references |
| `stages` | stage results and capabilities |
| `assurance` | assurance evidence |
| `stewardship` | stewardship evidence |
| `support` | support commitments and expiry |
| `lifecycle` | adoption, renewal, substitution, emergency, rollback, removal |

The projection must retain a loss record. The v0 command still compares
caller-provided section data without interpreting v1 semantics.

## Strictness and extensions

Canonical fields reject unknown members. Namespaced extensions use a
multi-segment lower-case key such as `example.owner-field`. An extension may
add evidence but cannot redefine a canonical field, result state, identity,
owner authority, lifecycle requirement, or removal rule.

Unsupported schema versions, malformed JSON, duplicate members, ambiguous
source locations, unsafe output-visible metadata, and records above the size
bound fail explicitly. Unknown evidence stays `unknown`; it is not filled
with a default.
