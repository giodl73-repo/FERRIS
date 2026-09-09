# Ferris ICELINES Artifact Qualification

Status: Cross-platform adopter proof complete

## Repository

- Repository: `giodl73-repo/ICELINES`
- Pull request: [#48](https://github.com/giodl73-repo/ICELINES/pull/48)
- Hosted run:
  [`33316361372`](https://github.com/giodl73-repo/ICELINES/actions/runs/33316361372)
- Platforms: GitHub-hosted Linux, macOS, and Windows

## Owner workflow

ICELINES remained responsible for building its native package, publishing the
workflow artifact, downloading it in consumer jobs, and deciding whether the
package was acceptable. The workflow exposed the exact packaged manifest as a
sidecar.

Ferris measured the downloaded archive and manifest sidecar, compared those
files with the producer declaration and complete consumer envelope, and emitted
one `ferris.artifact-qualification-report/v1` per platform. The owner verifier
independently checked that the sidecar matched the manifest inside the archive
before source, hash, and smoke acceptance.

## Result

The Linux, macOS, and Windows qualification jobs passed together with all
existing owner checks. A tampered artifact control produced a retained rejected
qualification report rather than success-shaped output.

The Windows adopter used a bounded sparse checkout because the retained Ferris
simulation corpus exceeds default Windows checkout path limits. That workaround
is a Ferris repository portability debt, not an ICELINES artifact-contract
requirement.

## What this proves

- Ferris can bind explicitly supplied artifact and manifest bytes to an owner
  declaration with bounded streaming SHA-256.
- The same qualification and complete fan-in contract operates across the three
  hosted operating systems.
- Owner verification remains independently visible rather than being replaced
  by Ferris compatibility output.
- Incompatible or tampered evidence can fail closed while retaining the
  complete report.

## What this does not prove

- Producer or provider authenticity.
- Artifact upload, download, storage, retention, extraction, or caching.
- Signing, publication, deployment, or release readiness.
- Windows full-checkout portability for the retained deep fixture corpus.
- Build-time savings, cache value, or production support.
