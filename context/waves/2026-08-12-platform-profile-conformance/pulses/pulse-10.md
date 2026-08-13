# Pulse 10: Native Dependency Profile Family

Status: Complete
Implementation authority: Bounded to this document

## Goal and authority

Complete one system-native family that calls only the host operating system's
process identity API through a minimal conditional FFI boundary.

Revision `r1` returns the current process identity. Revision `r2` adds one
related owner identity: parent process on Unix and current thread on Windows.

This pulse authorizes:

- two exact zero-crate-dependency consumers and lockfiles;
- minimal documented `unsafe extern` declarations and safe wrappers;
- Windows `kernel32` and Unix libc link/runtime observations;
- positive owner tests and explicit nonzero invariants;
- exact native provider, linker-name, host, runtime, and limitation records;
- locked/offline owner commands in isolated target directories;
- source-tree immutability, canonical profiles and digests;
- Windows and Unix development validation; and
- one nine-role review.

It does not authorize arbitrary FFI, bundled source, header generation,
dynamic loading, package-manager discovery, ABI portability beyond the exact
declarations, credentials, deployment, support, another family, production
generation, or held-out access.

## Acceptance

- conditional declarations exactly match the documented OS functions;
- unsafe code is limited to the two FFI modules and safe wrappers;
- both revisions return nonzero current process identities;
- `r2` retains its exact additional owner identity;
- Windows and Unix owner workflows and repository gates pass;
- system-native ownership and update responsibility remain explicit;
- source and profile digests are stable and distinct; and
- all nine roles accept measured evidence.

## Evidence

- [Authorization review](../../../../docs/plans/reviews/PULSE-10-NATIVE-DEPENDENCY-ROLE-REVIEW.md)
- [Windows and Unix validation](../../../../docs/plans/validation/PULSE-10-NATIVE-DEPENDENCY-FAMILY.md)

Implementation cutoff:
`41b7086cb43bc6b9a37b7ba5920cfdec39950f4a`.

Both revisions passed exact native link, execution, immutability, digest, and
repository gates on Windows and Ubuntu WSL2. The system provider remains
ambient and outside Cargo ownership.
