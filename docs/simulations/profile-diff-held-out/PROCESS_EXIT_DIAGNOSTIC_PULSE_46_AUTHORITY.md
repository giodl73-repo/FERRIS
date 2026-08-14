# Independent Pulse 46 Process-Exit Diagnostic Authority

Status: Authorized, unexecuted
Program:
`FERRIS-P46-INDEPENDENT-PROCESS-EXIT-DIAGNOSTIC-PUBLICATION-ORDER-AUTHORITY`
Schema: `ferris.process-exit-diagnostic-pulse-46-authority/v1`

## Boundary

Pulse 46 is one new independent authority at immutable cutoff
`22ea38e274b882d6e607810382f842b76e483f10`. It is not a Pulse 42 retry,
resume, reconstruction, reseed, reuse, correlation, or inference. Pulse 42
remains permanently `invalid-publication-integrity`, non-retryable, and
null-conclusion. The valid Pulse 17 first score remains immutable: it failed
only `process-exit-agreement`, and retry, rescore, reuse, and private-material
access remain prohibited.

Authoring this authority executes no custody, build, FERRIS candidate,
diagnostic, search, or private-data operation. It changes no product behavior,
score, certification, support, fix, or PLATFORM-001 conclusion.

## Immutable public custody

Before ordered execution, a custodian MUST independently verify the cutoff and
the complete exact current-cutoff release trees for Pulse 41, Pulse 39, Pulse
43, Pulse 44, and Pulse 45, including each sealed manifest, qualification
receipt, release seal, every raw file binding, and the declared release-tree
path set. Pulse 43 is the release committed at
`83c8c4a83e8962c90fe5bd80fd7181c565284ba1`; Pulse 44 is the release committed
at `ffa1cff179acae8ae7c8cc831e7734ba6a558126`; the cutoff itself seals Pulse
45. The declaration fixes their full identities.

The immutable cutoff identity is frozen once. Each platform then receives its
own fresh cutoff checkout with `core.autocrlf=false` fixed **before** checkout.
The platform work roots, retained executable/receipt final roots, seed, and
materialization outputs are controlled runtime state: they are never public
artifacts or committed files.

Pulse 41/Pulse 39 public custody executes once under the existing
transactional-copy and checkout-verifier controls. It is the first ordered
gate. No execution may begin until its public-release tree verification has
completed.

## Closed event catalog

The only ordered execution catalog is:

1. `pulse-41-pulse-39-public-custody`
2. `windows-retained-binary-custody`
3. `ubuntu-retained-binary-custody`
4. `exact-adapter-preflight`
5. `pulse-31-public-input`
6. `pulse-35-pulse-37-normalization`
7. `private-materialization`
8. `bounded-process-exit-search`

Records classified `public-artifact-self-validation` are separate from
`ordered-execution`: public manifest, release, schema, hash, and count checks
cannot advance an ordered gate. A stop prohibits all later ordered execution;
all later gate counts are indeterminate rather than execution. After a stop,
self-validation may be recorded only when this authority explicitly classifies
and permits it as public self-validation.

For each platform, Pulse 45 is invoked exactly once from the fresh cutoff
checkout and invokes Pulse 44 exactly once. Only Pulse 44's complete retained
binary final root (`2/2`, independently verified) becomes Pulse 45's
platform-specific Pulse 43 `gate-complete/passed`. A platform failure is a
terminal failure. Pulse 44's successful clean Windows qualification, including
the exact Pulse 33 binary
`sha256:0736392a9fab3fa9404554f86e82e1dfcdf9e68e44aa3df8a4cddd1d58a34fd8`,
is fixed public infrastructure evidence, not an execution result.

Only after both platform events pass may the inherited exact adapter
preflight, Pulse 31 `39/39` public-input validation, and Pulse 35/Pulse 37
normalized-release proof proceed. Then, and only then, fresh private
materialization uses one exact 32-byte seed and one bounded two-platform
search: 70 cases and 70 processes per platform, 140 processes total, one
launch, zero retries, zero fallbacks, and a first-target-mismatch stop.
Pulse 25/27, Pulse 31, Pulse 35, Pulse 37, and sanitized-reproducer controls
are inherited unchanged.

## Publication integrity and privacy

At the terminal disposition, Pulse 43's exact publisher is invoked exactly
once with a fresh absent absolute public-result final root. No public terminal
summary may be emitted before it returns `published`, with final files `2/2`,
recomputed raw and payload hashes, one rename, and zero retries/fallbacks.

If publication is `absent`, `rolled-back`, or `indeterminate`, the authority
result is `invalid-publication/null`; an external summary may state only that
publication posture, never private-gate detail. Public result privacy fields
are all false. Category, product, and fix conclusions remain null unless the
complete bounded search gate validly finishes.

## Declaration controls

The canonical declaration identity is
`sha256:92847e645338fd142710c1afcff5d6ad5540c35e6322ccf59b574f2fd3d61534`.
It has 9,208 comprehensive mutation controls; the declared registry total is
38,819. Every execution-state field is currently zero, false, or null.

- [Exact declaration](fixtures/process-exit-diagnostic-pulse-46-authority.json)
- [Closed schema](schemas/ferris.process-exit-diagnostic-pulse-46-authority.v1.schema.json)
- [Mutation controls](fixtures/process-exit-diagnostic-pulse-46-authority-mutations.json)
- [Nine-role review](../../plans/reviews/PULSE-46-PUBLICATION-ORDER-DIAGNOSTIC-AUTHORITY-ROLE-REVIEW.md)
- [Rust validator](../../../crates/ferris-cli/tests/process_exit_diagnostic_pulse_46_authority.rs)
