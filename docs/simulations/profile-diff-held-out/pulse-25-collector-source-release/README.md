# Qualified Collector Source Bundle

This directory publishes the exact Pulse 23 qualified collector infrastructure
source and unit tests. The `bundle/` bytes are immutable qualification bytes.
No diagnostic data is included, and this release does not authorize diagnostic
searches or workloads.

## Requirements

- Python 3.10 or newer, using only the standard library.
- Windows PowerShell 5.1 or newer for the Windows synthetic commands.
- WSL with Ubuntu, Python 3.10 or newer, and `/bin/sh` for the Ubuntu half of
  the cross-platform qualification.

## Run the unit tests

Use a disposable copy so test bytecode and qualification records do not alter
the published bundle:

```powershell
Copy-Item -Recurse .\bundle .\qualification-run
Push-Location .\qualification-run

python -m unittest discover -s tests -v

$WindowsRun = (Resolve-Path .).Path
$Drive = $WindowsRun.Substring(0, 1).ToLowerInvariant()
$Tail = $WindowsRun.Substring(2).Replace('\', '/')
$WslRun = "/mnt/$Drive$Tail"
wsl.exe --cd $WslRun --exec python3 -m unittest discover -s tests -v
```

Each platform runs 10 tests.

## Run the harmless synthetic qualification

From the same disposable `qualification-run` directory:

```powershell
python qualification.py
python verify_qualification.py
```

The qualification uses only fixed public strings and trivial shell exit codes.
It accepts no candidate, corpus, seed, stream, or external data path. A complete
run creates 20 Windows/Ubuntu pairs, executes 40 harmless commands, performs
fresh-process verification, and requires zero atomic-write residue.

Clean up the disposable copy after reviewing the receipts:

```powershell
Pop-Location
Remove-Item -Recurse -Force .\qualification-run
```

See `public-manifest.json`, `qualification-report.json`,
`release-receipt.json`, and `release-seal.json` for exact digests and results.
