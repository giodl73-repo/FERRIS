# Final Independent Process-Exit Diagnostic Normalized Public-Adapter Contract

Status: Authorized; unexecuted
Program:
`FERRIS-P30-INDEPENDENT-PROCESS-EXIT-DIAGNOSTIC-NORMALIZED-PUBLIC-ADAPTER`
Schema: `ferris.process-exit-diagnostic-normalized-public-adapter/v1`
Disclosure tier: `sanitized-reproducer` (precommitted before generation)

## Purpose

This contract authorizes one final new independent diagnostic program for the
released public category `process-exit-agreement`. It uses only the normalized
public Pulse 25 collector and Pulse 27 exact-two-pair adapter after the Pulse
29 checkout-normalization pass.

Pulse 30 is not a retry, resume, reseed, rescore, reuse, continuation,
correlation, or inference from Pulse 22, Pulse 24, Pulse 26, or Pulse 28.
Those four programs remain permanently invalid, permanently non-retryable,
and unable to produce a category conclusion. Their category conclusions
remain null.

This repository change is governance and test-only. It creates no custody
workspace, package copy, executable, seed, classifier, generator, manifest,
corpus, process row, pair seal, candidate, result, or reproducer. It MUST NOT
invoke the adapter, a verifier, or Ferris.

## Authority

Before independent handoff, this repository may contain only:

- this normative contract;
- one closed Draft 2020-12 schema;
- one public `authorized-unexecuted` declaration;
- strong public mutation controls;
- one nine-role pre-execution review;
- the Pulse 30 pulse record; and
- test-only validation of public repository artifacts.

Pulse 30 grants no Ferris production source, CLI, API, dependency, output,
exit-map, stream-route, score, certification, support, product-fix,
owner-system mutation, network, credential, or PLATFORM-001 status authority.

## Permanently closed programs

Pulses 22, 24, 26, and 28 MUST retain all of these properties:

| Requirement | Value |
|---|---|
| Disposition | `invalid` |
| Candidate retries | `0` |
| Category conclusion | null |
| Permanently closed | `true` |
| Retry, resume, reseed, rescore, reuse, continuation, correlation, inference | all `false` |

Pulse 30 MUST NOT read, copy, reconstruct, correlate, or infer from any private
Pulse 17, 19, 22, 24, 26, or 28 material. The closed Pulse 28 public result
MUST remain byte-unchanged.

## Immutable Ferris cutoff

Any later execution MUST use Ferris commit
`cf6b3309c31e5da37d4a8e6655a781f4e92ef603`.

That cutoff contains the root `.gitattributes` LF rules and the complete Pulse
29 checkout-normalization receipt. It MUST NOT contain this Pulse 30
authority. Custody MUST verify the exact commit independently, launch the
cutoff executable directly, and freeze its digest before preflight.

A different cutoff, an authority-bearing cutoff, an indirect launch, or an
unverified executable invalidates the package before candidates.

## Mandatory cutoff materialization before copy

Before copying any package file, the independent custodian MUST:

1. create a new isolated Git materialization of the immutable cutoff;
2. configure that materialization with `core.autocrlf=true`;
3. copy no bytes from the current working tree or any prior custody workspace;
4. enumerate every regular file below the Pulse 25 and Pulse 27 release
   roots;
5. run `git check-attr text eol -- <path>` for every enumerated file;
6. require `text: set` and `eol: lf` for each file;
7. read all 36 byte-bound files and require 36 LF-framed files with zero CR
   bytes; and
8. independently repeat all 76 Pulse 29 binding checks and require 76 passes
   and zero failures.

The exact release-tree and binding cardinalities are:

| Check | Required |
|---|---:|
| Pulse 25 release-tree files | 14 |
| Pulse 27 release-tree files | 22 |
| Total LF files | 36 |
| Pulse 25 binding checks | 22 |
| Pulse 27 binding checks | 45 |
| Cross-release collector identity checks | 9 |
| Total binding checks | 76 |

The normative Pulse 29 receipt is
`docs/simulations/profile-diff-held-out/pulse-29-checkout-normalization/PULSE-29-CHECKOUT-NORMALIZATION-RECEIPT.json`.
Its raw digest MUST be
`sha256:f75bf43fe47c07e8af7e5ee6148156fd272df47d0fc4de87d47ea0eb08f70225`
and its payload identity MUST be
`sha256:92e245685cbb1b6ce938701a901c4de9b9202f9149537690e646d13a113deb40`.

Any attribute mismatch, CR byte, file-count drift, binding mismatch, failed
check, current-working-tree copy, or prior-custody access invalidates the
package before copy. Repair or another materialization requires a later
authority.

## Exact normalized Pulse 25 collector bindings

The declaration pins all nine normalized manifest entries by path, kind, byte
length, and raw SHA-256. These aggregate and release bindings are normative:

| Binding | Required digest |
|---|---|
| Public manifest | `sha256:621ed59a5b2124204180be109f69010ac18337a09816c8d28e67713f63efb419` |
| Source aggregate | `sha256:71b41689202e0ee3c956c9e5408284deac63e53004530b717a403266237d73a7` |
| Test aggregate | `sha256:5de010365b3c1297144de030c1738e998e9f55994dee1497d0600b178b2d3de9` |
| Complete bundle aggregate | `sha256:e296329ff56fad14eba2274d928f45c0fdf6a281db3d2d554c1cee3814d4b406` |
| Qualification report | `sha256:04491bea4828fd7329d622c84f9b186d7315dbb31d491176598ffee09be4499e` |
| Release receipt | `sha256:4ec9d50c4ff0f4ba8b65d57751fad28f2a1fcd610e67e664f1727baeb78aaf69` |
| Release seal | `sha256:f1d10da9395f2b9f3834da260b6f11e365153ed5b33a75b937d7c410d9c08e1e` |

Custody MUST recompute every manifest size and digest and the source, test,
and complete-bundle aggregates using
`sha256-length-path-filedigest-v1`.

## Exact normalized Pulse 27 adapter bindings

The only permitted source release is
`docs/simulations/profile-diff-held-out/pulse-27-preflight-adapter-release`.
The declaration pins all 20 manifest entries by path, kind, byte length, and
raw SHA-256. These bindings are normative:

| Binding | Required digest |
|---|---|
| Public manifest | `sha256:7a6e61dacb3d58ab6d8c75cf1267a70f7919219baadd34329b835640931e8d5e` |
| Adapter-source aggregate | `sha256:cdca8d4a0206c9553c637b9228511cfa07e401b9082d96c439d112e2b25c6071` |
| Adapter-test aggregate | `sha256:426bd87a7695bb2d5cefdb4c98fc4bef1524616100365656c2e3bc2c19747dff` |
| Collector-copy aggregate | `sha256:7a4645f3d3f5e7dcee709351d802e76d1ae6333a7a3b92412fe41d8ae656fc5b` |
| Complete release aggregate | `sha256:531113c7c8a50f1c71c446bc708e44549702623114625ea46f5aa874b6aea721` |
| Root-cause report | `sha256:5f1760b7f7cf318029ea24407ef20a087340af16eb2991d7d0b7b0495efded1c` |
| Qualification receipt | `sha256:94ec4237fd046281b9971e7eea67dc1ae7208996ed69b50ce8430a78cd0b6886` |
| Release seal | `sha256:8abcc449d4b4aff30ed3ade168fa59c7f159e68d3172180703971bb79f096a6e` |

All nine `collector/` files MUST be byte-identical to the corresponding Pulse
25 files. No collector modification, compatible substitution, reconstruction,
repair, or private source is authorized.

## Exact 20-file custody package

Only after every cutoff-materialization check passes may custody:

1. create a new isolated package workspace;
2. copy exactly the 20 Pulse 27 manifest-listed files, preserving relative
   paths;
3. copy no manifest, seal, unlisted file, private file, or prior-custody file;
4. independently recompute all 20 raw digests;
5. independently recompute the adapter-source, adapter-test,
   collector-copy, and complete-release aggregates; and
6. compare every recomputed value with the declaration.

Any missing or extra file, path drift, byte drift, hash mismatch, aggregate
mismatch, collector mismatch, or copy before normalization verification makes
the package `invalid-before-candidates`.

## Mandatory exact preflight

After cutoff, executable, normalization, package, and aggregate verification,
custody MUST run exactly one adapter invocation. That invocation MUST create:

- two Windows rows;
- two Ubuntu rows;
- four total process rows;
- two atomic Windows/Ubuntu pairs; and
- two pair seals.

After the complete store exists, custody MUST launch exactly two fresh
read-only verifier processes: one on Windows and one on Ubuntu. Each MUST
require exact whole-store cardinality `2/2/2`: two Windows rows, two Ubuntu
rows, and two pair seals.

Adapter invocations, pairs, and verifier retries are all zero. The complete
store MUST contain zero interrupted-write residue. Preflight rows are
infrastructure controls, not diagnostic candidates, and do not count toward
search bounds.

Any launch, durability, seal, join, cardinality, freshness, residue, stream,
or exact-count failure makes the package `invalid-before-candidates`,
permanently prohibits candidate launch, and cannot create a category
conclusion.

## Fresh independent diagnostic material

Only after the complete preflight passes may the diagnostic phase freeze:

1. a new custody identity and diagnostic workspace;
2. a new private deterministic seed and public commitment;
3. a new independently implemented classifier;
4. a new independently implemented deterministic generator;
5. new case and coverage manifests; and
6. a new fresh corpus.

None of those identities, bytes, sources, descriptions, choices, or ordering
may be inherited from Pulse 17, 19, 22, 24, 26, or 28. Implementation authors
MUST NOT construct, select, inspect, or execute candidates.

Generation MUST remain deterministic:

`SHA256(seed || NUL || domain || NUL || counter)`

## Inherited coverage and oracle

Pulse 30 inherits the Pulse 26 declaration's `coverage`, `oracle`,
`search_bounds`, `collection`, `minimization`, and `publication` objects
byte-for-byte and without weakening.

That inheritance includes:

- every result class, exit mapping, JSON route, nullability state, diagnostic
  cardinality, metadata boundary, JSON value kind, numeric lexical family,
  pointer-key kind, duplicate depth, ordering, path, byte, and change-count
  obligation;
- exactly eight mandatory interactions;
- exactly eight independently compared oracle fields;
- exactly six target predicates;
- no inference from Ferris production code, test helpers, prior harnesses,
  captured output, or private material; and
- transactional durable pair collection with fresh-process reload before
  classification.

The target category is reproduced only when at least one of these predicates
is false:

```text
expected_exit == public_map(expected_class)
emitted_class == expected_class
emitted_exit == public_map(emitted_class)
emitted_exit == expected_exit
actual_os_exit == emitted_exit
actual_os_exit == expected_exit
```

Adjacent diagnostic, framing, nullability, privacy, or route failures remain
visible but MUST NOT be relabeled as `process-exit-agreement` unless one of the
six predicates also fails.

## Search, minimization, and publication bounds

The inherited search bounds remain:

| Bound | Maximum |
|---|---:|
| Unique logical cases | 512 |
| Cases per platform | 512 |
| Windows launches per reached case | 1 |
| Ubuntu launches per reached case | 1 |
| Search process launches | 1,024 |
| Candidate retries | 0 |
| Search executions | 1 |

Search stops after the first complete pair reproduces the target category.
Every reached pair must be durably recorded, sealed, reloaded, and verified
before classification.

The inherited minimization bounds remain 128 recorded transformations, two
direct launches per transformation, 256 total minimization process launches,
and zero candidate retries. The first reproducer remains immutable.

Publication remains limited to `reproduced`, `no-reproduction`,
`incomplete-after-reproduction`, or `invalid`. A no-reproduction result MUST
state:

`bounded no-reproduction; no fix authority`

An invalid result MUST publish the public-safe blocker and exact counts, MUST
retain a null category conclusion, and MUST prohibit further launches.

## Public declaration

The public fixture remains `authorized-unexecuted`. All normalization,
copying, preflight, generation, search, minimization, and result counts are
zero or false. No identity, digest, path, candidate, process, seal, or result
from a future custody package appears in this repository change.

## Stop conditions

Stop rather than widen this authority if work would:

- execute the adapter, a verifier, preflight, or Ferris during this change;
- use another cutoff or an authority-bearing cutoff;
- copy before the 36/36 LF, per-file attribute, and 76/76 binding gates pass;
- modify a Pulse 25, Pulse 27, Pulse 28, or Pulse 29 public artifact;
- copy other than the exact 20 manifest-listed files;
- change the one-invocation, two-pair, four-row, two-seal, two-verifier,
  `2/2/2`, zero-retry, or zero-residue requirements;
- create generation material before preflight passes;
- weaken inherited coverage, oracle, search, collection, minimization, or
  publication requirements;
- reopen, retry, resume, reuse, correlate, or infer from a closed program; or
- change PLATFORM-001 from Draft.
