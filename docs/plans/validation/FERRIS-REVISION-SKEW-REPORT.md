# Ferris Revision-Skew Report

Date: 2026-08-19
Status: Bounded read-only product behavior

## Purpose

`revision-skew` reports whether an explicitly named consumer lockfile revision
is equal to, behind, ahead of, or divergent from an explicitly named local
producer checkout. It also reports unavailable and unknown evidence without
turning those states into compatibility conclusions.

## Request

Paths are relative to the request file directory and must remain beneath it.
Every relationship and package name is explicit.

```json
{
  "schema": "ferris.revision-skew-request/v0",
  "analysis_id": "example/shared-substrate",
  "producers": [
    {
      "producer_id": "example/fletch",
      "repository_url": "https://github.com/example/FLETCH.git",
      "checkout_path": "fletch",
      "observed_revision": "0123456789abcdef0123456789abcdef01234567"
    }
  ],
  "consumers": [
    {
      "consumer_id": "example/application",
      "manifest_path": "application/Cargo.toml",
      "dependencies": [
        {
          "producer_id": "example/fletch",
          "package_name": "fletch-core"
        }
      ]
    }
  ]
}
```

## Evidence model

For each explicit dependency, Ferris:

1. finds matching git declarations among Cargo workspace-member packages;
2. finds matching package and repository source entries in `Cargo.lock`;
3. requires the producer's `observed_revision` to equal checkout HEAD; and
4. requires the producer checkout to be clean; and
5. compares the two local commit objects with `git merge-base --is-ancestor`.

Declaration evidence preserves branch, revision, tag, default-branch,
ambiguous, or missing state separately from the locked revision.

## Safety and authority

The command is non-executable. It requests no network access and performs no
fetch, checkout, update, build, test, validation, or file mutation. It does not
discover relationships, infer compatibility, recommend an update
automatically, or supersede producer and consumer owners.

A dirty producer checkout is reported as `unavailable`: its HEAD remains
observable, but that revision does not fully identify the source state being
presented for comparison.

The execution record and real five-repository replay are in
[Pulse 01](../../../context/waves/2026-08-19-revision-skew-report/pulses/pulse-01.md).
Dirty-checkout hardening and the adversarial matrix are in
[Pulse 02](../../../context/waves/2026-08-19-revision-skew-report/pulses/pulse-02.md).
