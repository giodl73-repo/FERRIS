# Pulse 86 WSL parent-owner binding root cause report

## Observed failure

The sole Pulse 84 invocation stopped before seed creation at Ubuntu capability
build custody with `P57-WSL-BUNDLE`. Exact Pulse 78 treats any staging stderr
as fatal. An immediate harmless command with the same ambient-user prefix
exited zero while WSL emitted:

`wsl: Failed to start the systemd user session for 'root'. See journalctl for more details.`

The same harmless command with Microsoft's documented `--user root` selector
exited zero with empty stderr. This proves an ambient default-user launch can
inject owner-launcher stderr on this host and that explicit user selection
removes that injection. It does not reconstruct Pulse 84's unretained internal
staging stderr or change the historical closeout.

## Successor approach

Pulse 86 keeps frozen Pulse 78 intact and changes only WSL execution-identity
selection:

- one explicit-root, read-only bootstrap validates the native parent, reads its
  no-follow directory metadata, resolves the owner UID through `pwd`, and emits
  only a closed username/UID record;
- every subsequent staging, revalidation, worker, and cleanup command includes
  `--user <resolved-owner>` before `--exec`;
- every operational bootstrap verifies both `os.geteuid()` and the opened
  parent's owner UID against the resolved owner UID before use;
- the username is derived from filesystem ownership rather than caller input,
  so operational custody remains with the parent owner;
- owner resolution rejects nonzero exit, any stderr, malformed protocol,
  invalid usernames, and missing account mappings as `P86-WSL-OWNER`; and
- all Pulse 78 worker stderr, cleanup, identity, and exact-tree failures remain
  fatal and unchanged. No stderr filtering or warning allowlist exists.

## Qualification boundaries

Qualification uses one harmless real WSL owner lookup plus fake-only capability
cycles. It proves exact Pulse 78/Pulse 75 binding, explicit owner argv,
unknown-stderr rejection, every inherited Pulse 78 control, 20 fake cycles,
2,760 harmless fake launches, and zero post-close residue. It executes no
authority, candidate, or real FERRIS diagnostic.
