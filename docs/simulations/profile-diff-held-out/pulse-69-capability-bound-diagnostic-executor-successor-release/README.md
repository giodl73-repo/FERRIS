# Pulse 69 cleanup-owning capability executor successor

Pulse 69 is a sealed infrastructure-only successor to Pulse 57. It preserves
exact Pulse 57/P56/P51 diagnostic semantics and public gate accounting, but it
changes ownership of the native staged `.p57-*` Ubuntu bundle so the session
retains exact bundle identity through worker close and removes only its owned
bundle after the worker/capability has ended.

## Production surface

The only production callable is
`run_capability_bound_diagnostic_executor(repo_root, descriptor_root, private_runtime_root, p27_cycle_root, ubuntu_runtime_parent)`.
It accepts no diagnostic authority, result publication root, callback,
process runner, environment override, or synthetic control injection.

Pulse 69 byte-binds the complete exact Pulse 57 release and therefore retains
exact Pulse 51 and Pulse 56 predecessor binding. It reuses exact Pulse 57
descriptor freezing, dispatch construction, normalization, topology
accounting, and terminal catalog/event semantics. The only behavior change is
native staged-bundle custody:

1. stage exact Pulse 57 worker/P56 bytes into one fresh native `.p57-*`
   bundle under caller-supplied `ubuntu_runtime_parent`;
2. capture the exact parent/root device+inode identity and the exact expected
   bounded tree shape for that owned bundle;
3. keep that owned bundle identity on the session for the full worker
   lifetime;
4. on every terminal path where the worker is ended, close the worker
   protocol/process, then remove only the retained owned bundle via a native
   no-follow bounded tree walk; and
5. verify bundle absence and parent sync posture, making cleanup uncertainty
   fatal with precedence.

Pulse 69 never deletes the caller parent itself or arbitrary siblings. Any
unexpected root identity change, symlink/reparse-style substitution, unexpected
tree entry, non-directory root, or unverifiable absence fails closed as
cleanup uncertainty.

## Fake-only qualification

Qualification is harmless and fake-only:

- 8 receipt-listed behavioral/source controls;
- 20 fake-only cycles preserving the exact `70/69/1` and `140/138/2`
  Pulse 57 topology;
- 2,760 harmless launches total;
- one owned staged-bundle cleanup per cycle with zero bundle residue after
  close; and
- zero authority execution and zero real FERRIS execution.

## Evidence

- [Qualification receipt](qualification-receipt.json)
- [Public manifest](public-manifest.json)
- [Release seal](release-seal.json)
- [Root-cause report](root-cause-report.md)
- [Qualification schema](schemas/ferris.pulse-69-capability-bound-diagnostic-executor-successor.v1.schema.json)
- [Python tests](tests/test_capability_bound_diagnostic_executor_successor.py)
