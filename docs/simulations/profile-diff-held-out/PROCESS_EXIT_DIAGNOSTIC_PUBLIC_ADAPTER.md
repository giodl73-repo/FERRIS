# Independent Process-Exit Diagnostic Public-Adapter Contract

Status: Authorized; unexecuted
Program: `FERRIS-P28-INDEPENDENT-PROCESS-EXIT-DIAGNOSTIC-PUBLIC-ADAPTER`
Schema: `ferris.process-exit-diagnostic-public-adapter/v1`
Disclosure tier: `sanitized-reproducer` (precommitted before generation)

## Purpose

This contract authorizes one new independent diagnostic program for the
released public category `process-exit-agreement`. It uses the exact public
Pulse 25 collector together with the exact public Pulse 27 two-pair adapter.

Pulse 28 is not a retry, resume, reseed, rescore, reuse, continuation,
correlation, or inference from Pulse 22, Pulse 24, or Pulse 26. It does not
reopen any predecessor package. It is diagnostic only, not certification, a
held-out score, a Pulse 17 retry, product-fix authority, or PLATFORM-001
advancement.

This governance change creates no custody workspace, copy, seed, commitment,
classifier, generator, manifest, case, corpus, executable selection,
preflight observation, process record, pair seal, result, or reproducer. It
MUST NOT invoke the adapter, a verifier, or a Ferris candidate.

## Permanently closed predecessors

Pulses 22, 24, and 26 remain permanently `invalid`, permanently
non-retryable, and unable to produce category conclusions:

| Pulse | Public label | Completed diagnostic pairs | Retries | Category conclusion |
|---:|---|---:|---:|---|
| 22 | `invalid` | 0 | 0 | null |
| 24 | `invalid-before-candidates` | 0 | 0 | null |
| 26 | `invalid-before-candidates` | 0 | 0 | null |

The exact predecessor bindings are:

| Artifact | Required digest |
|---|---|
| Pulse 22 public contract | `sha256:6f4447444c49814b61a3c6234af6f6690e40d618aa3f6acfa78e00fd8c0a2ec8` |
| Pulse 22 public result | `sha256:3dcd2def79cced56fb266e16e5d6c4bc12e9f7db688bf6f34cb0eed47743d2e7` |
| Pulse 24 public contract | `sha256:1fc3904bd55587e36d22ac1d7532b365e87793dbe493f0073bd392fb546e02e8` |
| Pulse 24 public result | `sha256:b845858d0ca8c7011443140d8cfdbebbbe925f1d17e26c414cad09e48df19db4` |
| Pulse 26 public contract | `sha256:e3546b7258706731141acd436ed83bb4fc05937d4af97ac87559b725e3daee86` |
| Pulse 26 public result | `sha256:00f19dda516fe4ec354b1b41ca0b9b78c32aba41a667ad077c505d60458d3842` |

Pulse 28 MUST NOT access, modify, retry, resume, reseed, rescore, reuse,
reconstruct, continue, correlate, or infer:

- Pulse 17 private material, expected records, oracle, seed, streams, or case
  identities;
- Pulse 22 private custody, seed, generator, classifier, corpus, process, or
  stream material;
- Pulse 24 private custody, workspace, executable, path, or verification
  material;
- Pulse 26 private custody, workspace, copied package, executable, preflight,
  process, or seal material; or
- Pulse 19 case bytes.

## Authority

Before custody, this repository may contain only:

- this normative public contract;
- one public machine-readable `authorized-unexecuted` declaration;
- its closed Draft 2020-12 schema and negative-control mutations;
- one nine-role pre-execution review; and
- test-only validation of those public artifacts and the Pulse 25 and Pulse
  27 releases.

After handoff, a new independent custodian MAY establish a new isolated
workspace, copy only the exact manifest-listed public package, independently
verify it, run the fixed preflight once, freeze wholly new generation
material, execute the bounded search once, minimize a reproduced case in a
separate phase, and publish the required public result.

Pulse 28 grants no authority for Ferris production code, CLI, API,
dependency, output, stream, result class, exit map, or product-behavior
changes. It grants no score, certification, support, owner-system mutation,
network use, credential use, mutable external-system action, or PLATFORM-001
status change.

## Immutable Ferris cutoff

Any later execution MUST use Ferris commit
`2935f44475b811e619f2ef62e0d408f39c7e8149`.

That commit contains the complete Pulse 27 public release and predates this
Pulse 28 authority. Before any release copy or preflight, custody MUST
independently verify the exact commit and freeze the directly launched
executable digest. A different commit, indirect launch, unverified
executable, or cutoff containing the Pulse 28 authority invalidates the
package before candidates.

## Exact Pulse 25 public collector bindings

Pulse 28 inherits every Pulse 25 public collector binding pinned by Pulse 26:

| Binding | Required digest |
|---|---|
| Public manifest | `sha256:771f8521acbdada3388cfd15d61b565a590ff4f74c65bd768f7e114682b30c75` |
| Qualification report | `sha256:1365dfd33f834b24f9e20f73afce313978b26d27e3c076ae90a8671eb841e723` |
| Source aggregate | `sha256:5c54e47be59ffd2ad1ce7b83fdfa1302d05bd6539197f19f28e2e92d480cc558` |
| Test aggregate | `sha256:7aa04613f6578e0ef29edf04d781332542833b20a22bab053b1ac769f8d48f62` |
| Complete bundle aggregate | `sha256:8311268bc5835a6d835da52531d02e450a14992ffb03cafbed7a019003f21bbc` |
| Release receipt | `sha256:ebd68c4f86f2602ed3af0f84154cec290e878d9b00109a2eb9615e9c69ed3780` |
| Release seal | `sha256:fa7cd42ff0766f126b7400959429f2c65a32dbeac8c4caeecfca3e5a445979c0` |

The declaration also pins all nine Pulse 25 manifest paths, kinds, byte
lengths, and per-file SHA-256 digests. Custody MUST independently verify the
public Pulse 25 manifest and files and recompute the source, test, and complete
bundle aggregates before accepting the Pulse 27 collector subset.

## Exact Pulse 27 public adapter release

The only permitted adapter release directory is:

`docs/simulations/profile-diff-held-out/pulse-27-preflight-adapter-release`

The exact release bindings are:

| Binding | Required digest |
|---|---|
| Public manifest | `sha256:449851e7b917f474fb1829b2d9f89a3f08a886733c476889dfad1ae27d097154` |
| Complete 20-file release aggregate | `sha256:31f38a79629d6b5da1fab9cb335450a95a1763f1ac80b1d8d851b103a318e540` |
| Adapter source aggregate | `sha256:33106fb3ffc6c71148f954870dc5ae00ec607fc6c3b86412a81258c1fe7cfa63` |
| Adapter test aggregate | `sha256:59b285cbc2eb6a285a88503c75c9c6b2d89f219851e7783c074b0a1f7a9a10ff` |
| Collector-copy aggregate | `sha256:c0421b4d44fecf132ea31939a044d7b8e1545dd2472da13ce5f0702defd85c0c` |
| Root-cause report | `sha256:9bd5ac7aa29b1f621df09eefc2ff33369c5ba5810e4e5f76f4e7e15aab57f478` |
| Qualification receipt | `sha256:94ec4237fd046281b9971e7eea67dc1ae7208996ed69b50ce8430a78cd0b6886` |
| Release seal | `sha256:6bff93434170ee79a4b5210ee26b647ab6dba351dccc4ccf3a5e224de56ced38` |

The public manifest's complete 20-entry `files` array is normative. Every
path, kind, byte length, and SHA-256 digest MUST match exactly. The manifest's
aggregate algorithm, all aggregate and artifact digests, counts, and
`collector_modified:false` binding are also normative.

The nine `collector/` entries MUST be byte-for-byte identical to the
corresponding Pulse 25 files. No collector modification, repair, compatible
substitution, or reconstructed equivalent is authorized.

## New isolated custody copy

The custodian MUST create a new isolated workspace and MUST NOT read from or
copy any private or prior custody workspace.

The custodian MUST:

1. verify the Pulse 25 public manifest, files, receipt, seal, and aggregates;
2. verify the Pulse 27 public manifest, root-cause report, qualification
   receipt, and release seal digests;
3. verify every Pulse 27 manifest entry at the public release directory;
4. compare all nine Pulse 27 `collector/` files byte-for-byte with Pulse 25;
5. copy exactly the 20 files listed by the Pulse 27 manifest into the new
   isolated workspace, preserving manifest-relative paths;
6. copy no manifest, release seal, private file, prior-custody file,
   diagnostic material, or unlisted file into that workspace;
7. independently recompute all 20 per-file SHA-256 digests;
8. independently recompute the adapter-source, adapter-test,
   collector-copy, and complete-release aggregates using
   `sha256-length-path-filedigest-v1`; and
9. compare every recomputed value with the pinned public binding.

The public manifest and release seal are verified at the public source but are
not among the 20 manifest-listed package files and therefore MUST NOT be
copied into custody.

Any missing file, extra copied file, path drift, byte drift, digest mismatch,
aggregate mismatch, collector mismatch, receipt mismatch, seal mismatch,
private-workspace access, attempted repair, or substitution makes the package
`invalid-before-candidates`.

## Mandatory exact-two-pair preflight

After all public-package and executable verification and before generation or
any candidate launch, custody MUST run exactly one adapter invocation from the
copied public package.

That single invocation MUST deterministically create exactly:

- two Windows x86-64 process rows;
- two Ubuntu 24.04 WSL2 x86-64 process rows;
- four total process rows;
- two atomic Windows/Ubuntu pairs; and
- two joined pair seals.

Only after the complete store exists, custody MUST launch exactly two fresh,
read-only verifier processes: one Windows verifier and one Ubuntu verifier.
Each verifier MUST require exact whole-store cardinality `2/2/2`: two Windows
rows, two Ubuntu rows, and two pair seals.

Adapter, pair, and verifier retries are all zero. No adapter invocation,
member command, or verifier may be repeated. The complete six-file store MUST
have zero interrupted-write residue. Preflight rows are infrastructure
controls, not candidates; they do not count toward candidate or search-process
bounds and cannot create a `process-exit-agreement` conclusion.

Any launch, stream, bound, durability, seal, join, exact-cardinality,
fresh-process, residue, or exact-count failure makes the package
`invalid-before-candidates` and permanently prohibits candidate launch.
Repair or another preflight attempt requires a separately authorized later
pulse.

## New custody and fresh generation

Pulse 28 requires:

1. a new custody identity;
2. a new isolated custody workspace;
3. a new private deterministic seed and public commitment;
4. a new independently implemented and frozen classifier;
5. a new independently implemented and frozen deterministic generator;
6. new independently frozen case and coverage manifests; and
7. a new fresh corpus.

None of those identities, bytes, digests, sources, descriptions, choices, or
ordering may be inherited from Pulse 17, Pulse 22, Pulse 24, Pulse 26, or
Pulse 19. Implementation authors MUST NOT construct, select, generate,
inspect, or execute Pulse 28 candidates.

Before generation, custody MUST append-only commit the exact private seed
bytes, creation time, program ID, custody identity and workspace, classifier
digest, generator digest, and generator version. Only the `sha256:` seed
commitment is public.

Generation MUST be deterministic using:

`SHA256(seed || NUL || domain || NUL || counter)`

Generation enumerates all mandatory obligations in public document order,
constructs only public-rule-valid descriptions, greedily selects the case
covering the most remaining obligations, breaks ties by description digest,
and stops only when all obligations close or 512 unique fresh cases exist.
Failure to close coverage within 512 cases invalidates before candidate
execution and cannot produce `no-reproduction`.

## Complete inherited public generation and oracle

Pulse 28 inherits without change every Pulse 26 coverage and oracle field:

- every result class, mapped exit, JSON route, record-nullability state, and
  diagnostic cardinality;
- both required human-format parity partners;
- every metadata site, UTF-8 byte boundary, character kind, JSON value kind,
  numeric lexical family, pointer-key kind, duplicate depth, member ordering,
  input role ordering, failure position, path state, path form, lexical path
  normalization behavior, input byte boundary, and change-count boundary;
- all eight mandatory interactions;
- all eight independent-oracle compared fields; and
- all six target predicates.

The independent classifier MUST be frozen before generation, MUST use only
public rules, and MUST NOT call, link, import, copy, or derive expected values
from Ferris production code, Ferris test helpers, a prior diagnostic harness,
captured candidate output, or private material.

For every JSON row, custody independently records:

- expected result class and public mapped exit;
- emitted result class and `process_exit_code`;
- actual operating-system exit;
- emitted diagnostic class;
- record nullability;
- stdout/stderr route;
- complete retained stream digests; and
- framing and parse disposition.

The target category is reproduced only if at least one exact predicate is
false:

```text
expected_exit == public_map(expected_class)
emitted_class == expected_class
emitted_exit == public_map(emitted_class)
emitted_exit == expected_exit
actual_os_exit == emitted_exit
actual_os_exit == expected_exit
```

Adjacent diagnostic, nullability, framing, privacy, or route failures MUST be
preserved and stop the search, but MUST NOT be relabeled as
`process-exit-agreement` unless one of the six predicates also fails.

## Exact search and collection bounds

Pulse 28 inherits the Pulse 26 search bounds exactly:

| Bound | Maximum |
|---|---:|
| Unique fresh logical cases | 512 |
| Cases per platform | 512 |
| Windows launches per reached case | 1 |
| Ubuntu launches per reached case | 1 |
| Total search process launches | 1,024 |
| Candidate retries | 0 |
| Search executions | 1 |

Each reached case is one atomic cross-platform pair. Search stops after the
first completed pair reproducing `process-exit-agreement`; no later candidate
may launch.

Every reached candidate remains one transactional durable pair:

1. directly launch the immutable Ferris binary once on the first platform;
2. durably synchronize that complete process record;
3. directly launch once on the partner platform;
4. durably synchronize that complete process record;
5. create and durably synchronize the pair seal only after both records
   exist;
6. start a fresh read-only process;
7. reload both records and the pair seal from durable storage;
8. verify identity, cardinality, stream digests, exits, and seal joins; and
9. only then classify the completed pair.

An in-memory row, unsynchronized row, partial pair, or same-process reread is
not a completed pair and MUST NOT be classified. After the first candidate
launch, any collection, manifest, classifier, or custody failure invalidates
the package and permanently prohibits further launches.

## First reproduction and separate minimization

After the first completed cross-platform pair reproduces the target category,
custody MUST preserve the first reproducer immutably and stop search launches.

Minimization inherits the Pulse 26 bounds exactly:

| Bound | Maximum |
|---|---:|
| Recorded transformations | 128 |
| Direct launches per transformation | 2 |
| Minimization process launches | 256 |
| Candidate retries | 0 |

Every lineage row records the transformation, parent and child identities,
input digests, both platform observations, predicate results, and
accept/reject disposition. A child replaces the current minimum only when it
reproduces on both platforms and is strictly smaller under the inherited
precommitted size tuple.

## Public result requirements

The permitted dispositions are:

- `reproduced`;
- `no-reproduction`;
- `incomplete-after-reproduction`; or
- `invalid`.

A completed `reproduced` disposition requires a fresh harmless sanitized
public directory, complete minimization lineage, direct Windows and Ubuntu
commands and observations, all required digests, zero overlap with prohibited
material, permanent retirement from certification, and a valid
`ferris.post-score-diagnostic-release/v1` receipt.

A completed `no-reproduction` disposition publishes only exact bounded
coverage, cardinality, platform, toolchain, command, timeout, stream, seed
commitment, generator, classifier, manifest, input, output, and process
aggregates, plus:

`bounded no-reproduction; no fix authority`

An `invalid` disposition publishes the exact public-safe blocker and stage,
preflight and candidate counts, completed-pair count, process count, retry
count, and further-launch prohibition. Its category conclusion MUST be null.

No disposition changes Pulse 17, repairs Pulse 22, Pulse 24, or Pulse 26,
establishes a score or certification, advances PLATFORM-001, or authorizes a
product fix.

## Custody handoff

The public declaration remains `authorized-unexecuted`. It contains no
custodian selection, workspace, copied package, recomputed custody digest,
seed, commitment value, candidate, generated input, executable digest,
preflight observation, process record, pair seal, result, or reproducer.

The new independent custodian must freeze, in order:

1. new custody identity and isolated workspace;
2. immutable Ferris cutoff and executable digest;
3. Pulse 25 public bindings and adapter collector byte identity;
4. Pulse 27 manifest, root-cause, qualification, seal, and directory
   verification;
5. the copied 20-file-only workspace;
6. all per-file and adapter/test/collector/release aggregate recomputations;
7. exactly one adapter invocation producing two pairs, four rows, and two
   seals;
8. exactly two fresh platform verifier processes enforcing cardinality
   `2/2/2`;
9. new classifier source and digest;
10. new generator source and digest;
11. new private seed commitment;
12. new case and coverage manifests; and
13. the one-execution candidate launch authorization.

This repository change performs none of those custody or execution actions.
