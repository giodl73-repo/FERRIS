# Pulse 84 witnessed capability/materialization diagnostic authority

Status: `authorized-unexecuted`

Immutable cutoff:
`f874ebfe29e58460fc0a553418d11d6785e84df9`

## Frame

Exact Pulse 82 is the working owner system. It owns execution ordering,
cleanup, publication, terminal failure, and the complete Pulse 35/Pulse 78
predecessor stack. The missing capability is one self-excluding governance
record that permits independent custody to attempt that exact callable once.

Pulse 84 contributes only the authority boundary around Pulse 82. Git retains
artifact identity, Windows and WSL retain platform semantics, Cargo and Python
retain owner behavior, and Pulse 82 retains all runtime and publication
semantics.

The deletion target is the Pulse 68 pattern of restating thousands of sealed
predecessor fields inside each authority. Pulse 84 instead binds the complete
exact Pulse 82 manifest and seal, then defines only authority-owned custody,
inputs, stop conditions, consumption, transfer destinations, and conclusions.

This thesis is disproved if the cutoff includes Pulse 84, omits Pulse 83, does
not contain the exact Pulse 82 release, or if any execution field is nonzero
before independent custody acts.

## Audit

- Pulse 68 is permanently withdrawn and cannot be retried or amended.
- Pulses 69-82 close its public predecessor blockers and terminate in one
  injection-free six-input callable.
- Pulse 83 statically proves that repair chain at merged cutoff `dfc889b`.
- Cutoff `f874ebf` contains Pulse 83 and exact Pulse 82 while excluding every
  Pulse 84 artifact.
- Pulse 82 derives terminal publication as a fresh sibling of
  `private_runtime_root`, runs terminal publication only after exact Pulse 81
  completes, and preserves null conclusions for non-success postures.

## Compare

| Analogue | Classification | Use |
| --- | --- | --- |
| Pulse 32 self-excluding authority | reuse | retain one-shot consumption, immutable cutoff, zero initial activity, null conclusions, and stop-before-widening |
| Pulse 68 exact root and transfer contracts | adapt | retain explicit custody roots and path-free public transfer, but replace withdrawn Pulse 59 and dynamic route claims |
| Pulse 82 sealed manifest and release seal | reuse | bind one complete owner artifact instead of restating transitive implementation detail |
| direct predecessor callable binding | avoid | all direct Pulse 43/47/51/52/56/78/81 calls remain prohibited |

SLSA provenance describes verifiable information about where, when, and how an
artifact was produced. Pulse 84 uses that model for cutoff and release
identity, while keeping execution permission in this separate authority.

Primary source: <https://slsa.dev/spec/v1.2/provenance>

## Evaluate

The nine-role review accepts a single unexecuted authority with these retained
risks and gates:

- Windows mutex and Linux abstract-socket behavior remains fake-qualified;
- the authority is consumed on the sole callable attempt even when native
  platform behavior fails;
- all supplied roots must pass exact freshness, separation, path-length,
  ownership, and reversible-creatability checks before invocation; and
- public records must remain path-free and contain no seed or private custody
  material.

## Slice

The bounded slice contains one declaration, one recursively closed schema, one
comprehensive mutation set, one role review, and one static Rust validator.
The validator reads immutable Git blobs and governance artifacts only. It
does not import or invoke Python.

## Authorized execution contract

Independent custody may perform public cutoff/release validation and harmless
reversible environment checks, then attempt exactly one Pulse 82 production
callable invocation with its six concrete inputs. Failed pre-call validation
creates no authority consumption. The sole invocation attempt consumes the
authority permanently.

The callable may produce only Pulse 82's terminal classes:
`published-result`, `published-failure-witness`,
`invalid-witness-publication`, or a pre-publication not-attempted failure.
No outcome grants a product, score, certification, support, fix, or
PLATFORM-001 conclusion.

## Decision

Pulse 84 is authorized and unexecuted. This pulse performs no custody action
and grants no retry, resume, alternate cutoff, alternate callable, direct
predecessor invocation, hidden-material access, or inference.
