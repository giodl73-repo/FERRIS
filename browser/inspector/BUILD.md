# FERRIS profile comparison build and delivery

Rust 1.95.0 (wasm32-unknown-unknown, clippy, rustfmt, rust-docs), wasm-bindgen
CLI 0.2.127, Python 3.11+, Node 22 and Playwright 1.61.1 are the tool versions.
Set CARGO_TARGET_DIR to an absolute directory. browser/inspector/README.md records
the native, companion, WASM and browser commands. build-prototype.py always builds
native and WASM before staging; JavaScript syntax is checked before compilation.

source.zip is the complete exact first-party Git tree identified by provenance,
with every tracked test/include/spec/fixture and both Cargo locks. Source-file
hashes and the Git archive comment bind it to that commit. It is not an offline
vendor archive: Cargo obtains pinned registry packages from the locks. Compile
and replay only the documented public development tests; this download grants
no diagnostic, held-out scoring, support or owner-execution authority.

The optional complete source archive is about 13 MB compressed because FERRIS
retains a substantial public research history. It is not fetched to start the
app. Budgets are separate: 2 MiB initial runtime, 20 MiB optional source ZIP,
25 MiB complete site. Native QA cases, Cargo metadata, screenshots and scratch
requests are never staged as site files. Compiled WASM and Rust source are
separate downloads. Both original FERRIS software licenses and all dependency/
Rust library notices remain intact.

From a clean checkout, after building the prototype:

```text
python browser/inspector/package-site.py
python browser/inspector/verify-site.py target/profile-release/FERRIS --commit <HEAD>
```

Default-branch publication must repeat qualification and verify the transferred
artifact before deploying. Fresh live hashes and browser checks are separate
from a passing PR. Independent source replay must prove response equality;
WASM binary identity across directories is not claimed. Browser resource
admission uses a distinct refusal schema and never manufactures a native result.
