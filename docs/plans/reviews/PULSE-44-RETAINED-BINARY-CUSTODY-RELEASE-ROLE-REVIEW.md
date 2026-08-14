# Pulse 44 retained-binary custody release nine-role review

Date: 2026-08-14
Disposition: Accept public infrastructure-only release
Implementation authority: Public adapter, records, documentation, and
test-only validation only

## Review question

Does Pulse 44 bind one exact retained Pulse 33 binary and receipt into a
durably checked, two-file transactional custody tree without treating the
invalid Pulse 42 summary as evidence or creating diagnostic/product authority?

## Role dispositions

| Role | Disposition | Boundary |
|---|---|---|
| Rust Safety Steward | Accept | The Rust change is a safe test-only validator; the custody adapter is standard-library Python and adds no `unsafe`, product, or execution authority. |
| Compiler Performance Engineer | Accept | The single locked release build is identity evidence, not a speed or reproducibility-performance claim. |
| Interop Boundary Auditor | Accept | Exact filename/platform/cutoff/hash/size checks, regular-file enforcement, duplicate-free receipts, final reconstruction, and explicit sync posture close every public filesystem boundary. |
| AI Assurance Skeptic | Accept | Pulse 42 remains invalid; every failure is terminal and unsummarized as success, while a completed ordered event is unavailable before final `2/2` verification. |
| Ecosystem Strategist | Accept | The adapter reuses immutable public Pulse 33 code and Python standard-library operations; it adds no resolver, registry, network, or product dependency. |
| Rust Maintainer | Accept | The release has a bounded adapter, schema, fixture, 29 focused Python methods, sealed records, and one Rust identity/mutation validator. |
| Native Platform Adopter | Accept | Each staged file fsyncs before close and each directory sync is truthfully `synced` or explicitly `unsupported`; failed final publication rolls back or becomes indeterminate. |
| Scope Keeper | Accept | This is retained-binary custody infrastructure only: no diagnostic, private-data, product, category, fix, or PLATFORM-001 conclusion follows. |
| Validation Checker | Accept | Tests cover retention, receipt identities, unsafe roots/files, all copy/sync/rename/final/rollback fault positions, deterministic summaries, one dirty-checkout rejection, and one clean actual retained-pair publication. |

## Actual-build posture

Two independent zero-retry Windows invocations were recorded. The first
correctly rejected a clone checked out under `core.autocrlf=true` before the
setting changed, because the resulting tracked tree was dirty. The second used
a fresh clone with `core.autocrlf=false` fixed before checkout and published
the exact retained executable/receipt pair `2/2` with one rename, artifact
size `1436672`, and SHA-256
`sha256:0736392a9fab3fa9404554f86e82e1dfcdf9e68e44aa3df8a4cddd1d58a34fd8`.
Neither invocation executed a diagnostic; all runtime roots were removed
after the public-safe result was recorded.

## Bound evidence

The five-payload-file/54158-byte manifest raw/aggregate identities are
`sha256:eae4db6c4add7f20a919cd301dc307cc7845f808f458219b5627c135ed5f0c94` /
`sha256:a22efbbb233ee53550c8ac9771a83af3829c16ce8f7f7a2ff15638adf2f58f94`.
The qualification receipt raw/payload identities are
`sha256:d17ac162d7e8d5afb9f41fa789afe43c2512f2ee1dd30b4afaae4bde16491f1b` /
`sha256:a5a5be3d0832476ba0addb4edda2790d3e02acda49a1266601e6065bc0f9cf29`;
the release seal raw/payload identities are
`sha256:97598062129317e89862407cc00971aa11ac179420088f4d508678b535cab2a8` /
`sha256:4b90c678255fe3567760ce2ef253192a5489ee684ae57a4eb15446f038c189b5`.

## Decision

All nine roles accept the bounded release. The final custody directory is
non-public runtime state and is never committed. No subsequent execution,
diagnostic, product, category, or fix authority is granted.
