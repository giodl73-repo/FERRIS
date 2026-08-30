# Ferris Artifact Compatibility Contract

Status: GO-WP-006 implementation target

## User outcome

Allow one successful owner build artifact to serve compatible validation
consumers without treating matching bytes as proof of fitness.

Repository owners retain build commands, artifact production, storage,
transport, retention, consumer semantics, and publication. Ferris contributes a
deterministic compatibility decision and complete fan-out/fan-in evidence.

## Interface

`ferris artifacts --request <JSON>` reads
`ferris.artifact-reuse-request/v1` and emits
`ferris.artifact-reuse-report/v1`.

An additive measured mode accepts:

```console
ferris artifacts \
  --request <JSON> \
  --artifact-path <FILE> \
  --manifest-path <FILE> \
  [--require-compatible]
```

The artifact and manifest paths are paired and cannot be supplied separately.
Measured mode emits `ferris.artifact-qualification-report/v1`, containing the
measured SHA-256 identities and byte sizes, producer-binding result, and the
complete nested reuse report. Paths are operational inputs and are not retained
in the portable report.

The request contains one producer attempt, its artifact identity and digest, a
complete compatibility envelope, consumer requirements, and the exact expected
fan-in member set. Unknown fields, malformed identities, duplicate consumers,
duplicate features, and duplicate expected members fail the request.

## Compatibility envelope

Reuse requires exact equality for:

- repository and source revision;
- toolchain identity;
- operating system and architecture;
- build target and profile;
- sorted feature set;
- configuration identity;
- manifest identity; and
- owner command identity.

Each consumer also binds the expected artifact identity and digest. Every
mismatched compatibility dimension remains visible in the report. Digest
equality establishes content integrity only; it never overrides a compatibility
mismatch.

## Producer and fan-in behavior

A failed, cancelled, or unavailable producer blocks every consumer before
compatibility evaluation. No fallback producer or rebuild is inferred.

Fan-in succeeds only when every expected consumer:

- is present;
- is marked required; and
- accepted the producer artifact under the full envelope.

Missing, optional, incompatible, or producer-blocked expected consumers remain
named and make fan-in non-success. Additional consumers cannot satisfy a
missing expected identity.

## Measured qualification

Ferris measures bounded regular files using streaming SHA-256:

- artifacts are limited to 16 GiB;
- manifests are limited to 16 MiB;
- unavailable, non-regular, or oversized inputs fail with named diagnostics;
- the measured artifact digest must equal the producer artifact digest; and
- the measured manifest digest must equal the producer manifest identity.

Qualification succeeds only when both producer measurements match and the
existing required fan-in succeeds. Without `--require-compatible`, a rejected
qualification remains an observation and exits zero. With
`--require-compatible`, the same complete report is emitted and the process
returns exit code 1 (`difference`). Invalid or unavailable inputs retain their
existing typed non-success classes.

## Evidence and authority

The base reuse report is deterministic and `observation_only`. It validates the
internal consistency of supplied evidence but does not authenticate the
producer, read or transfer artifact bytes, own storage, authorize cache reuse,
or grant publication.

The measured qualification report reads only the two explicitly supplied local
files. It establishes byte-to-declaration binding, not provider authenticity.
Owner CI still performs transport and decides whether a qualified artifact may
be consumed.

Validation success is not release success. Publication, signing, promotion,
deployment, retention, and rollback remain separate owner-controlled actions.

## Boundaries

GO-WP-006 does not add:

- an artifact or cache service;
- upload, download, eviction, or retention;
- remote provider APIs;
- credential handling;
- live build execution;
- archive extraction or manifest interpretation;
- publication or deployment; or
- consumer-repository modification.
