# Pulse 33 public build-freeze adapter

This adapter verifies an exact, clean Git cutoff with `core.autocrlf=false`,
builds `ferris-cli` with locked release dependencies, discovers the executable
from Cargo's `compiler-artifact` JSON, and freezes a SHA-256-bound receipt.

It does not inspect FERRIS source, run the executable, launch diagnostics, or
modify product files. It uses only Git, Cargo, rustc, and Python's standard
library.

## Windows

```powershell
python build_freeze.py `
  --repo <exact-cutoff-checkout> `
  --cutoff 29517d732db13cc2ffa304684b344f3538ab587d `
  --platform windows-x86_64 `
  --output frozen
```

## Ubuntu 24.04

```bash
python3 build_freeze.py \
  --repo <exact-cutoff-checkout> \
  --cutoff 29517d732db13cc2ffa304684b344f3538ab587d \
  --platform ubuntu-24.04-x86_64 \
  --output frozen
```

If `cargo` is absent from a non-login shell `PATH`, the adapter checks the
ordinary rustup location at `$HOME/.cargo/bin/cargo`.

The public-safe default does not retain the executable because ordinary Rust
artifacts can contain local toolchain paths. Add `--retain-executable` only for
a separately controlled non-public freeze location.

Windows builds add the standard MSVC linker `/Brepro` control because an
ordinary clean rebuild changes the PE timestamp. Qualification confirmed two
clean `/Brepro` builds produced the same SHA-256.

## Qualification

```powershell
python -m unittest discover -s tests -v
python synthetic_checks.py --work-dir .synthetic-work --output synthetic-checks.json
python publish.py
python verify_release.py
```

The synthetic qualification performs exactly 20 build-message,
output-discovery, naming, and hashing checks. It never executes a FERRIS
binary.
