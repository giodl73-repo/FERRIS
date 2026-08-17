# Pulse 75 stage-bootstrap/worker-identity capability executor successor

Pulse 75 is the sealed successor to frozen Pulse 72. It preserves exact Pulse 72 / Pulse 57 / Pulse 56 / Pulse 51 diagnostic semantics while fixing the remaining stage cleanup and worker-launch substitution gaps in the native WSL handoff.

## Production surface

The only production callable is `run_capability_bound_diagnostic_executor(repo_root, descriptor_root, private_runtime_root, p27_cycle_root, ubuntu_runtime_parent)`.

Pulse 75 does not amend frozen Pulse 72. Instead it:

1. byte-binds the exact Pulse 72 release through a local sibling `sealed_dependencies.py` loader rather than ambient import resolution;
2. makes the WSL stage bootstrap own every failure after exclusive root creation, using retained parent/root identity and deleting only the original owned tree;
3. returns a cleanup posture of either removed or indeterminate, and makes host classification treat cleanup uncertainty as precedence-fatal `P57-INDETERMINATE-CLEANUP`;
4. passes expected parent/root device+inode identity and expected worker/dependency hashes into the exact WSL `-c` bootstrap process;
5. revalidates parent/root identity inside that same new process, opens the worker no-follow, hashes the exact bytes, and executes from the verified descriptor via `/proc/self/fd/...`; and
6. makes the worker re-check bundle identity before loading exact dependency bytes, preserving the isolated protocol and refusing path-swapped code.

## Fake-only qualification

Qualification is harmless and fake-only:

- 15 receipt-listed control tests, including deterministic post-create cleanup, cleanup-precedence, root-swap, and worker-path-swap checks;
- 20 fake-only cycles;
- 2,760 harmless launches total;
- one staged-identity revalidation and one owned-bundle cleanup per cycle; and
- zero authority execution and zero real FERRIS execution.

## Evidence

- [Qualification receipt](qualification-receipt.json)
- [Public manifest](public-manifest.json)
- [Release seal](release-seal.json)
- [Root-cause report](root-cause-report.md)
- [Qualification schema](schemas/ferris.pulse-75-capability-bound-diagnostic-executor-stage-bootstrap-worker-identity-successor.v1.schema.json)
- [Python tests](tests/test_capability_bound_diagnostic_executor_successor.py)
