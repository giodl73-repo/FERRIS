# Pulse 51: Public diagnostic-executor release

Status: Complete public prelaunch infrastructure only

Implementation authority: public standard-library release source, synthetic
fixtures, documentation, review, and test-only validation only.

## Goal

Close every public prelaunch implementation blocker found after Pulse 50
without modifying any frozen predecessor or exercising an authority. Pulse 51
does not cure, execute, or extend Pulse 50. Pulse 50 remains
`authorized-unexecuted`, blocked, and launch unconsumed pending governance
closeout or a separately authorized successor.

## Fixed boundary

Pulse 51 MUST NOT:

- authorize or execute a Pulse 50/Pulse 51 diagnostic;
- create a private seed, descriptor corpus, case material, candidate, result,
  or witness root;
- invoke a FERRIS binary except public fake executables in synthetic
  qualification;
- alter P27, P31, P33, P35, P37, P43, P44, P45, P47, or Pulse 50 artifacts;
- publish descriptor/case/order/profile identifiers, paths, seed material,
  binary bytes, or private runtime records; or
- claim a product, category, score, support, certification, fix, or
  PLATFORM-001 conclusion.

## Released control

The sealed release at
[`pulse-51-diagnostic-executor-release`](../../../../docs/simulations/profile-diff-held-out/pulse-51-diagnostic-executor-release/README.md)
contains:

1. complete P35/P37 custody verification of all ten P35 release-tree files
   and the P35 machine schema, including raw-checkout variants, canonical LF,
   and exact Git-clean bytes;
2. a standalone P31 exact-contract verifier bound to frozen document, schema,
   six fixtures, and 33 mutations rather than repository inventory or the
   historical Rust test;
3. verified exact P27 `run_exact_two_cycle(cycle_root)` loading and one-call
   output validation, never P27 qualification CLI use;
4. direct P33 raw executable/receipt verification for Windows and Ubuntu,
   including expected size/hash, logical filename, checkout/build controls,
   and exact recorded toolchain;
5. canonical P44/P45/P43 platform mapping, with WSL mapped internally to
   `ubuntu-24.04-x86_64` before a public event can exist, sealed P45 invoked
   once from exact P44 summaries/final roots, and complete bridge identities
   checked;
6. an in-memory executor that validates exact P35 70/69/1 topology,
   descriptor order, request contract, declared-private-root confinement,
   full frozen output contracts including independently recomputed
   profile-diff semantics and all four frozen output identities, native Windows argv, and exact
   `Ubuntu-24.04` WSL argv/path translation; and
7. imported sealed P43/P45/P47 dependencies and a single separately invoked
   Pulse-47 terminal helper. Qualification does not invoke terminal
   publication.

The runner launches only 69 `launch-ready` descriptors per canonical platform
after external governance has established authority. It does not accept a
forgeable grant/trust mode or caller-created gate event; authority verification
is outside runtime infrastructure. Its final ordinal-70 no-launch descriptor
creates no process. Cross-platform comparisons use only the frozen public
process-exit semantic projection, while full normalized hashes remain private.
The only returned execution data are a caller-private in-memory record and a
bounded P43-valid catalog and event list.

## Synthetic qualification

The release executes at least 20 isolated cycles using public generated
synthetic descriptors and a fake executable only. It covers LF/CRLF custody,
all P31 mutations, P27 callable binding, 70/69/1 and 140/138/2 accounting,
exact native/WSL argv/cwd/path translation and confinement, 69 fake
launches/platform, final zero-process no-launch, Windows/Ubuntu mapping,
P33 hash and toolchain mutations, complete P44/P45 summary/identity
mutations, P27 partial-root cleanup, full output grammar, first-mismatch stop,
P43 privacy/event validation, terminal-root one-use controls, and documented
Python resolver precedence. Python bytecode is disabled for documented
commands; all qualification and test scratch is outside the sealed release
tree and is deterministically removed. A shared synthetic/test-only cleanup
helper retries `PermissionError` or Windows `WinError 32` on the fixed
0.02/0.05/0.10/0.20-second schedule, proves absence, and otherwise fails
explicitly; it does not alter immediate fail-closed P27 runtime cleanup.

No Pulse 50/Pulse 51 runtime artifact, public terminal root, private seed,
private descriptor, candidate, or FERRIS diagnostic exists as a result of this
pulse.
