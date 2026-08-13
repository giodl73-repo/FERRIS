# Boundary and Ownership

> Status: Guidance only.
> Implementation authority: None. This guide does not authorize product code,
> execution, mutation, approval, or changes to Ferris canonical records.

## Boundary rule

AI assistance operates at the proposal and explanation boundary. Deterministic
owner evidence, repository policy, and accountable humans establish accepted
scope and decisions. Ferris coordinates owner truth; it does not create it.
See the [seven-program architecture](../../plans/FERRIS_SEVEN_PROGRAM_ARCHITECTURE.md)
and [product contract](../../specs/FERRIS_PRODUCT_CONTRACT.md).

## Ownership table

| Subject | Owner authority | Permitted AI assistance | Prohibited inference |
|---|---|---|---|
| Source revision and change set | Git and repository maintainers | Summarize or propose a diff | Inventing the baseline or treating uncommitted context as complete |
| Packages, features, targets, units, freshness | Cargo | Explain observed metadata or command output | Replacing Cargo resolution or declaring a unit fresh |
| Rust language acceptance | rustc | Explain diagnostics and propose revisions | Claiming behavior, soundness, or safety from compilation |
| Tests and coverage | Repository test owners and test systems | Propose tests and explain results | Equating selected passing tests with complete validation |
| Dependencies | Maintainers, Cargo, registries, policy and assurance owners | Retrieve candidates and organize attributed evidence | Approving a crate from rank, popularity, one advisory scan, or AI preference |
| `unsafe` invariants | Code owners and qualified safety reviewers | Identify blocks and ask for invariant evidence | Adding or changing `unsafe` under an ordinary low-risk workflow |
| FFI and ABI | Native and Rust boundary owners | Map explicit boundary obligations and propose fixtures | Assuming C-shaped types preserve ownership, panic, threading, or lifetime semantics |
| Build scripts and proc macros | Cargo/rustc plus package owners | Expose execution, inputs, outputs, generated source, and fan-out | Treating generated output as pure, deterministic, or safely reusable without evidence |
| Performance | Workload owner and measurement protocol | Form hypotheses and interpret measured components | Claiming improvement from one wall-time sample or microbenchmark |
| Policy and approval | Named organizational principals | Present the plan and evidence | Authenticating, authorizing, approving, or extending approval |
| Deployment and operations | Deployment provider and service owners | Propose an action plan | Mutating an environment or claiming rollout success |

The product boundary is normative in
[PRODUCT-001](../../specs/FERRIS_PRODUCT_CONTRACT.md). Authentication,
authorization, connector permission, and approval remain distinct under
[GOVERNANCE-001](../../specs/FERRIS_ENTERPRISE_GOVERNANCE_CONTRACT.md).

## Deterministic scope authority

Scope must begin with owner-native anchors such as the repository revision,
Cargo workspace/package/target, exact activity, validation gate, native input,
contract, application, and policy boundary. Keep package, target, activity,
compilation, artifact, runtime, validation, deployment, and omitted scope
separate.

AI may propose:

- a mapping from changed inputs to owner scopes;
- an explanation of likely effects;
- a candidate exclusion or narrower closure; and
- missing tests or evidence to request.

AI may not make the proposal authoritative. Acceptance requires deterministic
owner evidence or explicit human/policy approval. Unknown, expired,
conflicting, unsupported, or failed mappings widen in the order defined by
[SCOPE-001](../../specs/FERRIS_SCOPE_CONTRACT.md); they do not become empty
scope. The research basis is
[Blueprint cross-command scope](../../research/2026-08-10-blueprint-cross-command-scope-model.md).

## Human approval

Human approval is meaningful only when the reviewer can see:

- the exact source baseline and patch;
- the requested outcome and non-goals;
- the risk band and affected owner boundaries;
- exact commands, toolchain, target, features, profile, and environment;
- selected and full-reference validation;
- failures, omissions, unknowns, and unavailable evidence;
- dependency, generated-code, unsafe, FFI, privacy, and performance effects;
- rollback and removal steps; and
- changes made after earlier review.

Material plan or patch changes invalidate the prior approval boundary. A model,
agent, CI identity, or connector cannot approve on behalf of a human merely
because it is authenticated. See
[GOVERNANCE-001 approval](../../specs/FERRIS_ENTERPRISE_GOVERNANCE_CONTRACT.md).

## Risk-band escalation

Use the [series risk bands](00-OVERVIEW.md#review-risk-bands). Escalate when:

- a public API or semantic contract changes;
- the patch changes dependencies, features, profiles, target configuration,
  lock state, build scripts, procedural macros, or generated source;
- `unsafe`, FFI, ABI, allocation, panic, exception, threading, or lifetime
  behavior is present;
- a security, privacy, compliance, performance, compatibility, or removal
  claim is material;
- tests are unavailable, flaky, selected-only, or unable to observe the
  changed behavior; or
- ownership or scope is unknown.

Escalation means broader evidence and a qualified reviewer, not greater AI
autonomy.

## Boundary-specific review questions

### Safe Rust

- What behavior changes even though the code remains safe Rust?
- Which invariants are enforced by types, and which remain external?
- Which error, cancellation, concurrency, persistence, and resource behaviors
  need negative tests?

### `unsafe`

- What exact invariant permits the operation?
- Who owns that invariant across all callers and platforms?
- Which aliasing, initialization, provenance, lifetime, panic, unwind, and
  concurrency conditions were tested or analyzed?
- Is there a safe design that avoids the boundary?

Compiler acceptance is not a safety proof. Apply the
[Rust Safety Steward](../../../.roles/parliament/rust-safety-steward.md) lens
and require a separately governed high-risk path.

### FFI and native boundaries

Record language, ABI, calling convention, layout, ownership, allocation,
deallocation, nullability, lifetime, panic/exception, threading, callback,
version, and error semantics. Test both success and intentional misuse or
rejection. The
[Interop Boundary Auditor](../../../.roles/parliament/interop-boundary-auditor.md)
requires reversibility and actionable failure evidence.

### Dependencies

Discovery and selection are different decisions. Candidate retrieval may use
Cargo, crates.io, documentation, curation, advisories, and policy tools, but
selection must name consumer requirements, exact release, source, features,
dependency closure, platform/toolchain evidence, alternatives, expiry, and
rollback. Unknown declarations remain unknown after a passing build. See
[crate discovery and selection](../../research/2026-08-10-rust-crate-discovery-selection.md).

### Generated code, build scripts, and proc macros

Generated output is not self-authorizing source. Identify the generator,
invocation, inputs, outputs, owner, toolchain, checked-in status, regeneration
path, and negative cases. Build scripts and proc macros execute code and may
have hidden or environment-sensitive inputs. A stable generated diff does not
prove deterministic generation or safe reuse.

### Performance

Name the consumer workflow, baseline, hardware, OS, filesystem, toolchain,
cache state, target topology, repetitions, variance, and limitations. Separate
check, build, test, codegen, and link. A faster result that drops validation or
changes runtime behavior fails. See the
[measurement contract](../../specs/BUILD_LATENCY_MEASUREMENT_CONTRACT.md).

## Ordinary Cargo preservation

An AI-assisted workflow must leave maintainers able to:

- run documented Cargo and repository commands directly;
- use rust-analyzer and editors without a required AI service;
- inspect standard manifests and lockfiles;
- reproduce failures without a model session;
- revert the patch with normal version control; and
- remove assistance without source changes needed to restore correctness.

Do not introduce hidden manifests, parallel dependency resolution, opaque
target reuse, mandatory writable shared target directories, or AI-owned
correctness state. These are adoption and removal obligations in
[PRODUCT-001](../../specs/FERRIS_PRODUCT_CONTRACT.md).

## Exit check

Before accepting a proposal, the maintainer should be able to say:

1. Which owner established each input and result?
2. Which statements are observed, inferred, proposed, approved, or unknown?
3. Why is scope no narrower than the evidence permits?
4. Which specialist owns every R2 or R3 boundary?
5. Which ordinary owner-native command reproduces the evidence?
6. How is the change rejected, reverted, or removed?

