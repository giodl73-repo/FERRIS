---
maxim_schema: maxim.frontmatter.v1
id: maxim:rust-security-assurance:parsing-deserialization-input-validation-and-protocol-abuse
kind: guide
module: rust-security-assurance
section: security-engineering
title: Parsing, Deserialization, Input Validation, and Protocol Abuse
status: source-custody
source_custody: partial
current_path: rust-security-assurance/07-PARSING-DESERIALIZATION-INPUT-VALIDATION-AND-PROTOCOL-ABUSE.md
canonical_path: rust-security-assurance/07-PARSING-DESERIALIZATION-INPUT-VALIDATION-AND-PROTOCOL-ABUSE.md
backsource_ids: [mdloom-backfill:rust-security-assurance:07-parsing-deserialization-input-validation-and-protocol-abuse]
concepts: [parsing, deserialization, input validation, serde, protocol security]
root_concepts: [input validation]
index_roles: [guide, root-concept]
remap_from: []
remap_to: []
updated: null
---
# Parsing, Deserialization, Input Validation, and Protocol Abuse

A memory-safe parser can still allocate without bound, accept ambiguous
representations, validate the wrong object, or drive a privileged state machine
into an unauthorized transition. Secure input handling is a staged
transformation from hostile bytes to authorized action, with explicit limits
before expensive work.

## The Big Picture

```
+============================================================================+
|                         HOSTILE INPUT PIPELINE                             |
+============================================================================+
| bytes -> framing -> syntax -> typed value -> semantic rules -> authz       |
|   |        |          |          |              |             |            |
| limits   length/EOF  grammar   duplicate/     ranges, IDs,   subject,       |
| first    compression  depth    canonical form  relationships  tenant, state |
+----------------------------------------------------------------------------+
| only after every stage: side effect / storage / privileged operation       |
+============================================================================+
```

Do not collapse the middle boxes into "`serde` succeeded." Deserialization
proves that bytes matched a representation accepted by the implementation.

## Bound Before You Parse

For a synchronous `Read`, cap bytes before buffering. This standard-library
example reads at most `MAX + 1` bytes and rejects overflow:

```rust
use std::io::{self, Read};

fn read_bounded(mut input: impl Read, max: usize) -> io::Result<Vec<u8>> {
    let limit = u64::try_from(max)
        .ok()
        .and_then(|n| n.checked_add(1))
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "limit too large"))?;
    let mut bytes = Vec::new();
    input.by_ref().take(limit).read_to_end(&mut bytes)?;
    if bytes.len() > max {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "input too large"));
    }
    Ok(bytes)
}
```

This bounds encoded bytes, not decompressed size, nesting depth, object count,
CPU, elapsed time, or downstream allocations. `Read` is synchronous, so a slow
network peer can still block unless the transport supplies a deadline. Choose a
practical `max` below the process allocation budget and place a budget at every
expansion boundary.

| Expansion | Required bound |
|-----------|----------------|
| Compressed -> plain bytes | maximum output and compression ratio/work |
| Frame -> collection | item count and per-item size |
| Recursive syntax | nesting depth or iterative parser budget |
| Name -> lookup | lookup count, latency, and result size |
| Regex/query | pattern complexity, timeout, or restricted grammar |
| Batch request | operations, fan-out, and transaction duration |

## Parse into Untrusted Types, Then Validate

```
WireRequest (syntactically valid, untrusted)
       |
       | validate ranges, canonical forms, cross-field rules
       v
ValidatedRequest (domain invariant)
       |
       | authorize subject + tenant + object + action + current state
       v
AuthorizedCommand
```

Use newtypes and private constructors to prevent accidental bypass. Keep
authorization context out of attacker-controlled payloads; a `tenant_id` field
is a request, not authority.

## Serde-Specific Review

Serde is a serialization framework, not an input-security policy.

| Concern | Review |
|---------|--------|
| Unknown fields | Reject for strict control messages or accept for forward compatibility by explicit policy |
| Duplicate map keys | Know the format/deserializer behavior; reject ambiguity where security-relevant |
| Untagged enums | Consider ambiguity and trial-parse cost; prefer explicit discriminants for protocols |
| Defaults | Ensure omitted fields cannot silently gain privilege or disable limits |
| Flattening | Review name collisions and ownership of fields |
| Custom deserializer | Treat as parser code; fuzz and bound allocations/recursion |
| Borrowed data | Ensure lifetime does not outlive the validated backing buffer |

`#[serde(deny_unknown_fields)]` is useful for some formats but is a compatibility
decision, not a universal rule.

## Canonicalization and Differential Interpretation

Many attacks exploit two components interpreting the same input differently:

```
client bytes
   +--> signature verifier sees canonical object A
   +--> router/parser sees object B
   +--> authorization cache keys object C
```

Define one canonical representation for signatures, cache keys, paths, host
names, Unicode identifiers, and normalized numbers. Avoid normalize-validate-
then-use-original bugs. When multiple parser implementations are unavoidable,
add differential tests over a shared corpus.

## Protocol State and Replay

| Property | Question |
|----------|----------|
| Ordering | Can message 3 arrive before message 2? |
| Replay | Is freshness bound to nonce, time, sequence, or channel? |
| Idempotency | Can retry duplicate a privileged effect? |
| Downgrade | Can a peer select weaker version/algorithm/capability? |
| Confused deputy | Is the caller's authority bound to the selected target? |
| Error oracle | Do failures reveal secret-dependent or account-existence information? |

Represent protocol states as enums and allow transitions through narrow methods,
but still test semantic and authorization invariants. Types help encode the
model; they do not prove the model is the right one.

## Old World -> New World Bridge

| Established practice | Rust expression |
|----------------------|-----------------|
| DTO then domain validation | deserialize into wire struct, convert via `TryFrom` to validated type |
| Parser combinators / compiler front end | staged framing, syntax, semantic analysis, then execution |
| ASP.NET model binding | Serde extraction; neither substitutes for authorization |
| XML entity/decompression bomb limits | explicit byte/output/depth/work budgets |
| Canonical request signing | one representation shared by verifier, router, and cache |

Microsoft API Management, WAFs, and service rate limits can reject some hostile
traffic before the Rust process. They are supplemental layers; the application
must still enforce its own representation, semantic, and authorization rules.

## Common Confusion Points

- **"Deserialized means valid."** It means representable, not authorized or
  semantically safe.
- **"Safe Rust parser cannot be exploited."** It may still panic, loop, allocate,
  amplify work, or trigger logic flaws.
- **"Reject all unknown fields."** That may be right for strict control planes
  and wrong for evolvable public formats; choose deliberately.
- **"Validate after decompression."** Enforce output/work limits during
  decompression, before full allocation.
- **"Normalize then keep the original."** Use the exact canonical value that was
  validated.
- **"Enums prevent protocol abuse."** They prevent some illegal local states;
  replay, authorization, and distributed ordering remain.

## Decision Cheat Sheet

| Situation | Do |
|-----------|----|
| Network/file input | Bound encoded bytes before buffering |
| Compressed/archive input | Bound expanded bytes, entries, depth, and work |
| Serde API request | Deserialize to untrusted wire type, then validated domain type |
| Signed or cached representation | Define and reuse one canonical form |
| Privileged operation | Bind subject, tenant, object, action, and current state after parsing |
| Multiple parsers/components | Differential-test a shared adversarial corpus |
| Recursive/ambiguous format | Add explicit depth/trial/work budgets and fuzz it |

## Primary Sources

- Serde documentation: https://serde.rs/
- Rust Fuzz Book: https://rust-fuzz.github.io/book/
- OWASP Input Validation Cheat Sheet:
  https://cheatsheetseries.owasp.org/cheatsheets/Input_Validation_Cheat_Sheet.html
- IETF Robustness Principles considerations, RFC 9413:
  https://www.rfc-editor.org/rfc/rfc9413
- CWE-20 Improper Input Validation: https://cwe.mitre.org/data/definitions/20.html

## Related Guides

- Previous: [06-SECRETS-CRYPTOGRAPHY-RANDOMNESS-AND-KEY-HANDLING.md](06-SECRETS-CRYPTOGRAPHY-RANDOMNESS-AND-KEY-HANDLING.md)
- Next: [08-CONCURRENCY-RESOURCE-EXHAUSTION-AND-DENIAL-OF-SERVICE.md](08-CONCURRENCY-RESOURCE-EXHAUSTION-AND-DENIAL-OF-SERVICE.md)
- Fuzzing: [10-FUZZING-PROPERTY-TESTING-AND-CORPUS-MANAGEMENT.md](10-FUZZING-PROPERTY-TESTING-AND-CORPUS-MANAGEMENT.md)
