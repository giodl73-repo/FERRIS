# Native Dependency Profile Family

Status: Controlled family under Pulse 10

Both revisions use a minimal conditional FFI boundary to ambient operating
system APIs. Windows links `kernel32`; Unix resolves libc process APIs.
The Cargo graph does not own installation, patching, ABI, or servicing of
those system components.
