---
maxim_schema: maxim.frontmatter.v1
id: maxim:rust-security-assurance:fuzzing-property-testing-and-corpus-management
kind: guide
module: rust-security-assurance
section: security-engineering
title: Fuzzing, Property Testing, and Corpus Management
status: source-custody
source_custody: partial
current_path: rust-security-assurance/10-FUZZING-PROPERTY-TESTING-AND-CORPUS-MANAGEMENT.md
canonical_path: rust-security-assurance/10-FUZZING-PROPERTY-TESTING-AND-CORPUS-MANAGEMENT.md
backsource_ids: [proof-backfill:rust-security-assurance:10-fuzzing-property-testing-and-corpus-management]
concepts: [fuzzing, property testing, corpus management, cargo fuzz, security testing]
root_concepts: [fuzzing]
index_roles: [guide, root-concept]
remap_from: []
remap_to: []
updated: null
---
# Fuzzing, Property Testing, and Corpus Management

Fuzzing is most valuable when the target encodes a security claim rather than
merely "does not crash." Property tests explore structured inputs and shrink
counterexamples; coverage-guided fuzzers mutate bytes toward new execution
paths. A governed corpus connects both techniques to regression testing and
incident response.

## The Big Picture

```
+============================================================================+
|                         ADVERSARIAL TEST LOOP                              |
+============================================================================+
| seeds/generators -> target harness -> oracle/property -> finding           |
|       ^                 |                  |              |                |
|       |                 v                  v              v                |
| corpus merge <---- coverage signal    reject/compare   minimize + triage   |
|       |                                                    |               |
|       +---------------- regression corpus <----------------+               |
+----------------------------------------------------------------------------+
| scope: target + features + toolchain + sanitizer/model + time + hardware   |
+============================================================================+
```

A campaign result is meaningful only with that scope. "Fuzzed" without target,
duration, corpus, configuration, and oracle is not reproducible evidence.

## Choose the Right Test Generator

| Technique | Input model | Best at | Limit |
|-----------|-------------|---------|-------|
| Example test | hand-selected values | known cases and regressions | narrow exploration |
| Property test | generated structured values | algebraic/domain invariants, shrinking | generator may exclude malformed encodings |
| Coverage-guided fuzzing | mutated byte sequences or structured adapter | parser edges, panics, sanitizer/UB findings | coverage is not correctness |
| Differential test | same input to two implementations/modes | semantic disagreement | both may share the same bug |
| Stateful/model test | sequences of commands | protocol/state-machine bugs | state space grows rapidly |

Use layers: byte-level fuzzing for framing/parsing, structured generators for
domain invariants, and stateful sequences for protocol abuse.

## Design a Security-Relevant Harness

```
raw bytes
   |
   +--> enforce harness cap
   +--> call parser without network/filesystem nondeterminism
   +--> if accepted:
          assert canonical round trip
          assert semantic invariant
          compare reference implementation
          bound output/work
```

Good targets are deterministic, fast, side-effect-light, and narrow enough that
the fuzzer reaches depth. Move expensive setup outside the per-input body where
the fuzzing API safely permits it.

| Weak oracle | Stronger oracle |
|-------------|-----------------|
| "did not panic" | accepted value satisfies domain invariant |
| serialize/deserialize returns | round trip preserves canonical meaning |
| parser A accepts | parser A and B agree on meaning and rejection |
| request completes | request stays within allocation/work budget |
| unsafe call returns | Miri/sanitizer also observes no violation on that path |

## `cargo-fuzz` Workflow

`cargo-fuzz` is a third-party Cargo subcommand using libFuzzer and normally a
nightly Rust toolchain. Install an exact approved version in a controlled tool
job and preserve its binary digest. From a crate root, on a host/architecture
supported by that pinned release:

```text
cargo +nightly fuzz init
cargo +nightly fuzz run parse_message -- -max_total_time=60 -timeout=5 -rss_limit_mb=2048
```

`fuzz init` creates or updates fuzz-project files, so run it during target setup,
not in a read-only release job. The final command assumes a generated target
named `parse_message`; its libFuzzer limits are seconds and MiB and remain
platform/tool-version dependent. Record the exact nightly
(`rustc +nightly -vV`) or use a dated nightly in CI. OS, architecture, compiler,
and sanitizer support change; consult the pinned cargo-fuzz release and Rust
Fuzz Book rather than assuming the command works on every target.

Stop conditions should be policy, not human fatigue:

| Campaign | Example stop/evidence rule |
|----------|----------------------------|
| Pull request smoke | fixed short duration per changed high-risk target |
| Nightly | fixed budget; preserve new corpus and findings |
| Release | required cumulative hours/coverage trend with no unresolved blocker |
| Incident/advisory | focused target until patched path and nearby variants explored |
| Continuous service | rolling campaign with deduplicated findings and retention |

Time is not a proof threshold; it is a repeatable investment.

## Property Tests and Shrinking

Properties should state invariants:

- decode(encode(x)) preserves canonical meaning;
- parsing never constructs a value outside declared ranges;
- authorization decision is invariant under irrelevant field ordering;
- resource use stays within declared budgets across generated size classes;
- operations preserve a data-structure invariant;
- rejected input causes no externally visible side effect.

Use `proptest` or another maintained framework under a pinned dependency
version. Persist the minimal counterexample as an ordinary regression test even
if the framework maintains its own failure cache.

## Corpus Management Is Configuration Management

```
seed corpus
   +-- valid representative inputs
   +-- boundary encodings
   +-- historical bugs/CVEs
   +-- cross-version formats
   +-- dictionaries/tokens
        |
        v
merge/minimize -> review sensitivity/license -> version -> distribute to CI
```

| Corpus concern | Policy |
|----------------|--------|
| Secrets/PII | never ingest production samples without approved sanitization |
| Licensing | know whether external samples can be redistributed |
| Size | minimize redundant coverage; keep high-value semantic seeds |
| Reproducibility | store finding input, target, toolchain, features, arguments |
| Security | treat crash inputs as potentially sensitive until disclosure |
| Retention | keep fixed regressions; define expiry for low-value generated bulk |

## Triage and Fix

1. preserve the original artifact read-only;
2. reproduce in the same scoped environment;
3. minimize without losing the security behavior;
4. classify panic, UB, resource issue, semantic flaw, or harness defect;
5. determine affected versions/targets/features and exploit preconditions;
6. patch the root invariant and add regression/property coverage;
7. re-run neighboring targets and update disclosure handling if needed.

## Old World -> New World Bridge

| Established practice | Rust workflow |
|----------------------|---------------|
| QuickCheck/FsCheck | `proptest`/`quickcheck` with shrinking |
| AFL/libFuzzer harness | `cargo-fuzz` target around Rust API |
| Parser corpus in test data | versioned fuzz corpus plus regression fixtures |
| Native sanitizer fuzzing | libFuzzer with target-appropriate sanitizer/Miri follow-up |
| Production crash dump | minimized reproducible input plus scoped build identity |

OSS-Fuzz or Microsoft OneFuzz can provide scalable orchestration where their
supported environments fit. They supplement target/oracle design; a cloud
fuzzer cannot rescue a harness that asserts nothing useful.

## Common Confusion Points

- **"More coverage means secure."** Coverage measures explored structure, not
  correctness or complete paths.
- **"No crash means no bug."** Logic, authorization, and excessive work need
  explicit oracles.
- **"Property testing replaces fuzzing."** Structured and byte-level search find
  different defects.
- **"The corpus is harmless test data."** It may contain secrets, licensed
  samples, or undisclosed exploit triggers.
- **"A fixed fuzzing-hour threshold proves safety."** It records effort, not a
  universal guarantee.
- **"Minimization can replace the original."** Preserve original evidence; the
  minimized input may omit environmental context.

## Decision Cheat Sheet

| Situation | Use |
|-----------|-----|
| Algebraic/domain invariant | Property testing with shrinker |
| Hostile byte parser | Coverage-guided fuzz target |
| Two parsers/implementations | Differential fuzzing |
| Protocol sequence | Stateful/model-based generator |
| Unsafe hotspot | Fuzz plus Miri/sanitizer on supported configurations |
| Production/advisory reproducer | Restricted corpus, regression test, disclosure controls |
| Campaign report | Target, oracle, corpus hash, toolchain, flags, time, findings |

## Primary Sources

- Rust Fuzz Book: https://rust-fuzz.github.io/book/
- cargo-fuzz: https://github.com/rust-fuzz/cargo-fuzz
- libFuzzer documentation: https://llvm.org/docs/LibFuzzer.html
- proptest documentation: https://proptest-rs.github.io/proptest/
- OSS-Fuzz: https://google.github.io/oss-fuzz/
- OneFuzz: https://github.com/microsoft/onefuzz

## Related Guides

- Previous: [09-FFI-NATIVE-LIBRARIES-KERNELS-AND-SANDBOX-BOUNDARIES.md](09-FFI-NATIVE-LIBRARIES-KERNELS-AND-SANDBOX-BOUNDARIES.md)
- Next: [11-MIRI-SANITIZERS-LOOM-MODEL-CHECKING-AND-TARGETED-FORMAL-METHODS.md](11-MIRI-SANITIZERS-LOOM-MODEL-CHECKING-AND-TARGETED-FORMAL-METHODS.md)
- Input targets: [07-PARSING-DESERIALIZATION-INPUT-VALIDATION-AND-PROTOCOL-ABUSE.md](07-PARSING-DESERIALIZATION-INPUT-VALIDATION-AND-PROTOCOL-ABUSE.md)
