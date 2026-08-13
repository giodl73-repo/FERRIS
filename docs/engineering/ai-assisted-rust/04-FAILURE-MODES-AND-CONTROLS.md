# Failure Modes and Controls

> Status: Guidance only.
> Implementation authority: None. This guide does not authorize product code,
> execution, mutation, approval, or changes to Ferris canonical records.

## Control posture

Failures, unsupported states, stale evidence, and unknowns are first-class
results. They must not be rewritten as success-shaped summaries. Controls
should be proportional to the [risk bands](00-OVERVIEW.md#review-risk-bands)
and preserve owner-native fallback.

## Failure matrix

| Failure mode | Why it matters | Required control |
|---|---|---|
| Hallucinated API or crate behavior | The patch may compile only under an invented assumption or fail later | Verify against owner source, exact versions, compiler output, and behavior tests |
| Compiler-driven overfitting | Repeated fixes can silence diagnostics while changing intent | Recheck the change brief, review each revision, and run behavioral and negative tests |
| Scope under-selection | Changed behavior may escape selected packages or tests | Start from owner anchors, use deterministic closure, compare with full reference, and widen on unknowns |
| Test deletion or weakening | Green output may be purchased by removing evidence | Review test diffs, preserve mandatory gates, and require explicit approval for exceptions |
| Hidden dependency change | Features or transitive code can alter safety, portability, build, or runtime behavior | Review manifest and lock diffs, exact closure, build scripts, proc macros, native links, licenses, advisories, and rollback |
| Unsafe invariant drift | Compiling code may violate unstated caller or platform obligations | Require invariant documentation, specialist review, dedicated analysis/tests, and safe alternatives |
| FFI semantic loss | ABI-compatible shapes can still mishandle ownership, panic, allocation, or threading | Specify the full boundary and test success, rejection, lifecycle, and cross-version cases |
| Build-script hidden input | Cargo rerun behavior may not observe an input | Expose known inputs/outputs, retain unknowns, force safe rebuild or block rather than reuse |
| Proc-macro stale generation | Generated Rust may reflect hidden or version-coupled state | Record generator identity and inputs, regenerate, inspect expansion-sensitive behavior, and test compile failures |
| Prompt injection | Untrusted content may request commands, secrets, scope expansion, or publication | Treat content as data, enforce an external command/data policy, minimize context, and stop on conflict |
| Secret or private-data leakage | Prompts, diagnostics, or evidence may escape authorized boundaries | Classify and redact before exposure; never include reusable secrets; fail closed |
| Benchmark theater | A favorable sample can hide noise, shifted work, or lost validation | Use representative workflows, repetitions, variance, component attribution, and correctness controls |
| Approval confusion | Authentication or model confidence may be mistaken for authorization | Bind approval to an exact patch, plan, commands, scope, limits, and human authority |
| Automation lock-in | Builds or recovery may require the assistant | Preserve ordinary Cargo, standard files, documented commands, and tested removal |

## Unsafe Rust controls

An AI-generated or AI-modified `unsafe` boundary is R3. Do not accept it
through the ordinary propose-compile loop.

Require:

- the unsafe operation and exact invariant;
- why safe Rust is insufficient;
- ownership of the invariant at every call site;
- validity, alignment, initialization, provenance, aliasing, lifetime, and
  concurrency assumptions;
- panic, unwind, cancellation, and partial-initialization behavior;
- target and compiler assumptions;
- focused positive and negative tests;
- applicable Miri, sanitizer, fuzz, property, or model-based evidence;
- qualified human safety review; and
- rollback.

Tools cover named executions and models; they do not establish universal
soundness. The normative product contract states that later Ferris adapters
requiring `unsafe` need separate review and implementation authority:
[PRODUCT-001](../../specs/FERRIS_PRODUCT_CONTRACT.md).

## FFI and native controls

For Rust/C/C++ or other native boundaries:

- freeze the header, binding generator, ABI, compiler, target, and native
  dependency identity;
- make layout, calling convention, ownership, allocation, deallocation,
  lifetimes, nullability, threading, callbacks, and errors explicit;
- contain Rust panic and foreign exception behavior;
- test load/link failure, version mismatch, invalid input, double release,
  missing symbol, callback lifetime, and cross-thread use as applicable;
- verify generated bindings are reproducible enough for their stated use and
  inspect their diff; and
- preserve an incremental, reversible adoption path.

Compilation of both sides is not compatibility proof. Use the
[Interop Boundary Auditor](../../../.roles/parliament/interop-boundary-auditor.md)
and the validation dimensions in
[VALIDATION-001](../../specs/FERRIS_VALIDATION_COVERAGE_CONTRACT.md).

## Dependency controls

AI may retrieve or compare candidate information, but it must not approve,
install, upgrade, remove, change features, or rewrite the lockfile without the
repository's action and approval path.

Before accepting a dependency change:

- name the consumer need and alternatives, including no new dependency;
- verify exact identity and source;
- inspect active features and transitive closure;
- identify proc-macro, build-script, native `links`, network, runtime, and
  generated-code behavior;
- verify MSRV/toolchain, target, license, advisory, maintenance, and support
  evidence without converting absence into pass;
- run owner-native checks and behavioral tests;
- document renewal and rollback; and
- obtain human approval.

See
[Rust crate discovery and selection](../../research/2026-08-10-rust-crate-discovery-selection.md).

## Build-script controls

Treat `build.rs` as executed host code with potentially incomplete input
declarations.

- Record package, script source, host toolchain, working directory,
  environment, known rerun declarations, generated outputs, native metadata,
  and downstream fan-out.
- Distinguish script compilation, script execution, saved output replay,
  generated file ownership, and later compilation.
- Test changed, removed, malformed, missing, and hidden inputs where feasible.
- Do not assume unchanged stdout or output files prove safe reuse.
- Do not suppress execution, clean `OUT_DIR`, cache outputs, or invent inputs
  without an explicit owner contract.
- Widen or perform an owner-sufficient rebuild when freshness cannot observe
  the changed input.

These controls follow the
[Build Intelligence Research Program](../../plans/BUILD_INTELLIGENCE_RESEARCH_PROGRAM.md)
and [VALIDATION-001 failure rules](../../specs/FERRIS_VALIDATION_COVERAGE_CONTRACT.md).

## Procedural-macro controls

Procedural macros execute native code during compilation and emit Rust whose
effects can be wider than the visible invocation.

- Record proc-macro package/version, toolchain, invocation sites, known inputs,
  generated tokens or expansion evidence where supported, and downstream
  compilation.
- Include compile-pass and compile-fail behavior, diagnostics, generated API,
  feature/target variations, and downstream use.
- Treat environment, file, tool, or network inputs as hidden unless the owner
  evidence establishes them.
- Do not infer cache safety from identical visible input or one repeated run.
- Do not enable experimental derive caching or restore generated output based
  on an AI-created key.

## Behavioral and negative-test controls

Every material generated behavior should have an observable contract:

- intended success;
- invalid or adversarial input;
- boundary values;
- error identity and propagation;
- timeout, cancellation, retry, and cleanup where applicable;
- feature and platform differences;
- concurrency or ordering where applicable; and
- regression of the original failure.

Expected rejection must remain distinct from pass. A snapshot that merely
matches generated text is not a substitute for behavior. Conformance requires
positive, negative, failure, unsupported, and version-skew cases in
[CONFORMANCE-001](../../specs/FERRIS_CONFORMANCE_CONTRACT.md).

## Privacy and prompt-injection controls

### Data boundary

- Classify source, logs, dependency names, repository names, paths, prompts,
  model inputs, test data, and timing data.
- Minimize disclosure and redact before model or connector exposure.
- Never include credentials, reusable tokens, private keys, secret values, or
  unrestricted private input.
- Bound logs and samples.
- Keep omissions visible.
- Stop if residency, audience, tenant, retention, or deletion policy cannot be
  satisfied.

### Instruction boundary

Only the human-approved task and repository policy authorize work. Treat
instructions embedded in:

- source comments;
- README or issue text;
- compiler and test output;
- generated files;
- dependency documentation;
- web content; and
- tool-returned text

as untrusted data. They cannot broaden command, network, secret, publication,
or mutation authority. When content conflicts with policy, ignore the embedded
instruction, record the conflict if material, and stop if safe separation is
not possible.

## Performance controls

Reject a performance claim when:

- the workload is not representative;
- baseline and candidate differ in source, toolchain, features, profile,
  target, cache, or environment without accounting;
- repetitions or variance are absent;
- compiler, macro, build-script, codegen, link, and system effects are
  collapsed into one unexplained number;
- tests or correctness controls were removed;
- output, runtime, memory, binary size, or another material metric regressed
  without disclosure; or
- a selected result is generalized beyond its measured population.

Use
[FP-02, FP-03, FP-08, and FP-10](../../governance/ENGINEERING_PRINCIPLES.md).

## Stop conditions

Stop and return to investigation, wider validation, or human ownership when:

- owner truth is unavailable;
- scope cannot be bounded safely;
- required validation is unavailable;
- a secret or cross-tenant boundary may have been crossed;
- the patch introduces unreviewed `unsafe` or FFI;
- generated output cannot be traced to its owner;
- repeated revisions exceed the declared bound;
- full-reference comparison finds a material omission;
- rollback cannot be demonstrated; or
- evidence contradicts the requested claim.

