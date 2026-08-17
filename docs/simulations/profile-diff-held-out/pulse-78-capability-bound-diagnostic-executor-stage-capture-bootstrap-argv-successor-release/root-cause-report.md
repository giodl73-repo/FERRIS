# Pulse 78 stage-capture/bootstrap-argv successor root cause report

## Defects fixed

Frozen Pulse 72 closed the first stage-time lexical substitution gap, but two review findings remained in the native WSL path:

1. after exclusive bundle-root creation, stage bootstrap still had a mkdir→open ownership-capture gap, so a rename or replacement before the first bound descriptor could leave uncertain ownership or tempt deletion of a replacement tree; and
2. the bootstrap still leaked internal dependency-loader path/hash arguments into worker `argparse`, so the exact worker startup path was not production-shape validated and the dependency-loader binding was not consumed in-process.

## Successor approach

Pulse 78 keeps frozen Pulse 75 intact and changes only the custody boundary around staging and worker bootstrap:

- the bounded WSL stage bootstrap now captures ownership only through the verified parent descriptor, treats any pre-capture reopen mismatch or failure as `P78-INDETERMINATE-STAGE-CLEANUP`, and never stages or deletes a replacement tree;
- after ownership capture, staging stays fd-relative or identity-revalidated, cleanup remains exact-tree only, and post-capture uncertainty still escalates as `P57-INDETERMINATE-CLEANUP`;
- the host interprets cleanup uncertainty with precedence and surfaces `P57-INDETERMINATE-CLEANUP` even if another protocol or execution failure also occurred;
- the launch handoff now passes expected parent/root identity plus expected worker and dependency-loader path/hash bindings into one exact WSL `-c` bootstrap process;
- that bootstrap consumes the dependency-loader binding in-process, re-validates parent/root identity, opens both dependency loader and worker descriptors no-follow, confirms the same inode by `fstat`, hashes the exact bytes, and executes only from the verified worker descriptor path under `/proc/self/fd`;
- the worker receives the expected bundle identity, verifies the staged bundle again before dependency load, and loads exact dependency bytes from the verified bundle rather than trusting a mutable path; and
- only exact named worker flags reach `argparse`; and cleanup still removes only the original owned tree, never a replacement. If the original orphan cannot be safely found and removed, the outcome remains fatal indeterminate cleanup.

## Qualification boundaries

Qualification is fake-only. It proves exact Pulse 75 binding, local-loader isolation, stage create/open substitution handling, dependency-loader binding consumption, production-shape worker ready/close startup, cleanup-precedence, root/parent substitution rejection, worker-bootstrap root/path swap rejection, retained-bundle lifetime, and zero post-close residue across 20 cycles and 2,760 harmless launches. It executes no authority and no real FERRIS diagnostic.
