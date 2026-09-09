# Ferris Go Artifact Compatibility Evidence

Status: GO-WP-006 base compatibility proof; measured qualification added later
Date: 2026-08-29

## Claim boundary

This evidence proves structural producer-consumer compatibility and complete
fan-in. It does not prove producer authenticity, transfer integrity, storage
availability, cache value, publication readiness, or production savings.

## Result

`ferris artifacts --request <JSON>` now:

- requires exact artifact identity and digest;
- compares repository, source, toolchain, operating system, architecture,
  target, profile, features, configuration, manifest, and owner command;
- reports every mismatched dimension;
- blocks all consumers when the producer failed, was cancelled, or is
  unavailable;
- retains missing and incompatible expected consumers in fan-in;
- rejects optional consumers as expected required fan-in members; and
- emits deterministic `observation_only` evidence independent of input order.

The base compatibility mode reads no artifact bytes. The later measured mode
reads only explicit bounded local artifact and manifest files, binds their
streamed SHA-256 identities to the producer declaration, and may fail closed
with `--require-compatible`. Ferris still owns no upload, download, storage,
retention, cache, signing, publication, or deployment behavior.

## Fixture disposition

The accepted fixture uses one successful producer, two compatible required
consumers, and complete fan-in. Independent mutations of all eleven
compatibility dimensions, artifact identity, artifact digest, producer status,
expected membership, strict shape, and identity validity block reuse or fan-in.

GO-WP-006 establishes the compatibility contract needed before any owner chooses
to wire real artifact transport. No adopter or performance claim is made.
