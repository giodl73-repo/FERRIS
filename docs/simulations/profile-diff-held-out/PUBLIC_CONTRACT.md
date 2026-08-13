# Profile Diff Held-Out Public Contract

Status: Frozen public design
Contract revision: 1
Executable fixture: Unbound
Oracle: Withheld under `CUSTODY_AND_PREFLIGHT.md`

## Evaluation question

Does an immutable Ferris build compare two explicit
`ferris.profile-evidence/v0` inputs deterministically, privately, and within
the Pulse 14 authority and bounds on both Windows and Unix?

The score evaluates command conformance only. It does not evaluate whether a
profile is true, compatible, supported, fresh, secure, approved, complete, or
fit for adoption.

## Frozen command surface

Every scored process MUST use one of:

```console
ferris profile-diff --before <BEFORE_JSON> --after <AFTER_JSON> --format json
ferris profile-diff --before <BEFORE_JSON> --after <AFTER_JSON> --format human
```

The validation owner MUST invoke the immutable release binary directly. The
harness MUST NOT call library APIs, modify inputs between launch and read,
wrap output in another schema, retry failed cases, or select favorable
variants after observing results.

Cases 1 through 53 and case 55 MUST use JSON mode. Cases 54 and 56 MUST use
human mode. The format allocation is frozen before hidden fixture
construction and may not be changed after observing output.

## Platform and process cardinality

The sealed package MUST contain exactly 56 case definitions. Every case MUST
run once on:

- Windows x86-64; and
- Unix x86-64.

The scored collection therefore expects exactly 112 Ferris process records.
Missing, duplicate, retried, or extra processes invalidate the fixture before
oracle release.

The validation owner MUST record OS version, CPU architecture, filesystem,
locale, shell, current directory, environment allowlist, executable digest,
command digest, start and completion times, stdout and stderr byte counts,
and actual process exit code for every run.

## Frozen case matrix

### A. Result and section semantics: 16 cases

1. byte-distinct but canonically identical evidence;
2. identical evidence relocated to different request paths;
3. revision-only difference;
4. one identity-section difference;
5. one closure-section difference;
6. one features-section difference;
7. one toolchain-section difference;
8. one targets-section difference;
9. one providers-section difference;
10. one native-section difference;
11. one stages-section difference;
12. one assurance-section difference;
13. one stewardship-section difference;
14. one support-section difference;
15. one lifecycle-section difference; and
16. one multi-section difference containing added, removed, and changed
    values.

### B. Structural and pointer behavior: 8 cases

17. added scalar;
18. removed scalar;
19. changed scalar;
20. positional array difference;
21. array insertion with shifted indexes;
22. added empty object or array;
23. removed empty object or array; and
24. object keys requiring both RFC 6901 `~0` and `~1` escaping.

### C. Strict validation and classification: 12 cases

25. duplicate top-level member;
26. duplicate nested member;
27. unknown top-level field;
28. unknown section field;
29. invalid profile identifier;
30. invalid revision;
31. invalid consumer;
32. invalid output-visible object key;
33. mismatched profile identifiers;
34. mismatched consumers;
35. unsupported schema; and
36. malformed JSON.

### D. Explicit-input and resource bounds: 8 cases

37. missing first input;
38. missing second input;
39. non-file first input;
40. non-file second input;
41. oversized first input;
42. oversized second input;
43. exactly 10,000 emitted changes; and
44. more than 10,000 changes.

### E. Identity, privacy, and process boundary: 12 cases

45. first member of an object-key-reordering identity-equivalence pair;
46. second member of that identity-equivalence pair;
47. first member of a path-relocation identity-equivalence pair;
48. second member of that identity-equivalence pair;
49. first pre-read missing-path request in a non-collision pair;
50. second pre-read missing-path request in that non-collision pair;
51. first second-input failure with valid first content;
52. second second-input failure with equivalent relocated first content;
53. sealed section-value privacy canaries in JSON mode;
54. sealed section-value privacy canaries in human mode;
55. bounded output-visible metadata in JSON mode; and
56. the same bounded output-visible metadata in human mode.

The case number and class are public. Exact inputs, keys, values, canaries,
paths, ordering, package layout, expected digests, and oracle predicates are
sealed.

## Required result classes and exits

The sealed package MUST exercise and score:

| Result class | Exit |
|---|---:|
| `success` | 0 |
| `difference` | 1 |
| `invalid` | 2 |
| `unsupported` | 4 |
| `incomplete` | 5 |
| `blocked` | 7 |

Any catchable panic, serialization failure, or success-output failure observed
by the harness MUST retain the Pulse 13 typed `internal`/11 boundary and MUST
NOT be converted into another expected case.

## Machine and human output

JSON-mode output MUST be exactly one complete UTF-8
`ferris.command-result/v2` envelope followed by one newline. Success and
difference use stdout with stderr empty. Non-success uses stderr with stdout
empty. The actual exit code MUST equal `process_exit_code` for every JSON
process and the corresponding typed result exit for every human process.

Human-mode scoring MUST verify that every field represented by the typed
profile-diff record remains available without emitting raw section values.
Human output is not permitted to add an interpretation, support claim,
compatibility claim, approval, or success-shaped fallback.

## Identity and determinism

The oracle MUST independently compute:

- canonical input content digests;
- selection, invocation, diff, and result identities;
- normalized pre-read request material;
- sorted changed and unchanged sections;
- sorted JSON Pointer changes;
- added, removed, and changed classification; and
- before and after value digests.

Successful identities MUST be independent of request paths and object-key
order. Failure identities MUST preserve the documented pre-read and
second-input distinctions. Repeated execution is not allowed after first
score, so determinism is established through equivalent sealed cases rather
than rerunning one held-out case.

## Privacy and prohibition predicates

The sealed inputs MUST contain unique canaries only in raw section values.
The scorer MUST inspect complete stdout and stderr bytes and fail if any
canary appears in either stream.

The scorer MUST also reject:

- input paths in successful records;
- control characters in output-visible metadata;
- raw profile section fragments;
- credentials or reusable secrets in the fixture package;
- owner, compatibility, support, certification, freshness, or approval
  conclusions; and
- partial successful output after a bound is exceeded.

Profile identifiers, revisions, consumers, and object keys remain documented
output-visible metadata and MUST be scored as present where the contract
requires them.

## Scoring disposition

The first score is one of:

- `pass`: every mandatory process and predicate passes;
- `fail`: a conforming fixture exposes an implementation deviation;
- `invalid`: fixture, harness, collection, custody, or scorer qualification
  failed before a valid implementation score;
- `unsupported`: the frozen environment cannot execute the bound binary; or
- `blocked`: an external prerequisite failed before Ferris execution.

There is no aggregate percentage threshold and no partial pass. Every
mandatory predicate is release-blocking for the held-out claim.

## Claim boundary

A pass establishes only conformance to this experimental local diff contract
at one immutable cutoff on the recorded environments. It does not advance
PLATFORM-001, authorize profile generation, prove real profile evidence,
establish support or compatibility, or authorize deployment.
