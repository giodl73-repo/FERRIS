# Pulse 32 public build blocker root cause

## Conclusion

The blocker is orchestration-only. The exact cutoff
`29517d732db13cc2ffa304684b344f3538ab587d` compiles successfully on Windows
and Ubuntu 24.04 WSL2 with locked dependencies. No FERRIS product change is
required.

The generic Ubuntu failure reproduces in a non-login WSL shell:

```text
bash: line 16: cargo: command not found
```

That shell has an executable rustup-managed Cargo installation at the ordinary
`$HOME/.cargo/bin` location, but the location is absent from `PATH`. The same
clean checkout and command succeed in a login shell and also succeed in the
same non-login shell when Cargo is addressed explicitly. The public adapter
therefore resolves Cargo from `PATH` and then from the ordinary rustup
location, rather than depending on shell startup files.

## Exact-cutoff reproduction

- Fresh detached Git checkout on Ubuntu 24.04 WSL2.
- `core.autocrlf=false`.
- Cargo command: `cargo build --locked --release --package ferris-cli --bin ferris`.
- Non-login implicit Cargo: exit `127`, `cargo: command not found`.
- Login-shell Cargo: exit `0`.
- Non-login explicit Cargo: exit `0`.
- Ubuntu executable: 1,945,448 bytes,
  `sha256:6cf654f1a7c277317753a9cb0f0a7bc1e183de40715b25e35828bcdb9d18cfe4`.
- Windows executable: 1,436,672 bytes,
  `sha256:0736392a9fab3fa9404554f86e82e1dfcdf9e68e44aa3df8a4cddd1d58a34fd8`
  with the adapter's MSVC `/Brepro` control.

The executables were only built, located, and hashed. Neither was executed.
Executable copies are not retained in this public bundle because ordinary Rust
artifacts may contain local toolchain paths; the path-free receipts retain
their sizes and digests.

## Alternative-cause disposition

| Candidate | Disposition | Public evidence |
| --- | --- | --- |
| Checkout path translation | Not causal | Native WSL checkout builds; Cargo metadata also succeeds through a translated `/mnt/c` checkout path. |
| Cargo target directory/output discovery | Not the initiating failure; guessing remains unsafe | Cargo did not start in the failing shell. The adapter uses Cargo `compiler-artifact` JSON instead of a guessed target path. |
| Executable naming | Not causal | Cargo reports `ferris` on Ubuntu and `ferris.exe` on Windows; both are found deterministically. |
| Toolchain/dependency availability | Installed and usable; shell discovery failed | Ubuntu Cargo/rustc 1.97.1 and Windows Cargo/rustc 1.95.0 complete locked release builds. |
| Line endings | Excluded | Both clean checkouts report identical counts: 928 `i/lf w/lf`, one binary `-text`, and three `none`; `core.autocrlf=false`. |
| Actual compilation | Excluded | Both exact-cutoff locked release builds complete successfully. |

An ordinary Windows clean rebuild changed only build-freeze identity because
the PE timestamp changed. Two clean builds with the standard MSVC `/Brepro`
linker control produced the identical digest shown above. This is another
external build-freeze concern, not a FERRIS compilation or product defect.

## Prior public Pulse 30 fact

Pulse 30 publicly recorded that both Windows and Ubuntu executables and both
environments were frozen for its earlier immutable cutoff, after 36/36
attribute checks, 36/36 LF checks, and 76/76 normalized bindings. That fact is
consistent with the current direct Ubuntu success and inconsistent with a
general FERRIS-on-Ubuntu compilation defect.

## Remediation

Use the external build-freeze adapter in this directory. It verifies the exact
commit and clean checkout, requires `core.autocrlf=false`, runs the locked
release build, discovers the platform executable from Cargo JSON, hashes and
freezes it, and emits a path-free receipt. It never runs a diagnostic or edits
FERRIS.
