# Pulse 72 stage-identity capability executor successor

Pulse 72 is the sealed successor to Pulse 69. It preserves exact Pulse 69/P57/P56/P51 diagnostic semantics while fixing the independent review finding that the native WSL staging path could be replaced between stage time and worker launch.

## Production surface

The only production callable is `run_capability_bound_diagnostic_executor(repo_root, descriptor_root, private_runtime_root, p27_cycle_root, ubuntu_runtime_parent)`.

Pulse 72 does not amend frozen Pulse 69. Instead it:

1. byte-binds the exact Pulse 69 release through a local sibling `sealed_dependencies.py` path/hash loader rather than ambient import resolution;
2. stages the exact Pulse 57/P56 bundle with a single bounded bootstrap that captures canonical root and parent identity after exclusive root creation and again after file finalization;
3. carries that exact root/parent device, inode, and type identity inside the staged bundle object;
4. revalidates the lexical root and parent identity immediately before worker launch; and
5. removes only the originally staged root path/inode, making substitution or replacement fatal `P57-INDETERMINATE-CLEANUP` without deleting replacements or siblings.

## Fake-only qualification

Qualification is harmless and fake-only:

- 11 receipt-listed behavioral and loader controls;
- 20 fake-only cycles preserving the exact `70/69/1` and `140/138/2` Pulse 57 topology;
- 2,760 harmless launches total;
- one prelaunch identity revalidation and one owned staged-bundle cleanup per cycle; and
- zero authority execution and zero real FERRIS execution.

## Evidence

- [Qualification receipt](qualification-receipt.json)
- [Public manifest](public-manifest.json)
- [Release seal](release-seal.json)
- [Root-cause report](root-cause-report.md)
- [Qualification schema](schemas/ferris.pulse-72-capability-bound-diagnostic-executor-stage-identity-successor.v1.schema.json)
- [Python tests](tests/test_capability_bound_diagnostic_executor_successor.py)
