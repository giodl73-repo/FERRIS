# Independent Process-Exit Diagnostic Public-Bundle Contract

Status: Authorized; unexecuted
Program: `FERRIS-P26-INDEPENDENT-PROCESS-EXIT-DIAGNOSTIC-PUBLIC-BUNDLE`
Schema: `ferris.process-exit-diagnostic-public-bundle/v1`
Disclosure tier: `sanitized-reproducer` (precommitted before generation)

## Purpose

This contract authorizes one new independent diagnostic program for the
released public category `process-exit-agreement`, using the exact public
Pulse 25 collector source bundle.

Pulse 26 is not a retry, resume, reseed, rescore, reuse, or continuation of
Pulse 22 or Pulse 24. It does not reopen either package. It is diagnostic
only, not certification, a held-out score, a Pulse 17 retry, product-fix
authority, or PLATFORM-001 advancement.

This governance change creates no custody workspace, seed, commitment, case,
input, executable selection, preflight observation, process record, pair
seal, result, or reproducer. It MUST NOT execute a preflight or candidate.

## Permanently closed predecessors

Pulse 22 remains permanently `invalid`, permanently non-retryable, and unable
to produce a category conclusion. Its incomplete Windows-only process cannot
become a Pulse 26 preflight, candidate, corpus member, comparison point, or
source of inference.

Pulse 24 remains permanently `invalid-before-candidates`, permanently
non-retryable, and unable to produce a category conclusion. Its report
verification and custody records cannot become Pulse 26 verification,
preflight, generation, candidate, or result evidence.

The exact public predecessor bindings are:

| Artifact | Required digest |
|---|---|
| Pulse 22 public contract | `sha256:6f4447444c49814b61a3c6234af6f6690e40d618aa3f6acfa78e00fd8c0a2ec8` |
| Pulse 22 public result | `sha256:3dcd2def79cced56fb266e16e5d6c4bc12e9f7db688bf6f34cb0eed47743d2e7` |
| Pulse 24 public contract | `sha256:1fc3904bd55587e36d22ac1d7532b365e87793dbe493f0073bd392fb546e02e8` |
| Pulse 24 public result | `sha256:b845858d0ca8c7011443140d8cfdbebbbe925f1d17e26c414cad09e48df19db4` |

Pulse 26 MUST NOT access, modify, retry, resume, reseed, rescore, reuse,
reconstruct, correlate, or infer:

- Pulse 17 private material, fixture inputs, candidates, streams, seed,
  manifest, expected records, oracle, or case identities;
- Pulse 22 private material, including its seed, commitment preimage,
  classifier, generator, corpus, case or coverage manifest, inputs, outputs,
  paths, streams, or retained process record;
- Pulse 24 custody material, including its identity, workspace, verification
  records, executable, paths, or any unpublished custody artifact; or
- Pulse 19 case bytes.

## Authority

Before custody, this repository may contain only:

- this normative public contract;
- one public machine-readable `authorized-unexecuted` declaration;
- its closed Draft 2020-12 schema and negative-control mutations;
- one nine-role pre-execution review; and
- test-only validation of those public artifacts and the released bundle.

After handoff, a new independent custodian MAY create a new isolated
workspace, copy the exact public bundle, independently verify it, run the
fixed preflight, freeze wholly new generation material, execute the bounded
search once, minimize a reproduced case in a separate phase, and publish the
required public result.

Pulse 26 grants no authority for Ferris production code, CLI, API,
dependency, output, stream, result-class, exit-map, or product-behavior
changes. It grants no score, certification, support, owner-system mutation,
network use, credential use, mutable external-system action, or
PLATFORM-001 status change.

## Immutable Ferris cutoff

Any later execution MUST use Ferris commit
`e01130a5c1fc5b8e58e13bbde03dfc39b8f1bf60`.

Before bundle verification or preflight, custody MUST independently verify
the exact commit and freeze the directly launched executable digest. A
different commit, indirect launch, or unverified executable invalidates the
package before candidates.

## Exact Pulse 25 public collector bundle

The only permitted collector source directory is:

`docs/simulations/profile-diff-held-out/pulse-25-collector-source-release/bundle`

The exact release bindings are:

| Binding | Required digest |
|---|---|
| Public manifest | `sha256:771f8521acbdada3388cfd15d61b565a590ff4f74c65bd768f7e114682b30c75` |
| Source aggregate | `sha256:5c54e47be59ffd2ad1ce7b83fdfa1302d05bd6539197f19f28e2e92d480cc558` |
| Test aggregate | `sha256:7aa04613f6578e0ef29edf04d781332542833b20a22bab053b1ac769f8d48f62` |
| Complete bundle aggregate | `sha256:8311268bc5835a6d835da52531d02e450a14992ffb03cafbed7a019003f21bbc` |
| Release receipt | `sha256:ebd68c4f86f2602ed3af0f84154cec290e878d9b00109a2eb9615e9c69ed3780` |
| Release seal | `sha256:fa7cd42ff0766f126b7400959429f2c65a32dbeac8c4caeecfca3e5a445979c0` |

The public qualification report remains bound as
`sha256:1365dfd33f834b24f9e20f73afce313978b26d27e3c076ae90a8671eb841e723`.

The custodian MUST copy only the nine files under the public `bundle`
directory into a new isolated custody workspace. The copied workspace MUST
contain no manifest, receipt, seal, private file, prior custody file,
diagnostic material, or substituted collector file.

Before preflight, the custodian MUST independently:

1. verify the public manifest, release receipt, and release seal digests;
2. verify that the public source directory contains all nine declared files;
3. copy exactly those nine files and no others into the isolated workspace;
4. recompute all nine per-file SHA-256 digests from the copied bytes;
5. recompute the source aggregate;
6. recompute the test aggregate;
7. recompute the complete bundle aggregate using
   `sha256-length-path-filedigest-v1`; and
8. compare every recomputed value with the pinned public binding.

Any missing file, extra copied file, path drift, byte drift, digest mismatch,
aggregate mismatch, receipt mismatch, seal mismatch, or attempted repair or
substitution invalidates the package before candidates. Collector repair or
requalification requires a separately authorized later pulse.

## Mandatory synthetic preflight

After all bundle and executable verification and before generation or any
candidate launch, custody MUST run exactly two harmless synthetic atomic
Windows/Ubuntu pairs from the copied public bundle.

Each pair MUST:

- use only fixed harmless public synthetic commands and payloads;
- retain one Windows x86-64 record and one Ubuntu 24.04 WSL2 x86-64 record;
- durably synchronize each complete process record;
- seal the pair only after both records exist;
- durably synchronize the pair seal;
- verify the sealed pair through a fresh read-only process; and
- leave zero interrupted-write residue.

Preflight retries are zero. A preflight pair, member, or command MUST NOT be
repeated. Preflight rows are infrastructure controls, not candidates; they
do not count toward candidate or search-process bounds and cannot create a
`process-exit-agreement` conclusion.

Any launch, stream, bound, durability, reload, residue, seal, digest,
cardinality, or exact-pair-count failure makes the package
`invalid-before-candidates` and permanently prohibits candidate launch.
Repair or another preflight attempt requires a separately authorized later
pulse.

## New custody and fresh generation

Pulse 26 requires:

1. a new custody identity;
2. a new isolated custody workspace;
3. a new private deterministic seed and public commitment;
4. a new independently implemented and frozen classifier;
5. a new independently implemented and frozen deterministic generator;
6. new independently frozen case and coverage manifests; and
7. a new fresh corpus.

None of those identities, bytes, digests, sources, descriptions, choices, or
ordering may be inherited from Pulse 17, Pulse 22, Pulse 24, or Pulse 19.
Implementation authors MUST NOT construct, select, generate, inspect, or
execute Pulse 26 candidates.

Before generation, custody MUST append-only commit the exact private seed
bytes, creation time, program ID, new custody identity and workspace,
classifier digest, generator digest, and generator version. Only the
`sha256:` seed commitment is public.

Generation MUST be deterministic using:

`SHA256(seed || NUL || domain || NUL || counter)`

Generation enumerates all mandatory obligations in public document order,
constructs only public-rule-valid descriptions, greedily selects the case
covering the most remaining obligations, breaks ties by description digest,
and stops only when all obligations close or 512 unique fresh cases exist.
Failure to close coverage within 512 cases invalidates before candidate
execution and cannot produce `no-reproduction`.

## Incorporated public generation and oracle contract

Pulse 26 incorporates without weakening the complete public generation,
coverage, oracle, minimization, and publication domains frozen by Pulse 22
and preserved exactly by Pulse 24:

- every result class, mapped exit, JSON route, record-nullability state, and
  diagnostic cardinality;
- both required human-format parity partners;
- every metadata site, UTF-8 byte boundary, character kind, JSON value kind,
  numeric lexical family, pointer-key kind, duplicate depth, member ordering,
  input role ordering, failure position, path state, path form, lexical path
  normalization behavior, input byte boundary, and change-count boundary;
- all eight mandatory interactions;
- every independent-oracle field; and
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

The target category is reproduced only if at least one of these exact
predicates is false:

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

## Exact search bounds

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

## Transactional durable collection

Every reached candidate is one transaction:

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
not a completed pair and MUST NOT be classified.

After the first candidate launch, any launch, timeout, stream-read,
stream-bound, durability, residue, pair-seal, fresh-process reload,
cardinality, manifest, classifier, or custody failure invalidates the entire
package and permanently prohibits further launches. It cannot be retried or
converted into `no-reproduction`.

## First reproduction and separate minimization

After the first completed cross-platform pair reproduces the target category,
custody MUST preserve the first reproducer immutably and stop search
launches.

Minimization is a separately identified phase bounded to:

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

No disposition changes Pulse 17, repairs Pulse 22 or Pulse 24, establishes a
score or certification, advances PLATFORM-001, or authorizes a product fix.

## Custody handoff

The public declaration remains `authorized-unexecuted`. It contains no
custodian selection, workspace, copied bundle, recomputed custody digest,
seed, commitment value, candidate, generated input, executable digest,
preflight observation, process record, pair seal, result, or reproducer.

The new independent custodian must freeze, in order:

1. new custody identity and isolated workspace;
2. immutable Ferris cutoff and executable digest;
3. public manifest, receipt, seal, and source-directory verification;
4. the copied nine-file-only workspace;
5. all nine per-file and source/test/bundle recomputations;
6. exactly two zero-retry harmless atomic Windows/Ubuntu preflight pairs;
7. new classifier source and digest;
8. new generator source and digest;
9. new private seed commitment;
10. new case and coverage manifests; and
11. the one-execution candidate launch authorization.

This repository change performs none of those custody or execution actions.
