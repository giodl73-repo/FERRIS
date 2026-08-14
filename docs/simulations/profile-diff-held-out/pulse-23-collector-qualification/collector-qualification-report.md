# Collector Durability Repair Qualification

**Outcome:** PASS  
**Scope:** Synthetic collector infrastructure only.

## Root cause

The failed writer closed the temporary file writer, reopened the file read-only, and then requested a file synchronization operation through that read-only descriptor. That access mode is invalid for the required Windows flush, so durability failed before atomic replacement. The old path also lacked an explicit durable containing-directory synchronization result.

## Repair

- Writable temporary descriptor remains open through userspace flush and file synchronization.
- Same-directory atomic replacement occurs only after file synchronization.
- Containing-directory synchronization is attempted and recorded explicitly.
- Interrupted-write residue is detected and rejected.

## Qualification

- Unit tests: 20 passed, 0 failed (10 Windows, 10 Ubuntu).
- Synthetic pairs: 20 passed, 0 failed.
- Command observations: 40 passed, 0 failed; 20 success exits and 20 nonzero exits.
- Routing: 30 stdout observations and 30 stderr observations.
- Retention: 20 Windows records, 20 Ubuntu records, and 20 pair seals.
- Fresh-process reload verifications: 2 passed, 0 failed.
- Cardinality exact: yes. Residue: 0. Read-only verification: idempotent.
- Closed source workspace: byte-for-byte aggregate unchanged.

## Digests

- Windows environment: `sha256:92597acdd22522ff1c3e06d508bd0797910f98282a76dfead1f640eb5e4e097a`
- Ubuntu environment: `sha256:344fbddf4b2efc1a7563057edf0903a605aa613eb46a12ac2db722e601838b72`
- Source: `sha256:5c54e47be59ffd2ad1ce7b83fdfa1302d05bd6539197f19f28e2e92d480cc558`
- Tests: `sha256:7aa04613f6578e0ef29edf04d781332542833b20a22bab053b1ac769f8d48f62`
- Qualification payload seal: `sha256:28210db303f128aedf591464736bc212c40c639ed1e28656eeb61afcbd3ac398`
- Qualification file seal: `sha256:465b78031a8fa79b90d5b69f3e15756e0fd1d677fb2316854c5ba6b35ce38252`
- Public JSON report: `sha256:1365dfd33f834b24f9e20f73afce313978b26d27e3c076ae90a8671eb841e723`

## Limitations

- Directory synchronization support is filesystem/API dependent and is never reported as successful when unsupported. Every counted write in this run reported a synchronized directory handle.
- Atomic replacement requires a same-directory temporary file.
- This was fixed-input infrastructure qualification, not a diagnostic search.
- No physical power-loss injection was performed.
