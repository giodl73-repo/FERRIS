# Pulse 78 stage-capture/bootstrap-argv capability executor successor

Pulse 78 is the sealed successor to frozen Pulse 75. It preserves exact Pulse 75 / Pulse 57 / Pulse 56 / Pulse 51 diagnostic semantics while fixing the remaining stage create/open ownership-capture and worker bootstrap argv gaps in the native WSL handoff.

## Production surface

The only production callable is `run_capability_bound_diagnostic_executor(repo_root, descriptor_root, private_runtime_root, p27_cycle_root, ubuntu_runtime_parent)`.

Pulse 78 does not amend frozen Pulse 75. Instead it:

1. byte-binds the exact Pulse 75 release through a local sibling `sealed_dependencies.py` loader rather than ambient import resolution;
2. captures root ownership only through the verified parent descriptor, treats any pre-capture reopen mismatch or failure as fatal `P78-INDETERMINATE-STAGE-CLEANUP`, and never stages a replacement tree;
3. keeps every post-capture path use fd-relative or identity-revalidated, deleting only the original owned tree and surfacing `P57-INDETERMINATE-CLEANUP` if post-capture cleanup certainty is lost;
4. passes expected parent/root identity plus worker and dependency-loader path/hash bindings into one exact WSL `-c` bootstrap process;
5. consumes those dependency-loader bindings inside that bootstrap, revalidates parent/root identity in-process, opens both dependency loader and worker descriptors no-follow, hashes the exact bytes, and executes only from the verified worker descriptor via `/proc/self/fd/...`; and
6. forwards only the exact named worker flags into `argparse`, after which the worker re-checks bundle identity before loading exact dependency bytes.

## Fake-only qualification

Qualification is harmless and fake-only:

- 18 receipt-listed control tests, including deterministic create/open substitution, dependency-loader binding, exact bootstrap ready/close, post-create cleanup, cleanup-precedence, root-swap, and worker-path-swap checks;
- 20 fake-only cycles;
- 2,760 harmless launches total;
- one staged-identity revalidation and one owned-bundle cleanup per cycle; and
- zero authority execution and zero real FERRIS execution.

## Evidence

- [Qualification receipt](qualification-receipt.json)
- [Public manifest](public-manifest.json)
- [Release seal](release-seal.json)
- [Root-cause report](root-cause-report.md)
- [Qualification schema](schemas/ferris.pulse-78-capability-bound-diagnostic-executor-stage-capture-bootstrap-argv-successor.v1.schema.json)
- [Python tests](tests/test_capability_bound_diagnostic_executor_successor.py)
