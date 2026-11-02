# FERRIS profile-evidence browser workbench

Browser companion for the existing experimental profile-diff contract. Two supplied JSON records go through the shared Rust core; the UI and
wasm-bindgen adapter do not implement another diff or resolve Cargo truth.
All twelve sections, strict metadata/duplicate validation, canonical identities,
JSON Pointers, value digests and non-executable limitations retain native meaning.
Raw section values are omitted from diff output; identifiers and keys remain
output-visible. Native file commands and root dependency pins are unchanged.

The byte entrypoint is additive. Its parse/admission error identity binds both
ordered explicit byte inputs; native file errors retain their original path
binding. Successful results and content-based comparison errors match exactly.
Browser compilation of the core does not make its native owner commands usable
or authorized in the browser. No profiles are generated and no owner tools run.

## Reproduce Pulse 01

Rust 1.95.0, wasm-bindgen CLI 0.2.127, Node 22, Playwright 1.61.1 and Python 3.11+.
Set CARGO_TARGET_DIR to an absolute build directory of your choice. From repo root:

```text
cargo +1.95.0 test -p ferris-core profile_diff --locked
cargo +1.95.0 test -p ferris-core --lib --locked
cargo +1.95.0 clippy -p ferris-core --locked --lib -- -D warnings
cargo +1.95.0 clippy -p ferris-core --locked --target wasm32-unknown-unknown -- -D warnings
python browser/inspector/build-prototype.py --bindgen /absolute/path/to/wasm-bindgen
```

The build uses its committed standalone lock, seeded from native versions,
builds the native protocol and WASM, stages only the local prototype and records
49 native response cases. The protocol accepts strict before/after string fields
on UTF-8 stdin with a 12 MiB + 1 KiB framing bound, sufficient for JSON escaping
of two 1 MiB input strings. Individual input/change bounds remain native-core rules.
Then from browser/inspector: `npm ci`, `npx playwright install chromium`,
`npm run test:pages`. The server uses /FERRIS/ on local port 4192.

The workbench includes resilient own-file/original-byte flows, typed mobile
results, worker deadlines/stale fencing and independent source/build/legal
delivery. Hosted qualification and fresh live verification remain distinct;
see the dated child wave release record and current deployment provenance. It grants
no compatibility, support, freshness, approval, readiness or held-out conclusion.

The probe CLI is a QA transport: a successfully emitted typed envelope returns
process status 0; native semantic status remains in process_exit_code and
result_class. The production ferris/cargo-ferris exit behavior is unchanged.


## Pulse 02 author flow and release candidate

The local author flow now keeps both opened originals, current texts and explicit
LF copies separate, with atomic paired example loading. Typed native classes,
canonical references, paged change paths/digests and complete report downloads
remain distinct from browser refusals. Worker/read deadlines (12/5/10 seconds),
restart, revisions/request IDs/generations and file tokens fence stale work.
Thirteen local browser cases pass, including all 49 original native/WASM reports.
The first generated LF regex was invalid JavaScript; syntax checks now precede
builds. Two retained failed reports also record the Windows fixture/test-key
assumptions corrected before the successful run. Native metadata was not relaxed.

Browser admission is additive: maximum 24 containers, 2 MiB combined leaf-path
budget and 4 MiB reports. The WASM linker caps linear memory at 64 MiB; a real
browser growth test enforces the ceiling. These are resource refusals under a
separate browser schema, not fabricated native comparison results. Four new
adapter tests pass. The native 1 MiB / 10,000-change contract is unchanged.

Source/build/legal packaging is now authored but still requires its candidate
checks and independent source replay. From a clean checkout after a fresh build:

```text
python browser/inspector/package-site.py
python browser/inspector/verify-site.py target/profile-release/FERRIS --commit <HEAD>
```

The default local browser server now serves target/profile-release. Set
FERRIS_SITE_DIR to ../../target/profile-prototype only for behavior checks that
explicitly exclude delivery cases. Publication still requires hosted/native
regression, exact transferred artifact custody and fresh live verification.


Independent source replay, after clean-source staging:

```text
python browser/inspector/verify-source.py --bindgen /absolute/path/to/wasm-bindgen
```

It extracts the delivered complete archive outside Git in a path containing
spaces, reruns the public core unit/memory tests, the three CLI profile-diff
regressions and four adapter tests, then rebuilds and repeats thirteen browser
behavior checks and all 49 requests/responses. The two exact-artifact delivery
checks remain separate in the full fifteen-case suite. Platform-specific core
unit counts and the two existing ignored subprocess helpers are recorded.
Original/rebuilt WASM hashes are retained without a binary-identity claim.


## Qualified candidate

Candidate d31994a passes hosted run 41370203012 and full post-download artifact
verification. Its Linux source replay passes 130 core tests (two existing ignored
helpers), six memory tests, three native CLI regressions, four adapter tests and
thirteen browser behavior cases, with all 49 requests/responses identical. The
hosted complete-site suite has fifteen cases. Default-branch build/deploy and
fresh live checks identify the actual published revision separately.
