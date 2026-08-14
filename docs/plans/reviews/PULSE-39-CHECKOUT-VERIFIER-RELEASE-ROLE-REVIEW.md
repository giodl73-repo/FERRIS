# Pulse 39 checkout-verifier release nine-role review

Date: 2026-08-14
Disposition: Accept public infrastructure-only release
Implementation authority: Public verifier, records, documentation, and
test-only validation only

## Review question

Does Pulse 39 correct the public checkout-root/cwd orchestration ambiguity
without retrying or changing Pulse 38, accessing private custody material, or
adding product or diagnostic authority?

## Role dispositions

| Role | Disposition | Boundary |
|---|---|---|
| Rust Safety Steward | Accept | Rust changes are test-only; no product Rust, unsafe code, FERRIS execution, or diagnostic candidate is added. |
| Compiler Performance Engineer | Accept | Git attribute verification is infrastructure evidence, not a compiler or performance claim. |
| Interop Boundary Auditor | Accept | The verifier fixes the exact `git -C <checkout-root>` boundary and NUL framing; paths are canonical, relative, and contained. |
| AI Assurance Skeptic | Accept | Pulse 38 remains permanently invalid and null-conclusion; no seed, corpus, private custody data, correlation, or inference is exposed. |
| Ecosystem Strategist | Accept | Git remains the attribute authority; no new dependency, registry, network, or product integration exists. |
| Rust Maintainer | Accept | The release contains only a standard-library Python verifier, documentation, public records, and tests. |
| Native Platform Adopter | Accept | The disposable Windows Git 2.55.0.windows.3 `core.autocrlf=true` proof is 36/36 attributes, LF, and zero CR. |
| Scope Keeper | Accept | Exactly 1 `check-attr` invocation and exactly 1 root-anchored read-only Git version probe make 2 total Git processes; there are 0 retries and no fallback check-attr form. |
| Validation Checker | Accept | Tests cover NUL parsing, cwd independence, malformed/unspecified output, path rejection, cardinality, CR, Git failure, deterministic output, cutoff checkout, and Pulse 29’s 76/76 receipt. |

## Bound evidence

The manifest is
`sha256:13d0c322a5e526ca251ec5a402d4d3ddbf94afc2ce6e2b952367f6f9afb8f50c`
with aggregate
`sha256:89d39cf71d7a8d7eb3b27265a6659f953c3e01aed6afb648ca98609b07618d4c`.
The root-cause report is
`sha256:afba07bcfd852a45ae4bfb0956b4e01b6659ae2f91758920baaad4f79c1838bd`;
the qualification receipt is
`sha256:7172813606420a0d2ca9fc2d2d8233ecdd37d2e6e782c86b2d729967f0e554f8`;
and the release seal is
`sha256:aefd9534ab9b5bd95483b496f7b7cb0692da314a3ffbc83cd93c5bc0ae16516c`.
Their payload identities are respectively
`sha256:fcfdf7c44c0f4084a6b6339d43626e67fa7b5a1e3b268c9262ae3587f9a4c5ab`,
`sha256:50be18d56a72508ba5aa0126f2e4a001f6307a0ad761b94e77080604bf7e3546`,
and
`sha256:9a3e30d49db7fa2fd64f7090fc4fac953b676857d08e696e32452f2b8a1c3c9b`.

## Decision

All nine roles accept Pulse 39 as public checkout-verifier infrastructure
only. It preserves Pulse 38’s invalid/no-retry state and adds no diagnostic,
product, fix, certification, support, score, or PLATFORM-001 authority.
