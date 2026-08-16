# Pulse 69 staged-bundle cleanup successor root cause report

## Defect fixed

Exact Pulse 57 stages one native `.p57-*` bundle under caller-supplied
`ubuntu_runtime_parent`, launches the Ubuntu worker from that bundle, then
closes the worker/capability without removing `staged.root` or proving
absence. Startup failure after staging had the same gap. Pulse 58 and Pulse 59
therefore inherited an unowned native residue path outside
`private_runtime_root`.

## Successor approach

Pulse 69 does not amend the frozen Pulse 57 release. Instead it binds the
exact Pulse 57 release bytes and reuses its complete descriptor, dispatch,
normalization, and result-accounting semantics while replacing only the native
WSL session ownership boundary.

The successor:

- stages the exact Pulse 57 worker plus exact Pulse 56 release tree;
- records the exact staged bundle root name plus parent/root device+inode;
- records the exact expected bounded tree layout derived from the byte-bound
  staged files;
- ends the worker/capability before cleanup on every terminal path that
  successfully ends the process;
- removes only the retained root via a native no-follow bounded tree walk with
  exact expected child names at every directory level;
- rejects root substitution, symlink traversal, unexpected file kinds, and
  unexpected entries before claiming cleanup success;
- syncs the parent directory when supported and records unsupported sync as an
  allowed bounded posture; and
- treats any cleanup uncertainty as fatal `P57-INDETERMINATE-CLEANUP`
  precedence at the executor result boundary.

## Qualification boundaries

Qualification is fake-only. It patches staging, cleanup, and worker-process
creation with harmless local doubles, proves the owned bundle persists during
worker lifetime, proves it is removed exactly once after successful close, and
proves cleanup uncertainty outranks protocol failure. It executes no authority
and no real FERRIS diagnostic.
