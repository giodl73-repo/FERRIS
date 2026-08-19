# Pulse 01: Licensed Shared-Substrate Diamond

Status: Complete
Implementation authority: Bounded to evidence and repository licensing
Budget: One measurement pass and one evidence correction

## Outcome

Run the unchanged `federated-validation-plan` across five clean public Rust
repositories at exact revisions:

| Workspace | Revision | Packages | Diamond role |
|---|---|---:|---|
| FLETCH | `361fb7eda730bf660de92e47d502b91dcfaf473f` | 3 | shared producer |
| METIS-CORE | `afa595148430707ac87c622611c8d45b28b79c1e` | 1 | shared producer |
| BISECT | `7f35c4ebb974800a29f2881f73d0f031078da381` | 40 | FLETCH and METIS-CORE consumer |
| ROUTE | `db3c9a86ef9347bd3c0325681a2068847cb01700` | 7 | FLETCH and METIS-CORE consumer |
| ICELINES | `fa0bebab3a33c5501f7cb78b242ea7cd260be499` | 6 | FLETCH consumer |

Every repository was clean before and after the pilot. Each root manifest
passed locked/offline Cargo metadata independently.

## Results

Five local planning samples were collected per scenario:

| Scenario | Direct result | Relationship/application fallback | Required scopes | Median |
|---|---|---|---:|---:|
| `fletch-core` | all 3 FLETCH packages | BISECT, ROUTE, ICELINES | 4/5 | 502.550 ms |
| `metis-core` | METIS-CORE | BISECT, ROUTE | 3/5 | 495.767 ms |
| both producers | both direct producer scopes | BISECT, ROUTE, ICELINES | 5/5 | 494.656 ms |
| `bisect-core` | 14-package BISECT reverse cone | none | 1/5 | 494.563 ms |
| `route-data` | all 7 ROUTE packages | none | 1/5 | 498.273 ms |
| `icelines-core` | all 6 ICELINES packages | none | 1/5 | 511.091 ms |
| application policy | none | all 5 application fallback | 5/5 | 523.635 ms |

## Findings

### FERRIS-SSD-01: The real diamond remained explicit

Observed: FLETCH widened BISECT, ROUTE, and ICELINES; METIS-CORE widened
BISECT and ROUTE; changing both producers required all five scopes.

Implication: Ferris can represent a cross-repository shared-substrate graph
without merging Cargo workspaces or lockfiles.

Confidence: high for these exact revisions and declared relationships.

### FERRIS-SSD-02: Consumer-local work remained isolated

Observed: changes rooted in BISECT, ROUTE, or ICELINES required one workspace
scope. Unrelated producers and sibling consumers remained `not_selected`.

Implication: application membership and reverse producer relationships do not
force unrelated owner-local work into a portfolio-wide validation event.

Confidence: high for the selected package inputs.

### FERRIS-SSD-03: The consumers resolve historical producer revisions

Observed: all three consumers lock FLETCH at
`7c7aacd4fbc8753299e2fc3181368d2cc6d90337`, while the producer snapshot is
the later descendant `361fb7eda730bf660de92e47d502b91dcfaf473f`. BISECT and
ROUTE lock METIS-CORE at `78ae34090e043e79a206f2daffaa3889389b4790`,
while the producer snapshot is the later descendant
`afa595148430707ac87c622611c8d45b28b79c1e`.

Implication: impact propagation and resolved-revision compatibility are
separate graph axes. Selecting a consumer for review does not prove which
producer revision it exercises.

Confidence: high from exact manifests, lockfiles, and ancestry checks.

### FERRIS-SSD-04: Declaration mode and resolved identity differ

Observed: BISECT declares both producers by `branch = "master"`, and ICELINES
declares FLETCH by branch. ROUTE pins both exact revisions. At these snapshots,
all consumers nevertheless resolve the same historical producer commits.

Implication: a useful future read-only skew report must preserve declared
source mode, resolved lock identity, and observed producer identity
separately. It must not infer compatibility or rewrite manifests.

Confidence: high for these exact manifests and locks.

### FERRIS-SSD-05: Planning cost still loads the full application

Observed: medians remained approximately 495-524 ms whether one or five
workspace scopes were required.

Implication: scope reduction currently improves the downstream validation
matrix, not Ferris metadata-loading cost. Caching or bounded parallel metadata
loading remains a separate optimization target.

Confidence: medium from five local samples per scenario.

### FERRIS-SSD-06: Ferris now follows the Rust dual-license convention

Observed: both Ferris packages report `Apache-2.0 OR MIT`; the repository
contains the standard Rust Project Apache 2.0 text and the existing MIT text.

Implication: users may choose either license under a convention familiar to
the Rust ecosystem.

Confidence: high for repository and Cargo metadata. This does not imply Rust
Project or Rust Foundation affiliation.

## Bounded follow-up

A future skew capability should be a separate read-only report that:

- reads declared git source mode and exact lockfile resolution;
- compares them with an explicitly supplied producer identity;
- reports equal, behind, ahead, divergent, unavailable, and unknown states;
- identifies the owner and source for every assertion; and
- never claims semantic compatibility, edits manifests, or executes owner
  validation.

The repository-level fix remains owner-controlled exact revision pins,
machine-readable compatibility contracts, explicit validation commands, and
tested migration and rollback.

## Environment and claim boundary

- Windows 11 Enterprise Insider Preview `10.0.26310`;
- Intel Core i7-12800HX, 24 logical processors;
- `rustc 1.95.0 (59807616e 2026-04-14)`;
- `cargo 1.95.0 (f2d3ce0bd 2026-03-21)`; and
- five samples per scenario.

No validation command, build, test, format, Clippy, consumer checker, network
operation, or child mutation was performed by Ferris. This is local evidence,
not a production, support, relationship-completeness, semantic compatibility,
or build-time savings claim.
