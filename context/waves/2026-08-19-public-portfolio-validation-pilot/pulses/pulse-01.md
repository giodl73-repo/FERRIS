# Pulse 01: Public Four-Repository Pilot

Status: Complete
Implementation authority: Bounded to this document
Budget: One measurement pass, one correction if evidence review finds a defect

## Outcome

Run the unchanged `federated-validation-plan` across four clean public Rust
repositories at exact revisions:

| Workspace | Revision | Packages | Pilot relationship |
|---|---|---:|---|
| FERRIS | `caed7059c86b2e6c496f389ffce07c5b6d2d1709` | 2 | contract producer |
| PARLOR | `3ad7cd6b87aa4d248c6785c3f48b30d5c048d789` | 6 | explicit migration consumer of FERRIS |
| RUNE | `3eae3c2f633f3c638308452029e199db6056d887` | 6 | explicit migration consumer of FERRIS |
| ICELINES | `e6bc0184fdba6f830d722a93fbd7810c03822191` | 7 | unrelated control |

Every repository was clean before and after the pilot. Each root manifest
passed:

```console
cargo +nightly metadata --format-version 1 --no-deps --offline --locked \
  --manifest-path <REPOSITORY>/Cargo.toml
```

## Results

Five local planning samples were collected per scenario:

| Scenario | Direct result | Relationship/application fallback | Required scopes | Median |
|---|---|---|---:|---:|
| Ferris validation-plan schema | FERRIS full-workspace fallback, 2 packages | PARLOR and RUNE relationship fallback; ICELINES omitted | 3/4 | 429.414 ms |
| `parlor-go` | `parlor-go`, `parlor-cli` | none; other repositories omitted | 1/4 | 383.896 ms |
| `rune-derive` | `rune-derive`, `rune-adopter`, `rune-shape-calculator` | none; other repositories omitted | 1/4 | 405.385 ms |
| `icelines-core` | all 7 ICELINES packages | none; other repositories omitted | 1/4 | 389.197 ms |
| Application policy | none | all 4 application fallback | 4/4 | 396.250 ms |

## Findings

### FERRIS-PVP-01: Owner metadata remained independent

Observed: all four workspaces passed locked/offline Cargo metadata separately,
and the pilot retained one direct plan only for an explicitly changed owner.

Implication: the real-repository pilot preserved Cargo workspace, package,
resolution, and lock authority rather than constructing a shared graph.

Confidence: high for these exact revisions.

### FERRIS-PVP-02: Existing consumer projections reproduced

Observed: PARLOR selected its documented `parlor-go` plus `parlor-cli` closure.
RUNE selected its documented procedural-macro anchor plus two example adopters.

Implication: application-level collation preserved the already reviewed
single-workspace consumer behavior instead of replacing it.

Confidence: high for the selected package inputs and exact revisions.

### FERRIS-PVP-03: Unrelated repositories remained omitted

Observed: PARLOR, RUNE, and ICELINES package changes each required only their
direct workspace scope. The other three repositories remained `not_selected`.

Implication: membership in one Application Definition does not itself force
global validation.

Confidence: high for the explicit pilot relationships.

### FERRIS-PVP-04: Contract migration widened conservatively

Observed: a changed Ferris validation-plan schema required the Ferris nested
full-workspace fallback plus PARLOR and RUNE relationship fallbacks. ICELINES
remained omitted.

Implication: explicit governance relationships can preserve consumer review
without widening unrelated portfolio applications.

Confidence: medium. The migration relationship is a deliberate pilot policy,
not discovered dependency truth.

### FERRIS-PVP-05: Unknown application ownership remained safe

Observed: an application-owned policy path outside all four workspace roots
widened all four workspaces.

Implication: real-repository composition retained the same conservative
fallback demonstrated by synthetic controls.

Confidence: high for this application definition.

## Environment and cleanup

- Windows 11 Enterprise Insider Preview `10.0.26310`;
- Intel Core i7-12800HX, 24 logical processors;
- 34,042,929,152 visible memory bytes;
- `rustc 1.99.0-nightly (1a98b1e13 2026-08-07)`;
- `cargo 1.99.0-nightly (c79e8f894 2026-08-04)`;
- five samples per scenario; and
- temporary Application Definition and policy files removed after measurement.

All four repositories remained clean. No validation command, test, build,
format, Clippy, consumer checker, network operation, or child-repo mutation
was performed by Ferris.

## Claim boundary

This is one local public-repository pilot, not production adoption or a
support claim. The result does not establish relationship correctness outside
the explicit migration policy, validation completeness, build/test savings,
remote reproducibility, or acceptable latency. Planning still loaded Cargo
metadata sequentially for all four repositories.
