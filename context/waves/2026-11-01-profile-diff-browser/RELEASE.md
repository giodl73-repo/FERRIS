# FERRIS profile-evidence workbench qualification and delivery

Candidate `d31994a4ec10684cde47e55fbdbedbb9eded27b8` passes hosted
[run 41370203012](https://github.com/giodl73-repo/FERRIS/actions/runs/41370203012).
The complete fifteen-case browser suite passes with zero skips/flaky outcomes
or whole-test retries. All 49 native/WASM responses match. An independently
extracted full source archive outside Git in a directory containing spaces
repeats thirteen behavior cases and all identical requests/responses. Linux
source replay passes 130 core unit tests, six memory/native equivalence tests,
three native CLI regressions and four adapter tests. Two existing subprocess
helpers remain ignored by the parent harness; none was newly ignored. Windows
local core evidence is 131 pass/two existing helpers due to platform test cfgs.
Strict core/companion native/WASM lint and changed-source formatting pass.

The candidate's 89 static files verify after artifact download: 354,038 initial
runtime bytes, an optional roughly 13 MB source archive, 29 Cargo notice groups,
and original Rust library copyright/twelve license notice files. The archive
contains all 2,076 exact tracked source files, including public test/spec includes.
Runtime/source/site budgets are separate (2/20/25 MiB); source is not startup data.
Private QA cases, captures, Cargo metadata and scratch requests are not public
site files. Both original FERRIS software grants remain unchanged. Rebuilt WASM
binary hashes differ; response equivalence is required, not binary identity.

## Reader contract

Compare two explicit experimental `ferris.profile-evidence/v0` records using
existing native semantics. Typed classes, all twelve sections, canonical IDs,
escaped paths, value digests, duplicate/metadata rejection, native 1 MiB and
10,000-change limits, non-executable outputs and limitations remain intact.
Original UTF-8/BOM/CRLF downloads are separate from current text and explicit LF
copies. Only output-visible metadata/keys and value digests appear in reports;
raw section values remain omitted. Native CLI/root Cargo pins and original
public development fixtures remain unchanged from entry `e930612`.

Browser admission has a separate refusal schema, never a fake native result:
24-container depth, 2 MiB combined leaf-path estimate, 4 MiB serialized output,
and 64 MiB WASM linear memory. The UI pages change rows while full downloads
retain every admitted change. Actual five-second file, twelve-second startup
and ten-second computation deadlines, restart and read/revision/worker/request
fencing retain inputs and remove stale exports. Source/build/legal routes are
independently usable under unavailable WASM. Desktop/mobile/keyboard cases pass.
Descriptive 941-byte example timings record browser/user agent and hardware;
they include startup, message/driver and serialization overhead, not build-speed
or comparative performance evidence.

## Failures and correction retained

The generated initial LF regex was invalid JavaScript; source escaping was fixed
and syntax checks now run before builds. Windows CRLF example materialization
and a test's non-admitted key assumptions were corrected without relaxing native
metadata rules. Those failed local reports are retained in QA custody.
The first hosted source replay failed an existing readiness test's unlink/recreate
identity assumption (129 pass/one fail/two helpers). Production uses device/inode
and bytes; immediate Unix inode reuse can make that fixture unchanged. The small
test-only correction creates the replacement while the original exists, asserts
distinct identities and retains both Stale assertions. Production readiness,
identity rules and closed authority records are unchanged. No assertion is
inverted, skipped or rehashed. The failed run 41369228295 remains retained.

## Promotion and authority

PRs qualify read-only; main repeats native, browser, source and artifact gates.
Only main can deploy, with scoped Pages/id-token permissions. Deployment verifies
the exact transferred artifact and source commit again before publication.
This is dated candidate evidence, not certification of all future main revisions.
Check deployed provenance.json, successful default-branch build/deploy and fresh
live bytes/browser evidence for the revision actually published.

No profile generation, Cargo/native owner execution, support, compatibility,
freshness, approval, readiness, platform conformance, held-out scoring/oracle,
sealed diagnostic invocation, or successor authority is introduced. The current
native command catalog is not made browser-compatible by compiling the core.
All-nine-role reviews are AI-simulated, not human or support approval. The existing
product/owner/Cargo/RUNE/Typebook authorities remain separate. Removal is additive:
remove the browser companion and memory entrypoint; native commands stay usable.
