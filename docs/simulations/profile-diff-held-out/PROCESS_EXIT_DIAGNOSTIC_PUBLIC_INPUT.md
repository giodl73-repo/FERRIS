# Independent Process-Exit Diagnostic Public-Input Authority Contract

Status: Authorized; unexecuted
Program:
`FERRIS-P32-INDEPENDENT-PROCESS-EXIT-DIAGNOSTIC-PUBLIC-INPUT`
Schema: `ferris.process-exit-diagnostic-public-input/v1`
Disclosure tier: `sanitized-reproducer` (precommitted before generation)

## Purpose

This contract authorizes one new independent diagnostic program for the
released public category `process-exit-agreement`. It combines only the
normalized Pulse 25 collector, the Pulse 27 exact-two-pair adapter, and the
complete public Pulse 31 `ferris.profile-evidence/v0` input contract.

Pulses 22, 24, 26, 28, and 30 remain permanently invalid, permanently
non-retryable, and unable to produce category conclusions. Pulse 32 is not
their retry, resume, reseed, rescore, reuse, continuation, correlation, or
inference. Every prior category conclusion remains null.

This repository change is governance and test-only. It creates no custody
workspace, package copy, seed, classifier, generator, case manifest, corpus,
candidate, process row, pair seal, result, or reproducer. It MUST NOT invoke
the adapter, a verifier, Ferris, or an owner command.

## Authority boundary

Before independent handoff, this repository may contain only this contract,
one closed Draft 2020-12 schema, one `authorized-unexecuted` declaration,
public mutation controls, one nine-role review, the Pulse 32 record, and
test-only validation. No Ferris production source, dependency, CLI, API, output,
exit-map, stream-route, score, certification, support, fix, network,
credential, owner mutation, or PLATFORM-001 authority is granted.

## Permanently closed programs


| Program | Required disposition | Candidate retries | Category conclusion | Permanently closed |
|---|---|---:|---|---|
| Pulse 22 | `invalid` | `0` | null | `true` |
| Pulse 24 | `invalid` | `0` | null | `true` |
| Pulse 26 | `invalid` | `0` | null | `true` |
| Pulse 28 | `invalid` | `0` | null | `true` |
| Pulse 30 | `invalid` | `0` | null | `true` |

For each closed program, retry, resume, reseed, rescore, reuse,
continuation, correlation, and inference MUST remain `false`. Pulse 30 also
retains its passed normalization/package/preflight facts, zero generated
cases, zero candidate processes, and permanent further-launch prohibition.

## Immutable execution cutoff


Any later execution MUST use Ferris commit
`29517d732db13cc2ffa304684b344f3538ab587d`.

That cutoff contains the complete Pulse 31 public input artifacts and the
Pulse 29 normalized public infrastructure, but it does not contain this
Pulse 32 authority. Custody MUST independently verify the commit, launch its
frozen executable directly, and verify every Pulse 32 authority artifact is
absent from the cutoff. A different or authority-bearing cutoff invalidates
the package before candidates.

## Inherited normalized public infrastructure


Pulse 32 inherits every Pulse 30 normalization, package, adapter preflight,
freshness, coverage, oracle, search, collection, minimization, and publication
rule without weakening. Cutoff-specific references advance only to the exact
Pulse 32 cutoff above.

Before package copy, custody MUST materialize that cutoff in a new isolated
Git checkout with `core.autocrlf=true`, run `git check-attr text eol --
<path>` for all 36 Pulse 25/Pulse 27 release files, require `text=set` and
`eol=lf`, verify 36/36 LF files with zero CR bytes, verify the Pulse 29
receipt raw digest
`sha256:f75bf43fe47c07e8af7e5ee6148156fd272df47d0fc4de87d47ea0eb08f70225`,
and pass all 76/76 bindings: 22 Pulse 25, 45 Pulse 27, and nine collector
identity checks. Any failure closes `invalid-before-candidates`.

Only after that gate may custody copy exactly the 20 Pulse 27 manifest-listed
files into a new isolated package, recompute all 20 raw digests and the four
adapter/test/collector/release aggregates, and verify all report, receipt,
and seal bindings. No other file or prior custody byte is permitted.

The mandatory adapter preflight remains exactly one invocation, two Windows/
Ubuntu pairs, four process rows, two pair seals, and exactly two fresh
platform verifiers enforcing whole-store cardinality `2/2/2`, with zero
retries and zero interrupted-write residue. Generation is prohibited until
the complete adapter preflight passes.

## Exact Pulse 31 public input bindings


All nine artifacts MUST be obtained as exact Git blobs by path from the
immutable cutoff. A converted working-tree copy is not an equivalent raw
binding. Every blob is UTF-8/LF, LF-terminated, and contains zero CR bytes.

| Artifact | Bytes | Raw SHA-256 |
|---|---:|---|
| `INPUT_PROFILE_EVIDENCE.md` | 9129 | `sha256:26fdb4b9eed558f1f03a66eaec13749bfbad7ea4612c6f7e58bb8e7b79e69295` |
| `ferris.profile-evidence.v0.schema.json` | 3108 | `sha256:67946b1a392d2d7537d487d343bee31439606c76b2d71862b97ff46641c3d62b` |
| `profile-evidence-v0-positive-scalars.json` | 487 | `sha256:650b15ffcadb46ff3673f889ef35900a14e9f8bc09e824739d0117f5ebeadf69` |
| `profile-evidence-v0-positive-arrays.json` | 587 | `sha256:4f2c99b179253eb7ea6aa3ba227321dffaabbb3a81a11c786018ff8217d56cc8` |
| `profile-evidence-v0-positive-objects.json` | 785 | `sha256:57bbd633d73ee9633b030d79f3452797928e93bc8766c84d2aeaea3d013bdd7a` |
| `profile-evidence-v0-positive-nested-mixed.json` | 1064 | `sha256:b4b682a0673899ac3cdf757bbd5280d2330cd3e72a423922ffe8cdfda1ad5dcd` |
| `profile-evidence-v0-positive-boundary-minimum.json` | 407 | `sha256:a07134d0ebf010515c3a057a3f7498105e23b351dc1b67fa030b79dec2cd68fd` |
| `profile-evidence-v0-positive-boundary-maximum.json` | 1427 | `sha256:75dbf918361795865dbe83f1b25b8aec1f23aad4475ce88f6620dd4d3efa0665` |
| `profile-evidence-v0-mutations.json` | 8818 | `sha256:b33985e51f54c2ed0121b94571b622ee47bbd00450c8ab1c3d65d0f463276158` |

The schema digest is exactly
`sha256:67946b1a392d2d7537d487d343bee31439606c76b2d71862b97ff46641c3d62b`.
The six positive fixtures cover scalar, array, object, nested mixed,
one-character, and 256-character boundaries.

## Exact 33 mutation-control digests


For each mutation object, compute SHA-256 over UTF-8 compact JSON with object
member names recursively sorted lexicographically and no insignificant
whitespace. The algorithm identifier is
`sha256-canonical-json-sort-keys-v1`. The exact public digests are:

| Control | SHA-256 |
|---|---|
| `unsupported-schema` | `sha256:e477472659f4eadcf454c6b262a5cc5328acf52352daaa2422ffa66d31e48dec` |
| `missing-schema` | `sha256:7d214af6d5c6e1c4f405bdbd0a5a717c67bf9a2c726cf76a5d7666576cf8e45b` |
| `non-string-schema` | `sha256:3e3f26cad5d8d97ad896daf421fa7ed00cffd00451cde27169171c29ca76c876` |
| `extra-root-member` | `sha256:be3377394ad7d6b8e3750c1d21cfb01eee0fbf08bc3d77e6458ce177bc7ebcc8` |
| `missing-profile-id` | `sha256:7ef0793688d8910707a2d3aeccfd559abfc0111c99c24ad02ea1e5038594a32c` |
| `empty-profile-id` | `sha256:caa8c2c0c8fbd962d00fd4765d26007be954a9ead3c0bab63033b021e7d71d5a` |
| `empty-revision` | `sha256:10770e9cef25d7e8aacd1f6061ed85de2bbac3b310837fde8b02fe0d91574a0f` |
| `empty-consumer` | `sha256:00e8cb791101cae101e7fe44b4e068f8eceb2310027cb3b14c406b89329dd66c` |
| `space-profile-id` | `sha256:c026c9850ef7380b46c58f88008d73b5e3f6cdc3055add6f18668a086f9eef8e` |
| `control-revision` | `sha256:280da5adf46e018e839dbf1624e3da3494d2356158ce3214237e479b51f57dd8` |
| `unicode-consumer` | `sha256:d2c88131b2f6e13522dcf19f46c906515bd9c8009406148d6d6f4aaaf16469cc` |
| `overlong-profile-id` | `sha256:43048e0d9117606e2f5f9c016144ef79c71a0d7ad2b952c600467afef888d029` |
| `overlong-revision` | `sha256:5236080e3259ac9bbd378d51eb7b1091b9232c1573bd9ff2f2bb9f3c856441a4` |
| `overlong-consumer` | `sha256:87bad9c2a4986fe4147afaba5a308850373f78590871b302d3a07a60dca1bd0b` |
| `sections-not-object` | `sha256:e61241fd949a1c8d4a555b72a3128d9d2b45dcda6d5f7992d063c328b3160b21` |
| `missing-section` | `sha256:2cdb0b485caa6dc4ed8051dc963f3c75fca2ec3e41f719f06600a6e149828100` |
| `extra-section` | `sha256:04cf3dad9282460815d37721e0347020e44918482d537625c3f9cd13e3a88ecb` |
| `invalid-root-key-space` | `sha256:9df7993973e6622e388af4c776e2a13613eaa80cbe233b86ba632bc6a394d58e` |
| `invalid-sections-key-empty` | `sha256:2dce97e13ca09894c488538cdc335f36e8c0c4e4ae8a81c731341bbbe1b48597` |
| `invalid-nested-key-empty` | `sha256:328d3eb3067d366ef90a7b83b97b94a3a4fa96819ac04b0dae6a77286f3a2a0c` |
| `invalid-array-object-key-space` | `sha256:ad831936f192f33f1df7e14e50798d65ea3488a4f0f5600cd86cda8260e40194` |
| `invalid-deep-key-unicode` | `sha256:2964a0bb408a156aa812d07a0f2f8c0be1d18f845505355b8bc11d2b1a4a48e7` |
| `invalid-deep-key-control` | `sha256:81a66856145f626e935b90dd20beb35af041e38196822c8ca7d5cc9e07c4735f` |
| `overlong-nested-key` | `sha256:ac5c464d1e7c1bd3cd3c8ab57952933216ecac17dfa77f6c8c525dd4fd860a08` |
| `duplicate-root-member` | `sha256:197525d8f85b6a07e534ad9723f41f6aa12203c1ebf56465d800f6775d3b1b3c` |
| `duplicate-sections-member` | `sha256:bfef44687c944be08bc68a76d7aa837775be30dcb74acfb1992d9e2ba25d12d7` |
| `duplicate-nested-member` | `sha256:d11d38d352111f2f397707dbdf7c8950a8f9ef66d6cdc3efec67d60f2f031e19` |
| `malformed-json` | `sha256:e60f5eaa6575048b17e71d9b222ff4e01e99a8fdb06e05215cea4b2005538f8f` |
| `empty-file` | `sha256:add87da5dcf45be96432d2d985beddff80038c653e18edea3326a27a9a596c11` |
| `oversized-file` | `sha256:f969ddd364160fd7616de24053e8eebd9cf86556ac133d158a71db75519377d7` |
| `missing-input` | `sha256:c2d8890264b89a3265f5891011232b354b974a4a5baa6e72e8cc08b19407ed47` |
| `non-file-input` | `sha256:cd47c8db8b80734bd85644735f1108563956dfe3292593d0f1564bcc31c828a7` |
| `unreadable-input` | `sha256:f78010922d5cba1bbf07fa0b5109b5673597d360743b370aff4e4337c56f72d6` |

The mutation file raw digest, its schema
`ferris.profile-evidence-v0-mutations/v1`, base fixture
`profile-evidence-v0-positive-nested-mixed.json`, all 33 IDs, and all 33
per-control digests MUST agree before self-validation begins.

## Public-only contract self-validation


After the adapter preflight passes and before freezing a generator or
classifier, the custodian MUST use only the nine public artifacts above to:

1. verify the contract, schema, six fixture, and mutation-file raw digests;
2. verify all 33 per-control canonical digests;
3. validate all six positive fixtures as accepted inputs;
4. construct every declared mutation exactly as the public contract defines;
5. classify all 33 mutations with the exact declared first-applicable result
   class and diagnostic; and
6. record 39/39 classifications, zero failures, and a passed public contract
   self-validation receipt.

The authorized read scope is exactly `INPUT_PROFILE_EVIDENCE.md`, the public
schema, the six positive fixtures, and the mutation file. Ferris production
source, Ferris tests, prior custody, captured output, and hidden material are
outside scope. The generator and classifier MUST be independently implemented
from only these public rules. Source/test inspection or a failed/missing
self-validation makes the package `invalid-before-generation`, prohibits
generation and candidate launch, and requires a later authority for repair.

## Fresh diagnostic material


Only after both preflights pass may custody freeze a new identity, workspace,
private seed and commitment, independently implemented generator and
classifier, case and coverage manifests, and fresh corpus. None may inherit
bytes, identities, descriptions, choices, ordering, or output from Pulses 17,
19, 22, 24, 26, 28, or 30. Implementation authors MUST NOT construct, select,
inspect, or execute candidates.

Deterministic generation remains:

```text
SHA256(seed || NUL || domain || NUL || counter)
```

## Inherited coverage and oracle


The declaration retains every Pulse 30 coverage and oracle field, including
all result classes, metadata boundaries, recursive JSON kinds, numeric lexical
families, duplicate depths, ordering, path, byte, and change-count obligations;
exactly eight mandatory interactions; exactly eight compared oracle fields;
and exactly six target predicates. The public-only classifier MUST NOT infer
expected values from Ferris source, tests, output, prior harnesses, or private
material.

The target is reproduced only when at least one predicate is false:

```text
expected_exit == public_map(expected_class)
emitted_class == expected_class
emitted_exit == public_map(emitted_class)
emitted_exit == expected_exit
actual_os_exit == emitted_exit
actual_os_exit == expected_exit
```

Adjacent framing, privacy, nullability, diagnostic, or routing failures stay
visible but are not relabeled as `process-exit-agreement` without a failed
target predicate.

## Search, collection, minimization, and publication bounds


| Bound | Maximum |
|---|---:|
| Unique logical cases | 512 |
| Cases per platform | 512 |
| Search processes | 1,024 |
| Search executions | 1 |
| Candidate retries | 0 |
| Minimization transformations | 128 |
| Minimization processes | 256 |

Collection remains transactional: each platform record is durably synced, a
pair seal is written only after both records, and a fresh process reloads and
verifies identity, cardinality, streams, exits, and seals before classification.
Any post-first-candidate package failure stops all later launches.

Publication remains either a bounded sanitized reproducer with its exact
receipt and zero-overlap controls or the statement
`bounded no-reproduction; no fix authority`. Invalid results require a
public-safe blocker and exact counts, and their category conclusion is null.

## Declaration and mutations


The declaration identity is `sha256:88bdbd263fed865e94d16cbd0e6f78a2f330cdae5788f7d7bf93c51afd758812`.
The positive declaration is `authorized-unexecuted`, and every activity,
verification, generation, candidate, retry, and conclusion field remains at
its zero/false/null initial value. The public mutation suite contains exactly
538 rejection controls and includes every inherited Pulse 30 control plus
closed Pulse 30, cutoff, contract, schema, six fixture, 33 per-control,
public-only read-scope, self-validation, generator/classifier, result, unknown
member, and identity mutations.

## Custody handoff


The declaration is ready only for a new independent validation custodian.
It authorizes one bounded search execution after all gates pass. It does not
select a custodian, create a workspace, freeze a seed, or execute anything.

## Stop conditions


Stop rather than widen if work would use another cutoff; place authority in
the cutoff; bypass normalization, package, adapter preflight, public-input
digest, or self-validation gates; read Ferris source/tests for generation or
classification; access prior custody or hidden material; retry or infer from a
closed program; weaken inherited coverage/oracle/search/minimization/
publication bounds; execute under this repository change; or alter production
behavior, Pulse 17, any closed result, or PLATFORM-001 status.
