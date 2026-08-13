# Pulse 12 Assurance, Packaging, and Deployment Family Validation

Date: 2026-08-13
Implementation cutoff: `e60d67e`
Disposition: Windows and Unix development validation passed

Both exact revisions construct a non-empty Cargo `.crate` package in an
isolated target directory. `r1` inventories one artifact. `r2` adds an
explicit channel, distinct current/prior identities, and prior identity as
the exact rollback identity.

| Revision | Source-tree digest | Canonical profile digest |
|---|---|---|
| `r1` | `sha256:e908099c512f57803f1377ba557e986dcb83d86d5dc3cc7fe844828127b70384` | `sha256:355ea0356878d62de57381d2098599f0c5f3e59bc7bce6bcc1d83d55f3f9f4f6` |
| `r2` | `sha256:aa6c82c12bba4582e5e081e6fa7c4cedfec7545b6bf3dcdb16d1ad65e8ac9c9b` | `sha256:26ce6fe651f98fed2068149d54fd4b9b632e08ebf4b9e0ad272f215d41ad0475` |

Windows and Ubuntu WSL2 Rust/Cargo 1.95.0 runs reported 76 passing tests, 2
ignored helpers, and no failures. Signing, attestation, installation,
deployment, and operations remain unavailable.
