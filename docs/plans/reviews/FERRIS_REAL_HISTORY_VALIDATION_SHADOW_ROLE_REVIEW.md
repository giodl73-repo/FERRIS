# Ferris Real-History Validation Shadow Role Review

Date: 2026-09-09
Stage: post-evaluation closeout
Status: evidence accepted; affected-only execution not promoted

## Evidence reviewed

- Ferris revision: `e66795f4292a9e23a95fffc13a17bece8f5fff61`.
- Completed revisions: the exact 12 BISECT and 28 ICELINES revisions and first
  parents in
  [`2026-09-09-ferris-real-history-shadow.csv`](../../research/evidence/2026-09-09-ferris-real-history-shadow.csv).
- Execution attempts: all five owner lanes in
  [`2026-09-09-ferris-real-history-execution.csv`](../../research/evidence/2026-09-09-ferris-real-history-execution.csv).
- Owner command custody: ICELINES `0be0c480` declares the selected package
  commands in `.github/workflows/ci.yml` and the full
  `cargo test --workspace` reference in both `README.md` and
  `CONTRIBUTING.md`; the workflow's required non-secret environment applies to
  both lanes.
- Result: 40 successful plans, 40 matching repeated plan/change-set identity
  pairs, 37 full fallbacks, three non-fallback plans, two passing ICELINES
  execution lanes, and three failed BISECT lanes.

## Role dispositions

| Role | Disposition | Finding |
|---|---|---|
| Rust Safety Steward | `pass-with-boundary` | No source or safety behavior changed. Passing tests do not establish safety; historical denied-warning failures remain visible. |
| Compiler Performance Engineer | `stop-no-value` | The only narrowed case with passing selected and full lanes was 7.7% slower. Package-unit reduction is not compiler-work or latency evidence. |
| Interop Boundary Auditor | `pass` | Planning, owner execution, and retained evidence remain separate. No workflow provider, ABI, transport, or attestation authority moved into Ferris. |
| AI Assurance Skeptic | `pass-with-limitation` | All 40 repeated plan/change-set identities match and all failed outcomes remain non-success. Full revision-binding identity was not retained. The executable false-negative denominator is only one and cannot support promotion. |
| Ecosystem Strategist | `pass-with-condition` | Existing Cargo and repository commands remain authoritative. Further value work requires better owner declarations and command-shape preservation, not a parallel runner. |
| Rust Maintainer | `pass-with-cost` | The cohort is reproducible from bounded public evidence, but planning overhead and a 92.5% fallback rate make routine affected-only adoption premature. |
| Native Platform Adopter | `pass-with-platform-limit` | Local Windows planning completed. BISECT historical execution completed with denied-warning failures under the current Rust toolchain; no Linux or macOS history execution was attempted. |
| Scope Keeper | `pass` | The work stayed within two public adopters, 40 revisions, three non-fallback cases, and documentation/evidence. No adopter or product behavior changed. |
| Validation Checker | `pass-with-rejection` | The ledgers support the stated counts and timings. They reject affected-only promotion and do not claim full-suite equivalence, prevented iterations, or savings. |
| Product Value Governor | `stop-value-exhausted` | The cohort answered the product question: current mapping falls back 92.5% of the time, and the only narrowed case with two passing lanes was slower. Do not spend another pulse on selector breadth without owner declarations and command-shape changes. |
| Autonomy Supervisor | `stop` | The user-authorized one-shot evaluation and corrective review retries are complete. No follow-on cohort, execution, or implementation is authorized. |

The first nine rows are the required technical role set. Product Value Governor
and Autonomy Supervisor are the additional mandatory wave-control roles.

## Control checkpoint

| Control | Record |
|---|---|
| Product outcome | Decide whether current Ferris affected selection has credible owner-history correctness and latency value. |
| Work completed | 40 revisions planned twice; all three non-fallback revisions received their bounded owner execution attempt. |
| Value obtained | Repeated plan/change-set identities matched; current fallback and command-shape costs were quantified; affected-only promotion was rejected before owner CI changed. |
| Remaining risk | One executable selected/full denominator, no cross-platform history execution, historical BISECT toolchain incompatibility, and 92.5% fallback. |
| Pulses/retries consumed | One authorized pulse; two planning passes; one corrected owner-environment execution pass; review corrections only. |
| Proposed next action | Stop this wave. Seek separate approval only after one adopter supplies broader owner declarations and a shape-preserving selected command. |
| Product Value Governor | `stop-value-exhausted` for further work under this authority. |

## Completed revision disposition

All 40 revisions are closed for planning evidence. The three non-fallback
revisions are closed as follows:

- BISECT `d9550b09`: selected and full failed with different denied-warning
  diagnostics, so timing evidence is inadmissible;
- BISECT `453ca260`: collapsed selected-equals-full lane failed with a
  denied-warning diagnostic, so timing evidence is inadmissible; and
- ICELINES `0be0c480`: selected and full passed; selected was 7.7% slower.

No revision is promoted as savings evidence.

## Remaining gates

1. Separate user approval for any new cohort or owner-command execution.
2. Owner declarations covering root policy, workflows, lockfiles, generated
   files, and documentation before expecting materially lower fallback.
3. An owner-native selected command that preserves full-command compilation
   shape.
4. A frozen executable cohort with alternating cold and warm lane order.
5. Zero selected-pass/full-fail cases across a materially larger denominator.
6. Cross-platform evidence before any platform-general claim.

## Final authority

The evaluation authority is consumed. This closeout grants **no implementation
authority**, no successor pulse, no adopter mutation, no workflow narrowing,
and no support or savings claim. Affected-only execution remains advisory and
unpromoted.
