# Exact-Two-Pair Public Preflight Adapter

This public infrastructure package uses the immutable qualified collector
modules in `collector/` and adds a separate exact-two-pair orchestration layer.
It does not modify or execute Ferris and contains no diagnostic workload.

The adapter creates exactly two fixed harmless Windows/Ubuntu synthetic pairs,
durably writes four process rows and two pair seals, then verifies the complete
six-file store from fresh Windows and Ubuntu Python processes. Verification is
strictly read-only and rejects extra, missing, duplicate, unjoined, tampered,
partially sealed, or residue-bearing stores.

Run:

```powershell
python -m unittest discover -s tests -v
wsl.exe --exec python3 -m unittest discover -s tests -v
python reproduce_cardinality_failure.py
python qualify.py --cycles 50
```

The reproducer demonstrates only the generic public infrastructure error: a
whole-store exact-cardinality verifier was invoked with a pair-local count of
one after a second row had been appended. The collector correctly rejected the
extra row. The adapter instead stages the complete two-pair store and verifies
it with an exact expected count of two.
