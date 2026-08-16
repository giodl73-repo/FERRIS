# Pulse 72 stage-identity successor root cause report

## Defect fixed

Pulse 69 retained ownership of the staged native `.p57-*` bundle and cleaned it with bounded no-follow traversal, but the single original WSL staging bootstrap still returned only a lexical root path plus post-write device/inode data. An attacker that could rename or replace the staged root or its parent between staging and worker launch could race the handoff: the worker might start from a replacement lexical path, and cleanup would then face ambiguity about whether the path still named the originally staged bundle.

## Successor approach

Pulse 72 keeps frozen Pulse 69 intact and binds its exact bytes through a local sibling sealed loader. It changes only the staging custody boundary:

- the bounded WSL staging bootstrap creates the exclusive root, captures canonical root and parent identity immediately, writes the exact expected file tree, then captures final root identity again before returning bounded JSON;
- the staged bundle object carries `root_device`, `root_inode`, `root_type`, `parent_device`, `parent_inode`, and `parent_type` together with the expected layout;
- immediately before worker launch, Pulse 72 revalidates that the lexical root and lexical parent still resolve to the exact stage-time identities;
- if the root or parent was renamed, replaced, or otherwise substituted, launch fails closed before capability execution; and
- cleanup removes only the original stage-time root path/inode and treats any substitution or replacement as fatal `P57-INDETERMINATE-CLEANUP`, preserving custody knowledge without deleting replacements or arbitrary siblings.

## Qualification boundaries

Qualification is fake-only. It patches staging, prelaunch identity revalidation, cleanup, and worker-process creation with harmless local doubles. It proves the local loader ignores ambient `sealed_dependencies`, proves fresh module loading, proves stage-time identity capture is wired into prelaunch revalidation, proves root and parent substitution become indeterminate cleanup, and proves bundle cleanup still happens exactly once after close with zero residue. It executes no authority and no real FERRIS diagnostic.
