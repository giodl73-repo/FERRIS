# Pulse 51 public diagnostic-executor release

Status: public infrastructure and synthetic qualification only.

This release fixes public prelaunch executor controls. It does **not** execute
or authorize Pulse 50/Pulse 51 diagnostic work, create a private seed or
descriptor, invoke a private candidate, or create a terminal result or witness
root. Pulse 50 remains `authorized-unexecuted`, blocked with its launch
unconsumed pending governance closeout or a separately authorized successor.

## Production boundary

`diagnostic_executor.run_diagnostic_executor` is the sole exported runtime
operation. It accepts a materialized descriptor root, one declared private
runtime root, an absent P27 cycle root, and two `P44CustodyBinding` values.
Each custody binding contains only an independently captured Pulse 44 public
summary plus its private work/final roots. There is no grant string, trust
mode, caller-supplied gate event, launcher, P27 runner, or P33 expectation
injection.

Authority verification is external governance. The runtime cannot
cryptographically mint or verify an authority from a forgeable string. Its
first historical P41/P39 catalog event records that external precondition
only; it does not authenticate it. All other gates are runtime checks.

The private runtime root and every descriptor, P27, custody, executable,
receipt, current directory, `--before`, and `--after` path MUST be absolute,
inside that root, non-UNC, non-device-alias, and free of symlink traversal.
The executor rejects outside, nested escape, alias, unmappable, or
command-injection-shaped path requests before a process starts.

## Fixed Windows and Ubuntu dispatch

Windows launches the exact retained P33 `.exe` natively with a direct argv:

```text
<windows-executable> profile-diff --before <windows-before> --after <windows-after> --format <json|human>
```

Ubuntu launches the retained ELF only through the exact WSL distribution:

```text
wsl.exe --distribution Ubuntu-24.04 --cd <wsl-cwd> --exec <wsl-executable> profile-diff --before <wsl-before> --after <wsl-after> --format <json|human>
```

Executable, cwd, and both input paths are independently verified Windows
absolute paths, then translated only as `/mnt/<lowercase-drive>/...`. UNC,
extended/device aliases, relative paths, and untranslatable roots fail closed.
Neither launch uses a shell.

Before either launch, the executor verifies the fixed P33 logical filename,
two-file custody tree, raw executable size/hash, exact receipt envelope and
payload hash, cutoff, clean checkout, locked release command, toolchain, and
reproducibility controls. Windows is fixed to Cargo/Rust `1.95.0` plus
`/Brepro`; Ubuntu is fixed to Cargo/Rust `1.97.1`.

## Predecessor custody and P27

For each canonical platform, the executor maps the internal
`ubuntu-24.04-wsl2-x86_64` label to `ubuntu-24.04-x86_64` **before** any
custody event. It validates the retained final tree, invokes sealed
`bridge_pulse_44` exactly once with the supplied P44 summary callback, and
requires the complete sealed P45 bridge receipt: all P44
manifest/receipt/seal hashes, platform, invocation count `1`, and retries
`0`. Caller-forged P45 gate events are not accepted.

It loads only the sealed P27 callable and invokes it once with a fresh absent
cycle root. Expected P27 `ValueError`, `RuntimeError`, `OSError`, and
`subprocess` failures are converted to bounded P51 terminal failures. A
successful P27 durable cycle root is retained as private custody evidence; a
partial, malformed, or failed cycle is recursively removed and absence is
proved before its terminal result is returned. There is no retry.

The executor also validates the frozen P31 artifacts and all 33 mutations,
then binds all ten P35 release-tree files, the P35 machine schema, and
Git-clean LF identities. Filesystem, Git, sealed-dependency, descriptor
aggregate/scandir, P31, P35, WSL, and expected process failures are bounded
without catching programmer faults.

## Complete process-output contract

JSON output must be one LF-terminated, duplicate-free
`ferris.command-result/v2` profile-diff envelope with exact command version,
semantic command, result class, exit, stream route, diagnostic
cardinality/class/code/shape, and nullable-record posture. Identity syntax is
only a preliminary check: the executor independently reloads the accepted
before/after profiles and applies the frozen public Rust algorithm to derive
matching `profile_id` and `consumer`, result class, content references,
section partition, complete ordered change set/count, value digests, and
`diff_id`. It then independently recomputes selection, invocation, and
result identities from the exact NUL-framed domains and compact
declaration-order payloads. A self-consistent-looking candidate that differs
from the input semantics is rejected.

Human success/difference output is parsed against the exact frozen grammar and
the same independently derived semantic record; a nonempty line is not
sufficient. The private record retains raw and normalized output hashes.
Windows/Ubuntu agreement compares only the held-out process-exit semantic
projection (class, exit, diagnostics, and path-free record semantics), never
raw platform-sensitive identity or path bytes.

## Terminal integration

`TerminalPulse47Once` is the separately created, one-use terminal object.
`invoke_terminal_pulse47_once` requires it and validates that both P43 and
P47 roots are absolute, fresh absent, distinct and non-overlapping, below one
declared safe terminal parent, and have no symlinked ancestor in that parent.
It calls sealed P47 only once. Bounded P47 precondition and witness failures
retain P47's path-free public summary shape. Qualification never invokes P47
or creates a result/witness root.

## Synthetic qualification

The underscore-prefixed qualification helper is private test infrastructure,
not an exported/runtime-authority API. It uses generated harmless descriptors,
fully conforming fake command envelopes/human records, synthetic P44
summaries, and a final-boundary process runner. It still exercises the exact
production dispatch constructor and asserts native Windows argv plus the
complete `Ubuntu-24.04` WSL argv, cwd, executable, before, and after
translations.

```console
python -B -m unittest discover -s tests -v
python -B qualify.py --cycles 20
```

The tests and 20 isolated cycles cover 69 fake launches per platform, final
no-launch disposition, full output schemas/grammar, semantic-only
cross-platform comparison, frozen identity vectors, every output identity,
profile/consumer mismatch, result-class lie, changed-section partition,
ordered change-set/count mutations, all P44 summary and P45 bridge identity
mutations, P33 mutations, P27 one-call partial-root cleanup, P31/P35/Git
failures, descriptor path/scandir failures, WSL absence, first mismatch stop,
terminal-root/one-use checks, and zero P43/P47 terminal publication
invocations.

`-B` disables bytecode output. Unit and qualification runtime roots are
created only below the repository `target/` directory and are removed on
success or failure; the scripts also remove stale P51 `__pycache__`, `.run`,
and `.qualification-work` residue before returning. The sealed release tree
therefore remains unchanged when the documented commands run in order.

Synthetic/test-root cleanup alone uses a shared bounded schedule of 0.02,
0.05, 0.10, and 0.20 seconds after `PermissionError` or Windows
`WinError 32`, then requires the root to be absent. It raises an explicit
cleanup failure after the bound or if deletion reports success but leaves a
root. Production P27 failed-cycle cleanup remains immediate and fail-closed.
The unit suite holds a synthetic receipt with a brief exclusive Windows handle
to prove bounded recovery and simulates a permanent sharing violation to prove
the explicit bounded failure. No process handle outlives its scoped runner.

`public-manifest.json`, `qualification-receipt.json`, and `release-seal.json`
seal this release. Removal deletes this infrastructure release and associated
governance records only; it does not change predecessor authority, runtime
custody, product state, or Pulse 50's unconsumed launch.
