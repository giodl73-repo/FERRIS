# Pulse 86 WSL parent-owner binding capability executor successor

Pulse 86 is the sealed successor to frozen Pulse 78. It preserves exact Pulse
78 / Pulse 75 / Pulse 57 / Pulse 56 / Pulse 51 capability semantics while
replacing ambient WSL default-user selection with an explicit execution
identity derived from the native runtime parent's owner.

## Production surface

The only production callable is `run_capability_bound_diagnostic_executor(repo_root, descriptor_root, private_runtime_root, p27_cycle_root, ubuntu_runtime_parent)`.

Pulse 86 does not amend frozen Pulse 78. Instead it:

1. byte-binds exact Pulse 78 and its exact Pulse 75 stack through a local
   sibling `sealed_dependencies.py` loader;
2. uses one explicit-root, read-only bootstrap to `stat` the native runtime
   parent and resolve its UID through the distribution account database;
3. validates the returned username and inserts `--user <owner>` before
   `--exec` for staging, revalidation, worker, and cleanup spawns;
4. rejects nonzero exit, malformed output, unknown stderr, and missing owner
   accounts as `P86-WSL-OWNER`; and
5. preserves Pulse 78's fd-relative ownership capture, exact-tree cleanup,
   worker/dependency byte binding, protocol stderr fatality, and public gate
   semantics unchanged.

## Fake-only qualification

Qualification is harmless and fake-only except for one read-only owner lookup:

- 25 receipt-listed controls, including real harmless owner resolution,
  explicit owner argv binding, unknown owner-probe stderr rejection, and all
  inherited Pulse 78 controls plus parent-owner and effective-UID mismatch
  rejection, malformed owner-protocol classification, and lock-free fork-child
  sealed-loader reset;
- 20 fake-only cycles;
- 2,760 harmless launches total;
- one staged-identity revalidation and one owned-bundle cleanup per cycle; and
- zero authority execution and zero real FERRIS execution.

## Evidence

- [Qualification receipt](qualification-receipt.json)
- [Public manifest](public-manifest.json)
- [Release seal](release-seal.json)
- [Root-cause report](root-cause-report.md)
- [Qualification schema](schemas/ferris.pulse-86-wsl-parent-owner-binding-capability-executor-successor.v1.schema.json)
- [Python tests](tests/test_capability_bound_diagnostic_executor_successor.py)
