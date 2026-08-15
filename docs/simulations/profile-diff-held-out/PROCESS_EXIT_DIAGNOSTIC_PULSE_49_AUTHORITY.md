# Pulse 49 public-catalog successor authority

Status: `authorized-unexecuted` only. This is one fresh, independent
authority at immutable cutoff
`96b2bda8bea455a2e5a4610c4ab1722e2e68fcb3`.

## Independence and scope

Pulse 49 is not a retry, resume, reconstruction, reseed, reuse, correlation,
or inference of Pulse 48. Pulse 48 remains permanently
`invalid-publication-integrity`, non-retryable, and null-conclusion. This
successor corrects only Pulse 48's public catalog incompatibility: ordered
gate seven is `bounded-materialization`.

The authority reuses the exact current-cutoff public custody for Pulse 41,
Pulse 39, Pulse 43, Pulse 44, Pulse 45, and Pulse 47, and retains Pulse 48's
bounded ordered execution contract: one launch; one Pulse 41/Pulse 39 custody
operation; one Pulse 45 and one nested Pulse 44 operation for each platform;
70 cases/processes per platform; 140 total processes; zero retries and
fallbacks; and a null conclusion unless the complete valid search result is
available.

## Nonadvancing public catalog preauthorization

Before any ordered execution, the declaration binds a successful,
deterministic public catalog preauthorization proof:

- all eight ordered gate IDs, in order;
- all externally releasable Pulse 43 validation and ordered-event identifiers;
- Pulse 43's 1--24 catalog cardinality, uniqueness, 48-character lowercase
  identifier rule, and exact forbidden standalone-part set; and
- a domain-separated evidence identity
  `sha256:0dc56e97c455bdc139b150c69236d386b97bace9e509467996696718fce5fd90`.

The exact identifiers are limited to the closed gate catalog plus
`public-catalog-prevalidation` and `public-input-contract` validation IDs.
There are no caller-supplied failure identifiers. The proof is nonadvancing,
does not change execution state, invokes neither Pulse 43 nor any private
operation, and is bound by the declaration identity.

## Ordered catalog and terminal publication

The closed ordered catalog is:

1. `pulse-41-pulse-39-public-custody`
2. `windows-retained-binary-custody`
3. `ubuntu-retained-binary-custody`
4. `exact-adapter-preflight`
5. `pulse-31-public-input`
6. `pulse-35-pulse-37-normalization`
7. `bounded-materialization`
8. `bounded-process-exit-search`

Public self-validation is explicitly nonadvancing. A failed ordered gate stops
later ordered gates, whose counts remain indeterminate rather than execution.

At terminal disposition, exact Pulse 47 is invoked once and invokes exact
Pulse 43 once. There is no direct Pulse 43 terminal call, retry, or fallback.
The Pulse 43 result root and Pulse 47 witness root must be fresh, absent,
absolute, distinct, and non-overlapping. Only a complete valid result may
produce a non-null conclusion; public disclosure remains bounded and contains
no private ordered-gate detail.

## Bound artifacts

- [Canonical declaration](fixtures/process-exit-diagnostic-pulse-49-authority.json)
- [Closed schema](schemas/ferris.process-exit-diagnostic-pulse-49-authority.v1.schema.json)
- [Exhaustive mutations](fixtures/process-exit-diagnostic-pulse-49-authority-mutations.json)
- [Nine-role review](../../plans/reviews/PULSE-49-PUBLIC-CATALOG-SUCCESSOR-AUTHORITY-ROLE-REVIEW.md)
- [Rust authority validator](../../../crates/ferris-cli/tests/process_exit_diagnostic_pulse_49_authority.rs)

Declaration identity:
`sha256:01101bb7d2a63b657940f82f80eb3edcd3ab7bba05cb8cd54e4dd0c87ce8a3ee`.
The declaration has 9,657 exhaustive deterministic mutation controls; the
declared repository registry total is 57,974. No execution or result artifact
is created by this authority.
