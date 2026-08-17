# Pulse 75 stage-bootstrap/worker-identity successor root cause report

## Defects fixed

Frozen Pulse 72 closed the first stage-time lexical substitution gap, but two review findings remained in the native WSL path:

1. after exclusive bundle-root creation, stage bootstrap cleanup still depended on host follow-up, so failures between create and return could leave uncertain ownership or tempt deletion of a replacement tree; and
2. host revalidation still happened before launching a separate worker path, leaving a race where the parent, root, worker file, or worker dependency loader could be swapped between host validation and actual interpreter open.

## Successor approach

Pulse 75 keeps frozen Pulse 72 intact and changes only the custody boundary around staging and worker bootstrap:

- the bounded WSL stage bootstrap now owns every failure after exclusive root creation, retains exact parent/root identity, attempts cleanup internally, verifies absence after deletion, and reports only `removed` or `indeterminate`;
- the host interprets cleanup uncertainty with precedence and surfaces `P57-INDETERMINATE-CLEANUP` even if another protocol or execution failure also occurred;
- the launch handoff now passes expected parent/root identity plus expected worker and dependency-loader hashes into one exact WSL `-c` bootstrap process;
- that bootstrap re-validates parent/root identity in-process, opens the worker descriptor no-follow, confirms the same inode by `fstat`, hashes the exact bytes, and executes only from the verified descriptor path under `/proc/self/fd`;
- the worker receives the expected bundle identity, verifies the staged bundle again before dependency load, and loads exact dependency bytes from the verified bundle rather than trusting a mutable path; and
- cleanup still removes only the original owned tree, never a replacement. If the original orphan cannot be safely found and removed, the outcome remains fatal indeterminate cleanup.

## Qualification boundaries

Qualification is fake-only. It proves exact Pulse 72 binding, local-loader isolation, stage-bootstrap cleanup ownership, cleanup-precedence, root/parent substitution rejection, worker-bootstrap root swap rejection, worker path swap rejection, retained-bundle lifetime, and zero post-close residue across 20 cycles and 2,760 harmless launches. It executes no authority and no real FERRIS diagnostic.
