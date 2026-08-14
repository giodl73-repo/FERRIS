# Pulse 37 Public-Artifact Checkout Normalization Nine-Role Review

Date: 2026-08-14
Disposition: Accept Git-clean release rebinding
Implementation authority: Public release metadata, evidence, documentation,
and test-only validation only

## Review question

Does Pulse 37 rebind the current Pulse 35 public release to exact LF
Git-clean bytes while preserving the original cutoff identities and immutable
Pulse 36 invalid result, without semantic source change, diagnostic execution
or authority, qualification rerun, or product change?

## Role dispositions

| Role | Disposition | Boundary |
|---|---|---|
| Rust Safety Steward | Accept | Test-only Rust normalizes checkout framing in memory and adds no production Rust or unsafe boundary. |
| Compiler Performance Engineer | Accept | Clean-filter checkout/hash validation is reproducibility evidence, not a build or performance claim. |
| Interop Boundary Auditor | Accept | The exact anchored LF attribute and resulting-index checkout preserve public bytes across the Git/Windows boundary. |
| AI Assurance Skeptic | Accept | Historical Pulse 35 identities and the Pulse 36 2/8-versus-6/8 invalid result remain visible; normalization is not diagnostic success. |
| Ecosystem Strategist | Accept | Ordinary Git attributes and a disposable index add no package manager, registry, network, credential, or external owner authority. |
| Rust Maintainer | Accept | The patch is removable release metadata, documentation, and integration validation; `crates/ferris-cli/src` remains unchanged. |
| Native Platform Adopter | Accept | Windows `core.autocrlf=true` clean-filter materialization is directly bound without claiming native Linux support. |
| Scope Keeper | Accept | This one bounded rebinding creates no retry, candidate, fix, score, certification, or PLATFORM-001 authority. |
| Validation Checker | Accept | Cutoff blobs prove six exact CRLF-to-LF deltas, two unchanged envelopes, current manifest/seal identities, and 8/8 resulting-index bindings. |

## Shared findings

All roles retain the historical cutoff manifest
`sha256:9baef3aa3030d7e8261072b26e7bd40436c362163f9138f929f0e4264fd0289b`,
aggregate
`sha256:585f0caf7aa4cbe821a71dcb60e5a1b7d6ad0650677b715dcbf143456612a0d7`,
total `405414`, and release-seal raw/payload
`sha256:51edf2f2df9210291705332fa8a4c3b55cb2a19a1aff22ecd882434a5ebefef2` /
`sha256:5b5e4383ffe5274f36f355069a5339c1684674aea342229f54f63ef247d21e52`.

The normalized successor is manifest
`sha256:f30e6dabeb43a835855da4cfa757858d03ff00a3e1c7ad101fced6150915b7e1`,
aggregate
`sha256:f61e0261ac589660ac3b2e950a3267ac7dfc4a1aea2db6bb654b40558318ff69`,
total `403316`, and release-seal raw/payload
`sha256:17459123c674f2664d7d09ea03c00dcba72129bb1cf532cfe11f8cf4edeffd23` /
`sha256:834781867ea008dc14a54d7b811002ee1b8fa759c0b1d7f32432ea6c0d5c5375`.
The receipt identity is
`sha256:e312d8265c406c6330d537e24913168508cab6dd40018bcb36bbbc1e2116bfae`.

The receipt seals the existing release-root `text eol=lf` rule, a disposable
Windows resulting index with `core.autocrlf=true`, 8/8 file size/hash
bindings, six LF text files with zero CR bytes, and the two unchanged JSON
envelopes. It records no diagnostic execution, authority, candidate,
qualification rerun, private-data access, or product-file modification.

## Decision

All nine roles accept this public-artifact normalization only. Pulse 36
remains permanently invalid, non-retryable, and null-conclusion; Pulse 37
does not reopen it or change PLATFORM-001.
