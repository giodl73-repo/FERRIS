# FERRIS profile comparison browser probe

Local, unpublished portability proof for the existing experimental profile-diff
contract. Two supplied JSON records go through the shared Rust core; the UI and
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

The prototype is not publication-ready. Pulse 02 must complete resilient own-file
and original-byte downloads, stale/failure handling, meaningful mobile results,
resource/output limits, independent source/build/legal delivery, final native
regression gates, hosted artifact custody and fresh live verification. It grants
no compatibility, support, freshness, approval, readiness or held-out conclusion.

The probe CLI is a QA transport: a successfully emitted typed envelope returns
process status 0; native semantic status remains in process_exit_code and
result_class. The production ferris/cargo-ferris exit behavior is unchanged.
