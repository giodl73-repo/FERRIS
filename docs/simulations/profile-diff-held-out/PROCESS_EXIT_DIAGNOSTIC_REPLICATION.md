# Process-Exit Diagnostic Replication Contract

Status: Authorized; unexecuted
Program: `FERRIS-P22-PROCESS-EXIT-DIAGNOSTIC-REPLICATION`
Schema: `ferris.process-exit-diagnostic-replication/v1`
Disclosure tier: `sanitized-reproducer` (precommitted before generation)

## Purpose

This contract authorizes one fresh independent diagnostic search for the
released public category `process-exit-agreement`.

It is not certification, a held-out score, a Pulse 17 retry, or a product-fix
program. The valid Pulse 17 fail and its public-safe result remain immutable
whether Pulse 22 reproduces the category, does not reproduce it, or cannot
complete.

Pulse 22 MUST NOT read, modify, retry, rescore, reuse, reconstruct, correlate,
or infer the Pulse 17 package, fixture, inputs, paths, canaries, digests,
expected records, or oracle. Implementation authors MUST NOT construct,
select, generate, inspect, or execute Pulse 22 candidates.

## Authority

Before custody, this repository may contain only:

- this public contract;
- the public machine-readable `authorized-unexecuted` declaration;
- its closed schema and negative-control mutations;
- the nine-role pre-execution review; and
- test-only validation of those public artifacts.

After handoff, an independent custodian MAY construct the fresh corpus, freeze
the independent oracle, execute the bounded search once, minimize a reproduced
case in the separate phase, and publish the required result artifacts.

Pulse 22 grants no authority for:

- Ferris production code, CLI, API, dependency, output, stream, result-class,
  or exit-map changes;
- hidden or old fixture access;
- candidate retries, scores, rescoring, favorable-result selection, or
  certification;
- owner-system, repository, network, credential, account, privileged, or
  mutable external-system actions; or
- PLATFORM-001 advancement.

## Precommitted disclosure

The disclosure tier is permanently fixed to `sanitized-reproducer` before
candidate generation. It MUST NOT be lowered, raised, or reinterpreted after a
seed, case, target output, or result is observed.

The
[prospective post-score diagnostic release protocol](POST_SCORE_DIAGNOSTIC_RELEASE.md)
governs the final public reproducer receipt. Its legacy `score_attempt` and
`scorer_attempt` field names MUST both remain `1` for schema compatibility,
but they identify this program's one immutable diagnostic search disposition;
they do not make Pulse 22 a score or certification program.

## Public source boundary

The reference classifier and generator MAY use only:

- the public Pulse 14 `profile-diff` input, metadata, result, privacy, and
  bound rules;
- [`PUBLIC_CONTRACT.md`](PUBLIC_CONTRACT.md), excluding every withheld field;
- [`IDENTITY.md`](IDENTITY.md);
- the public command-result and profile-diff schemas under
  [`schemas/`](schemas/README.md);
- the public Pulse 19 matrix as a list of already exercised branch categories,
  not as candidate bytes, generator code, expected records, or an oracle; and
- this contract and the prospective diagnostic release protocol.

The classifier MUST be an independent custody implementation. It MUST NOT
call, link, import, copy, or derive expected values from Ferris production
code, Ferris test helpers, the Pulse 19 harness, captured candidate output, or
Pulse 17 material.

## Private seed and deterministic generation

Before any case is generated, the custodian MUST:

1. create a fresh deterministic generation seed;
2. keep the seed private to custody;
3. commit its exact bytes, creation time, program ID, classifier digest, and
   generator version to an append-only custody record;
4. record a public `sha256:` commitment without disclosing the seed; and
5. freeze the generator and independent classifier digests.

Generation MUST then be deterministic:

1. enumerate the mandatory coverage atoms and interactions below in document
   order;
2. derive choices from `SHA256(seed || NUL || domain || NUL || counter)`;
3. construct only public-rule-valid candidate descriptions;
4. greedily select the candidate covering the most remaining obligations;
5. break ties by the candidate description digest; and
6. stop when every obligation is covered or 512 unique candidates exist.

If coverage cannot close within 512 unique candidates, custody MUST stop
before execution and publish no `no-reproduction` claim. Candidate IDs,
descriptions, inputs, and ordering MUST be frozen in one private manifest
before the first process launch.

## Exact search cardinality

The frozen maximums are:

| Bound | Maximum |
|---|---:|
| Unique logical cases | 512 |
| Windows launches per reached case | 1 |
| Ubuntu WSL2 launches per reached case | 1 |
| Cases per platform | 512 |
| Total search process launches | 1,024 |
| Candidate retries | 0 |
| Search executions | 1 |

Each candidate is one atomic cross-platform pair. The custodian MUST launch
the immutable binary directly once on Windows x86-64 and once on Ubuntu 24.04
WSL2 x86-64, retain both rows, and only then evaluate the pair. If either row
reproduces the target category, no later search candidate may launch.

A launch, timeout, stream-read, cardinality, manifest, classifier, or custody
failure invalidates the search. It MUST NOT be retried or converted into
`no-reproduction`.

## Mandatory fresh coverage

Coverage obligations are not a Cartesian-product requirement except where an
interaction is explicitly stated. Every atom and interaction MUST appear in
the pre-execution private coverage ledger and the final public aggregate
coverage report.

### Result classes, exits, records, diagnostics, and routes

Every public class MUST be independently classified and executed in JSON:

| Class | Exit | JSON route | Record | Diagnostics |
|---|---:|---|---|---|
| `success` | 0 | stdout only | non-null | empty |
| `difference` | 1 | stdout only | non-null | empty |
| `invalid` | 2 | stderr only | null | exactly one matching class |
| `unsupported` | 4 | stderr only | null | exactly one matching class |
| `incomplete` | 5 | stderr only | null | exactly one matching class |
| `blocked` | 7 | stderr only | null | exactly one matching class |

Human mode MUST include fresh byte-distinct partners for at least one
`success` and one `difference` case. Each pair MUST agree on actual exit,
stream route, profile metadata, changed and unchanged sections, change paths
and kinds, value digests, and raw-value non-disclosure.

### Output-visible metadata

The sites are `profile_id`, `revision`, `consumer`, and every JSON object key.

For each site, visible ASCII `!` through `~` MUST cover UTF-8 byte lengths
`0`, `1`, `255`, `256`, and `257`. The zero and 257-byte cases are expected
negative boundaries. Each site MUST also cover:

- at least one ASCII control byte;
- at least one non-ASCII Unicode scalar encoded as UTF-8;
- a slash `/`;
- a tilde `~`; and
- a combined slash-and-tilde key.

Non-ASCII lengths are measured in UTF-8 bytes, never characters. Generated
metadata is harmless and MUST NOT contain secrets, credentials, personal
data, Pulse 17 identifiers, or quarantine-derived strings.

### JSON values, members, duplicates, and pointers

Section values MUST cover:

- `null`, `false`, `true`, string, number, array, and object;
- empty and nonempty arrays and objects;
- nested arrays and objects;
- added, removed, and changed leaves;
- array index shifts and added or removed empty containers;
- equivalent reordered object members in before, after, and both inputs; and
- JSON Pointer keys containing `/`, `~`, and both, with exact `~1` and `~0`
  escaping.

Number source representations MUST include exactly these public lexical
families at least once:

```text
0
-0
1
-1
1.0
1e0
1E+0
1e-0
9007199254740991
-9007199254740991
```

Duplicate object members MUST cover duplicate depths `0`, `1`, `2`, `8`, and
`32`, where zero is the top-level object and positive values are nested
object depth. Duplicate-bearing text is rejected before canonicalization and
MUST NOT be normalized into a valid object.

### Input order, failure position, and paths

The public read order is before input first, then after input. Coverage MUST
include:

- one fresh difference pair in both before/after role orders;
- before failure with an otherwise valid after input;
- valid before input followed by after failure;
- both inputs failing, proving before-failure precedence; and
- equivalent first content at relocated paths followed by the same after
  failure.

Both before and after positions MUST cover missing paths, non-file paths, and
regular files using these path-form families where constructible:

- simple relative;
- relative with `.`;
- relative with reducible and unreducible `..`;
- Windows drive absolute;
- Windows extended-path absolute;
- Windows UNC;
- Unix absolute; and
- mixed separators.

The public lexical normalizer MUST independently cover extended-prefix
stripping, backslash replacement, empty and dot component removal, reducible
dot-dot popping, rooted dot-dot discard, relative dot-dot preservation,
repeated-separator collapse, drive-case preservation, and the distinction
between `C:/rooted` and relative `C:relative`.

The classifier MUST use declared filesystem setup and public lexical rules. It
MUST NOT infer path behavior from target output.

### Exact product boundaries

Both before and after positions MUST cover input byte sizes:

```text
1048575
1048576
1048577
```

Fresh comparisons MUST cover emitted leaf/path change counts:

```text
9999
10000
10001
```

The 1 MiB input boundary is inclusive. The 10,000-change boundary is
inclusive. A bound failure MUST NOT be accepted as a truncated or partial
success.

### Required interaction coverage

The private coverage ledger MUST close these interactions:

- metadata site by visible-ASCII byte boundary;
- metadata site by character kind for nonempty values;
- input position by path state by path-form family;
- input position by input byte boundary;
- JSON value kind by before/after member-ordering variant;
- duplicate depth by before/after failure position;
- expected result class by JSON route; and
- success/difference by JSON/human format pair.

## Independent reference classifier

The classifier MUST be frozen before generation and MUST emit, for every case:

- expected result class;
- expected mapped exit;
- expected diagnostic class or absence;
- expected record nullability;
- expected stream route;
- expected JSON or human format;
- applicable public rule citations; and
- classifier identity and case-description digest.

The public class map is:

```text
success=0
difference=1
invalid=2
unsupported=4
incomplete=5
blocked=7
```

For every JSON row, custody MUST independently record:

- expected class and exit;
- emitted envelope class and `process_exit_code`;
- actual operating-system exit;
- emitted diagnostic class;
- record nullability;
- stdout/stderr route;
- complete retained stream digests; and
- framing and parse disposition.

The target `process-exit-agreement` category is reproduced if any of these
predicates is false:

```text
expected_exit == public_map(expected_class)
emitted_class == expected_class
emitted_exit == public_map(emitted_class)
emitted_exit == expected_exit
actual_os_exit == emitted_exit
actual_os_exit == expected_exit
```

Diagnostic-class, record-nullability, framing, privacy, or route mismatches
MUST be recorded and stop the search, but they MUST NOT be relabeled as
`process-exit-agreement` unless one of the six predicates above also fails.

The classifier and collector MUST preserve mismatches exactly. They MUST NOT
normalize an exit, repair an envelope, move a stream, synthesize a diagnostic,
or select a favorable interpretation.

## First mismatch and separate minimization

After a completed cross-platform pair reproduces the target category, custody
MUST:

1. stop the search before the next candidate;
2. make the first candidate description, raw harmless inputs, process rows,
   outputs, exits, and digests immutable;
3. start a separately identified diagnostic minimization phase; and
4. retain explicit lineage from the first reproducer to every derived
   candidate.

Minimization is bounded to:

| Bound | Maximum |
|---|---:|
| Transformation attempts | 128 |
| Direct launches per transformation | 2 |
| Minimization process launches | 256 |
| Candidate retries | 0 |

Each lineage row MUST contain transformation index and ID, parent and child
digests, exact public transformation, input file digests, both platform
observations, predicate results, and accept/reject disposition.

The transformation order and acceptance rule MUST be frozen before the first
transformation. A child may replace the current minimum only when it preserves
the target category under the public predicate on both required platforms and
is strictly smaller under the precommitted tuple:

```text
(regular_file_count, total_input_bytes, total_json_nodes, total_argv_bytes,
 lexical_description_digest)
```

Derived public candidates may be executed because this is a post-search
diagnostic phase. They remain fresh Pulse 22 material and MUST never read,
touch, compare with, or infer Pulse 17.

## Result dispositions

The machine-readable declaration/result uses:

- `authorized-unexecuted`: public contract frozen; no seed, cases, or
  processes exist;
- `reproduced`: a target predicate failed, minimization completed, a public
  reproducer and release receipt were published, and both platforms reproduce
  the category;
- `no-reproduction`: all mandatory coverage completed within the frozen
  search bounds with no target predicate failure;
- `incomplete-after-reproduction`: a target predicate failed but bounded
  minimization, cross-platform confirmation, zero-overlap, or publication
  could not complete; or
- `invalid`: generation, classifier, launch, stream, cardinality, coverage,
  custody, or declaration integrity failed without a valid completed result.

`incomplete-after-reproduction` preserves the observed mismatch and MUST NOT
be changed to `no-reproduction`. None of these dispositions is a score,
certification result, support claim, or Pulse 17 status change.

## Reproduced-category publication

A completed `reproduced` disposition MUST publish a minimal public directory
containing:

- a README with the category, limitations, and non-certification statement;
- exact direct Windows and Ubuntu WSL2 commands;
- every raw harmless input required to reproduce;
- executable, input, command, stdout, stderr, and aggregate digests;
- actual exits, emitted classes and exits, diagnostics, record nullability,
  and routes for both platforms;
- the complete minimization lineage;
- the public rule citations and failed predicate; and
- a valid `ferris.post-score-diagnostic-release/v1` receipt.

The directory remains subject to the prospective release bounds: at most 16
regular files, 1,048,576 total bytes, four commands per platform, 60,000 ms
per command, and 1,048,576 retained bytes per stream.

All zero-overlap counts MUST equal zero. Pulse 22 code, generator, classifier,
and minimizer MUST NOT open Pulse 17 to perform this check. A quarantine
custodian MAY return only a non-disclosing zero-count attestation from an
already authorized one-way denylist or sealed comparison boundary. If a
zero-overlap attestation would require exposing or reopening Pulse 17, the
program MUST stop with `incomplete-after-reproduction`.

The receipt's `original_result` refers to the immutable Pulse 22 diagnostic
search result, never Pulse 17. The fresh private search package and public
reproducer MUST both record `future_certification_eligible:false`.

## Bounded no-reproduction publication

A completed `no-reproduction` disposition MUST publish:

- exact generated and executed case cardinality;
- exact per-platform process counts;
- zero retry count;
- seed commitment digest, without the seed;
- generator, classifier, case-manifest, coverage-ledger, executable, input
  aggregate, output aggregate, and process-record aggregate digests;
- coverage counts for every mandatory atom and interaction;
- platform, toolchain, command, timeout, and stream bounds;
- confirmation that no target predicate failed; and
- the statement `bounded no-reproduction; no fix authority`.

Individual non-reproducer case bytes need not be published. The report MUST
not claim the category cannot occur, explain Pulse 17, establish
certification, or authorize a product change.

## Custody handoff

At handoff, the public declaration MUST remain
`authorized-unexecuted` with:

- no seed or seed commitment value;
- no case manifest or case digest;
- no selected candidate;
- no generated input;
- no executable selection;
- no process record;
- no result; and
- no reproducer.

The independent custodian must next freeze, in order:

1. custody identity and workspace;
2. immutable Ferris cutoff and executable digest;
3. independent classifier source and digest;
4. deterministic generator source and digest;
5. private seed commitment;
6. generated case and coverage manifests; and
7. the one-execution launch authorization.

This repository change performs none of those custody actions.
