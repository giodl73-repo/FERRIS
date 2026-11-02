# Pulse 01 - local portable profile comparison

Status: local portability qualified. Product outcome: compare two explicit
experimental profile inputs in real browser WASM using the same parser,
canonicalizer and diff as native FERRIS. This does not publish a Pages release.

The file adapter still owns metadata/open/read failures and native path-bound
identities. A shared byte parser and comparator replace duplicated work; the
new memory entrypoint returns the existing command-result envelope. Success
and parsed comparison failures match native identities. Parse/admission failures
use a domain-separated ordered byte-request identity, with no invented paths.
Schema, duplicate/metadata rejection, twelve sections, change/size limits,
non-executable records and omission of raw section values remain intact.

Fourteen original profile-diff tests and six new integration tests pass. The
latter compare all nine public development pairs and a parsed identity mismatch
to native file-based envelopes, and cover formatting identity, failure input
binding, classes, redaction, escaping and blocked-over-limit output. The full
core unit run passes 131 with two existing ignored subprocess helpers retained.
Three local Chromium cases pass, including 49 exact native/WASM responses and
both sides of the 10,000-change boundary. See portability.json for artifact and
scope. Compiler acceptance is separate from these behavior checks.

One bounded portability repair closed native-only dead-code and explicit-file-
drop lints on wasm32. The six cfg attributes retain Windows/Unix bodies, add no
unsafe code and change no observation policy. Strict core and companion host/
WASM lint pass. Root Cargo files and native CLI are unchanged. Existing unrelated
CLI formatter-only changes were restored rather than added to this pulse.

## Value checkpoint before Pulse 02

- Outcome/value obtained: a real pure Rust browser comparison without native
  installation or owner processes, with canonical native equality.
- Budget: Pulse 01 completed inside its 90-minute cap; one portability correction,
  zero sealed diagnostic/authority attempts, no successor chain.
- Remaining risks: browser file/original-byte flow, worker deadlines/restart/stale
  fencing, clear non-success and metadata visibility, deeply nested input/output
  amplification, native regression breadth, source/legal closure and hosted/live
  delivery. Do not claim support, freshness, approval or platform readiness.
- Next action: complete the already-budgeted single browser/delivery pulse using
  the existing portfolio pattern; no new custody architecture. Explicitly label
  any adapter depth/output bounds without changing the native contract.
- Product Value Governor: continue-within-budget for Pulse 02, capped at 120
  minutes plus at most one corrective product/delivery repair. If scope changes
  layers, two attempts are invalid or infrastructure eclipses the product, stop
  with retained evidence; do not invent a native-only exception.
- Autonomy Supervisor: follow the recorded scope; no closed authority is revived.

Review follow-up is AI-simulated. No human, held-out or support approval is claimed.
