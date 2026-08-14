# Pulse 29: Public-Artifact Checkout Normalization

Status: Complete
Implementation authority: Git checkout attributes, public release rebinding,
public receipt, documentation, role review, and test-only validation only

## Goal

Make every byte-bound file in the Pulse 25 collector release and Pulse 27
adapter release materialize with LF bytes on every checkout, including Windows
with `core.autocrlf=true`.

## Boundary

Pulse 29 changes no Ferris production source, CLI, API, dependency, output,
exit map, stream route, diagnostic candidate, collector behavior, adapter
behavior, score, certification, fix authority, or PLATFORM-001 status.

The Pulse 28 result remains immutable. Its first mismatch remains the
historical expected CRLF-derived Pulse 25 manifest digest
`sha256:771f8521acbdada3388cfd15d61b565a590ff4f74c65bd768f7e114682b30c75`
against the LF Git-blob digest
`sha256:03322e9fe6a3df6c71161e5f3916c51cc66c9453e9f1f3141bcc703bd02d7a0d`.
Pulse 29 does not retry or repair that closed custody package.

## Checkout contract

Root `.gitattributes` contains anchored recursive rules:

```gitattributes
/docs/simulations/profile-diff-held-out/pulse-25-collector-source-release/** text eol=lf
/docs/simulations/profile-diff-held-out/pulse-27-preflight-adapter-release/** text eol=lf
```

The rules cover direct and nested `.py`, `.json`, `.md`, and any later
byte-bound file under those two release roots. Tracked source, test, and
documentation files were renormalized to their intended LF Git-blob bytes.

## Normalized bindings

Pulse 25:

- public manifest:
  `sha256:621ed59a5b2124204180be109f69010ac18337a09816c8d28e67713f63efb419`;
- source aggregate:
  `sha256:71b41689202e0ee3c956c9e5408284deac63e53004530b717a403266237d73a7`;
- test aggregate:
  `sha256:5de010365b3c1297144de030c1738e998e9f55994dee1497d0600b178b2d3de9`;
- bundle aggregate:
  `sha256:e296329ff56fad14eba2274d928f45c0fdf6a281db3d2d554c1cee3814d4b406`;
- qualification report:
  `sha256:04491bea4828fd7329d622c84f9b186d7315dbb31d491176598ffee09be4499e`;
- release receipt:
  `sha256:4ec9d50c4ff0f4ba8b65d57751fad28f2a1fcd610e67e664f1727baeb78aaf69`;
- release seal:
  `sha256:f1d10da9395f2b9f3834da260b6f11e365153ed5b33a75b937d7c410d9c08e1e`.

Pulse 27:

- public manifest:
  `sha256:7a6e61dacb3d58ab6d8c75cf1267a70f7919219baadd34329b835640931e8d5e`;
- adapter-source aggregate:
  `sha256:cdca8d4a0206c9553c637b9228511cfa07e401b9082d96c439d112e2b25c6071`;
- adapter-test aggregate:
  `sha256:426bd87a7695bb2d5cefdb4c98fc4bef1524616100365656c2e3bc2c19747dff`;
- collector-copy aggregate:
  `sha256:7a4645f3d3f5e7dcee709351d802e76d1ae6333a7a3b92412fe41d8ae656fc5b`;
- complete release aggregate:
  `sha256:531113c7c8a50f1c71c446bc708e44549702623114625ea46f5aa874b6aea721`;
- root-cause report:
  `sha256:5f1760b7f7cf318029ea24407ef20a087340af16eb2991d7d0b7b0495efded1c`;
- qualification receipt:
  `sha256:94ec4237fd046281b9971e7eea67dc1ae7208996ed69b50ce8430a78cd0b6886`;
- release seal:
  `sha256:8abcc449d4b4aff30ed3ade168fa59c7f159e68d3172180703971bb79f096a6e`.

## Materialization evidence

A disposable alternate Git index staged the resulting uncommitted state and
`git checkout-index` materialized it on Windows with
`core.autocrlf=true`. The verification inspected 36 release-tree files and
found 36 LF files and zero CR bytes.

The binding audit passed 76 of 76 checks:

- Pulse 25: 22 checks: one raw manifest, nine sizes, nine file digests, and
  three aggregates;
- Pulse 27: 45 checks: one raw manifest, 20 sizes, 20 file digests, and four
  aggregates; and
- cross-release collector identity: nine byte-for-byte checks.

The public
[checkout-normalization receipt](../../../../docs/simulations/profile-diff-held-out/pulse-29-checkout-normalization/README.md)
seals the materialization method, counts, and normalized hashes. Its raw
SHA-256 is
`sha256:f75bf43fe47c07e8af7e5ee6148156fd272df47d0fc4de87d47ea0eb08f70225`
and its payload identity is
`sha256:92e245685cbb1b6ce938701a901c4de9b9202f9149537690e646d13a113deb40`.
No build,
preflight, generation, diagnostic candidate, pair, seal, retry, or private
data access occurred.

## Evidence

- [Checkout-normalization receipt](../../../../docs/simulations/profile-diff-held-out/pulse-29-checkout-normalization/README.md)
- [Nine-role review](../../../../docs/plans/reviews/PULSE-29-PUBLIC-ARTIFACT-CHECKOUT-NORMALIZATION-ROLE-REVIEW.md)
- [Test-only validator](../../../../crates/ferris-cli/tests/public_artifact_checkout_normalization.rs)

## Decision

The two public release roots now have stable LF checkout semantics and
coherent normalized manifests, raw file digests, aggregates, receipts, and
seals. Pulse 29 adds no diagnostic authority and does not alter the closed
Pulse 28 result.
