# Pure Data Profile Family

Status: Controlled family under Pulse 04

The family has two independent local Cargo consumers:

- `r1` rejects internal ASCII whitespace; and
- `r2` accepts internal ASCII whitespace and collapses each run to `-`.

Both revisions reject empty and non-ASCII input. They have no external
dependencies, build scripts, procedural macros, unsafe code, native code,
providers, runtime services, network, deployment, or support commitment.

`family.json` binds the exact package revisions, source-tree digests, and
materialized profile digests. The Rust integration test materializes complete
v1 profile values in memory from the frozen schema exemplar; normal test runs
do not write generated profile files.
