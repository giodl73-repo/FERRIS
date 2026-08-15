# Pulse 55 execution record

Status: consumed one-call terminal prerequisite-identity failure
`terminal-prerequisite-identity-failure` under
`P55-P33-RETAINED-IDENTITY-CONTRACT`. The sole invocation returned
`not-attempted`; no public result or witness tree was transferred.

## Authority and public validation

- Authority commit: `36b3ac6b9692924af57c7c98b0a976835fe778f6`
- Immutable self-excluding cutoff:
  `47113e444ef3309afec9a844f0cba62775f19f6f`
- Declaration identity:
  `sha256:45ac35775c34e8a86fdc90ad1554104f2728a676d51ab46125bfcf126db21655`
- An unauthenticated GitHub API request for the authority commit returned HTTP
  `200` before custody work.
- Fresh anonymous Windows and Ubuntu-24.04 checkouts of the cutoff were
  detached at the exact revision, clean, and configured
  `core.autocrlf=false`.
- The exact public Pulse 55 authority validator passed all `4/4` tests on
  both fresh validator checkouts: Windows Rust/Cargo `1.95.0` and
  Ubuntu-24.04 WSL Rust/Cargo `1.97.1`.

## Public prerequisite evidence

One fresh Pulse 44 custody operation was completed once per platform before
the call. Both summaries reported published `2/2` final files, `2/2` work and
stage verification, one rename, and zero retries. Windows reported the
explicit unsupported stage/final-parent directory-sync posture; Ubuntu
reported both as synced.

The observed retained Pulse 33 identities were:

| Platform | Artifact size / SHA-256 | Receipt payload SHA-256 |
| --- | --- | --- |
| Windows | `1435136`; `sha256:58b3e9cc815008ed76caced2ef9b54d86fc815d892a132926f1e7180d335452c` | `sha256:c2d6c5b92ec2e17814aa26c2a571d49ff4ac1d50b0865cead6ba6b89809e4303` |
| Ubuntu-24.04 WSL | `1945448`; `sha256:6cf654f1a7c277317753a9cb0f0a7bc1e183de40715b25e35828bcdb9d18cfe4` | `sha256:0611621f66a8044a747e262b43adac29a1d1abc08f63ec1b781d0c7e1f5318e6` |

The Windows artifact did not match the exact P51 expectation of `1436672`
bytes and
`sha256:0736392a9fab3fa9404554f86e82e1dfcdf9e68e44aa3df8a4cddd1d58a34fd8`.
Ubuntu did match P51's expected Ubuntu artifact size and hash
(`1945448`; `sha256:6cf654f1a7c277317753a9cb0f0a7bc1e183de40715b25e35828bcdb9d18cfe4`);
this record does not claim an Ubuntu artifact mismatch. Its retained receipt
payload identity nevertheless differed from P51's published non-retaining
Ubuntu receipt identity
`sha256:b01e45259e340309772e1d5d5c947cff163ada8dad8b9ddf19775f0a537c4cae`.
The Windows retained receipt likewise differed from P51's published
non-retaining Windows receipt identity
`sha256:b1d42470ca709406c5869bae9e677334539e745faf8e0f400e4ae93f34cf7d7a`.

This identifies a structural contract issue, not a product conclusion:
the `build_freeze` receipt field `retained_in_public_bundle` changes when the
executable is retained, so an exact published receipt payload identity cannot
equal a real retained-custody receipt. Windows `/Brepro` plus the Rust/Cargo
version alone also did not reproduce the historic binary, showing that the
Windows linker/SDK environment is underbound. This is limited to public
prerequisite identity evidence.

## Sole Pulse 53 call

`run_witness_preserving_ordered_executor(...)` from the exact sealed Pulse 53
release was invoked exactly once.  No Pulse 51, Pulse 52, Pulse 47, or Pulse
43 runtime callable was directly invoked.

- Returned public publication disposition/classification: `not-attempted`
- Ordered events: one `terminal-stop/failed` at
  `pulse-41-pulse-39-public-custody`; zero completed gates.
- Process counts: Windows `0`, Ubuntu `0`; no-launch dispositions `0`.
- P27, P39, P41, materializer, verifier, and terminal P47 invocation counts:
  all `0`.
- No seed, descriptor root, candidate process, result, or witness artifact was
  created by this call.

## Public-artifact disposition

The exact Pulse 55 result path
`docs/simulations/profile-diff-held-out/pulse-55-public-result/` is absent.
The exact Pulse 55 witness path
`docs/simulations/profile-diff-held-out/pulse-55-publication-witness/` is
absent.  No transfer was authorized by the returned `not-attempted`
classification: result and witness transfers are both `0`, so
transferred-tree re-verification and artifact digests are not applicable.

Category, diagnostic, product, and fix conclusions are all null.  This record
does not authorize a retry, amendment, resume, or further launch. Pulse 55 is
permanently non-retryable and non-resumable: its one P53 call is consumed.

Any successor authority requires a new sealed executor chain, not an
implementation change under this authority. It must bind a corrected P33
retained-build/custody contract, a fully bound Windows linker/SDK environment
or qualified deterministic linker route, semantic receipt verification
appropriate to retained artifacts, and replacement ordered/witness layers
that bind the corrected diagnostic executor.
