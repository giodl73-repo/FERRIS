# Pulse 01: Synthetic Cargo Failure Evaluation

Status: Complete

## Control record

- User outcome: maintainers can see whether the narrow dependency classifier
  recognizes real Cargo output across materially different workspace shapes.
- Maximum effort: five clean synthetic repositories, one disposable archived
  copy and one removed path-dependency target per repository, and one report.
- Completion test: Cargo fails before build work, Ferris emits a classified
  dependency report for each case, no raw diagnostic or temporary path appears
  in Ferris output, all source repositories remain clean, and all temporary
  copies are removed.
- Abandonment condition: stop if a case requires source mutation, network,
  compilation, retained raw diagnostics, or classifier changes.
- Product Value Governor: `continue-within-budget`.

## Claim boundary

The evaluation covers one generated missing path-dependency failure per
topology on the current Windows host. It is not owner adoption, cross-platform
evidence, a representative failure distribution, or root-cause proof.

## Result

All five corrected controls passed. Cargo returned 101 for each missing
path-dependency case, and Ferris emitted
`FERRIS-CARGO-DEPENDENCY-BLOCKED` with `classification=dependency`,
`raw_output_retained=false`, and `executable=false`. Input sizes ranged from
574 through 628 bytes. Neither generated dependency names nor temporary paths
appeared in Ferris output.

The first control construction was invalid because omitting an existing path
dependency also omitted a workspace member; Cargo therefore reported a missing
workspace manifest. That result was discarded without changing Ferris. The one
corrective construction used Cargo to add a disposable external path
dependency before removing only its generated target. Cargo also temporarily
registered those generated crates in the enclosing Ferris workspace because
the evaluation root was under `target`; the entries were detected and fully
restored. All five synthetic repositories remained clean, and the exact
temporary evaluation root was removed. Disposition: `stop-complete`.
